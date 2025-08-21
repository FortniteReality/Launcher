
use sha1::{Digest, Sha1};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::{mpsc, Semaphore, Mutex};

use crate::manifest::chunk_data::ChunkInfo;
use crate::manifest::downloader::download_utils::{download_chunk_from_r2_streaming, guid_to_u128};
use crate::manifest::downloader::errors::DownloadError;
use crate::manifest::downloader::progress_update::ProgressUpdate;
use crate::manifest::ParsedManifest;
use crate::DownloadControl;

pub async fn download_game(
    manifest: ParsedManifest,
    bucket: String,
    install_dir: PathBuf,
    tx: mpsc::Sender<ProgressUpdate>,
    control: Arc<DownloadControl>,
    old_manifest: Option<ParsedManifest>,
) -> Result<(), DownloadError> {
    // Build chunk map once
    let chunk_map: Arc<HashMap<u128, ChunkInfo>> = Arc::new(
        manifest
            .chunk_data_list
            .elements
            .iter()
            .map(|c| (guid_to_u128(&c.guid), c.clone()))
            .collect(),
    );

    // Build reuse map from old manifest (filename + chunk guid/offset/size) -> (source filename, absolute byte offset)
    let mut reuse_map: HashMap<(String, u128, u32, u32), (String, u64)> = HashMap::new();
    if let Some(old) = &old_manifest {
        for old_file in &old.file_manifest_list.elements {
            let mut old_offset = 0u64;
            for old_cp in &old_file.chunk_parts {
                let key = (
                    old_file.filename.clone(),
                    guid_to_u128(&old_cp.guid),
                    old_cp.offset,
                    old_cp.size,
                );
                reuse_map.insert(
                    key,
                    (
                        old_file.filename.clone(),
                        old_offset + (old_cp.offset as u64),
                    ),
                );
                old_offset += old_cp.size as u64;
            }
        }
    }
    let reuse_map = Arc::new(reuse_map);

    // === Global byte-progress accounting ===
    let total_files = manifest.file_manifest_list.elements.len();
    let total_bytes: u64 = manifest
        .file_manifest_list
        .elements
        .iter()
        .map(|fm| fm.file_size as u64)
        .sum();
    let downloaded_bytes = Arc::new(Mutex::new(0u64));

    // Limit concurrency
    let semaphore = Arc::new(Semaphore::new(10));
    let mut task_handles = Vec::new();

    for fm in manifest.file_manifest_list.elements.clone() {
        if control.cancelled.load(Ordering::Relaxed) {
            break;
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let final_path = install_dir.join(&fm.filename);
        let tmp_path = install_dir.join(format!("{}.tmp", &fm.filename));
        let chunk_map = chunk_map.clone();
        let reuse_map = reuse_map.clone();
        let tx = tx.clone();
        let control = control.clone();
        let bucket = bucket.clone();
        let install_dir_clone = install_dir.clone();
        let downloaded_bytes = downloaded_bytes.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit; // keep the semaphore slot until task completes

            // Retry logic for the entire file download
            const MAX_FILE_RETRIES: u32 = 3;
            let mut file_attempt = 0;

            loop {
                if control.cancelled.load(Ordering::Relaxed) {
                    // Clean up partial file on cancellation
                    let _ = fs::remove_file(&tmp_path).await;
                    return Err(DownloadError::Cancelled);
                }

                match download_file_attempt(
                    &fm,
                    &final_path,
                    &tmp_path,
                    &chunk_map,
                    &reuse_map,
                    &tx,
                    &control,
                    &bucket,
                    &install_dir_clone,
                    total_files,
                    total_bytes,
                    downloaded_bytes.clone(),
                )
                .await {
                    Ok(()) => break,
                    Err(DownloadError::Cancelled) => {
                        let _ = fs::remove_file(&tmp_path).await;
                        return Err(DownloadError::Cancelled);
                    }
                    Err(e) => {
                        file_attempt += 1;

                        if file_attempt > MAX_FILE_RETRIES {
                            let _ = fs::remove_file(&tmp_path).await;
                            return Err(e);
                        }

                        eprintln!(
                            "File download attempt {} failed for {}: {}. Retrying...",
                            file_attempt, fm.filename, e
                        );

                        // Clean up partial file
                        let _ = fs::remove_file(&tmp_path).await;

                        // Backoff with frequent cancellation checks
                        let retry_delay = Duration::from_millis(1000 * file_attempt as u64);
                        let mut elapsed = Duration::ZERO;
                        while elapsed < retry_delay {
                            if control.cancelled.load(Ordering::Relaxed) {
                                let _ = fs::remove_file(&tmp_path).await;
                                return Err(DownloadError::Cancelled);
                            }
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            elapsed += Duration::from_millis(100);
                        }
                    }
                }
            }

            Ok::<(), DownloadError>(())
        });

        task_handles.push(handle);
    }

    // Await tasks with cooperative cancellation
    let mut results = Vec::new();
    let mut cancelled = false;

    for handle in task_handles {
        if control.cancelled.load(Ordering::Relaxed) {
            handle.abort();
            cancelled = true;
            continue;
        }

        match handle.await {
            Ok(Ok(())) => results.push(Ok(())),
            Ok(Err(DownloadError::Cancelled)) => {
                cancelled = true;
                break;
            }
            Ok(Err(e)) => results.push(Err(e)),
            Err(_) => {
                // Aborted/panic -> treat as cancelled
                cancelled = true;
                break;
            }
        }
    }

    if cancelled {
        return Err(DownloadError::Cancelled);
    }

    let failed_files: Vec<DownloadError> = results.into_iter().filter_map(|r| r.err()).collect();
    if !failed_files.is_empty() {
        return Err(DownloadError::Multiple(failed_files));
    }

    // Final 100% progress tick to ensure UI flips to complete
    {
        let mut g = downloaded_bytes.lock().await;
        if *g < total_bytes {
            *g = total_bytes;
        }
        let _ = tx
            .send(ProgressUpdate {
                filename: String::new(),
                downloaded_bytes: *g,
                total_bytes,
                total_files,
            })
            .await;
    }

    Ok(())
}

async fn download_file_attempt(
    fm: &crate::manifest::file_manifest::FileManifest,
    final_path: &PathBuf,
    tmp_path: &PathBuf,
    chunk_map: &Arc<HashMap<u128, ChunkInfo>>,
    reuse_map: &Arc<HashMap<(String, u128, u32, u32), (String, u64)>>,
    tx: &mpsc::Sender<ProgressUpdate>,
    control: &Arc<DownloadControl>,
    bucket: &str,
    install_dir_clone: &PathBuf,
    total_files: usize,
    total_bytes: u64,
    downloaded_bytes: Arc<Mutex<u64>>,
) -> Result<(), DownloadError> {
    // Early cancellation
    if control.cancelled.load(Ordering::Relaxed) {
        return Err(DownloadError::Cancelled);
    }

    // Fast path: existing file matches expected hash -> count its bytes toward global progress
    if final_path.exists() {
        if let Ok(existing) = fs::read(&final_path).await {
            let mut hasher = Sha1::new();
            hasher.update(&existing);
            let hash = hasher.finalize();
            if hash.as_slice() == fm.hash.as_slice() {
                {
                    let mut global = downloaded_bytes.lock().await;
                    *global += fm.file_size as u64;
                    let _ = tx
                        .send(ProgressUpdate {
                            filename: fm.filename.clone(),
                            downloaded_bytes: *global,
                            total_bytes,
                            total_files,
                        })
                        .await;
                }
                return Ok(());
            }
        }
    }

    if let Some(parent) = tmp_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut file = File::create(&tmp_path).await?;

    for cp in &fm.chunk_parts {
        if control.cancelled.load(Ordering::Relaxed) {
            return Err(DownloadError::Cancelled);
        }

        let guid = guid_to_u128(&cp.guid);
        let key = (fm.filename.clone(), guid, cp.offset, cp.size);

        // Reuse bytes from existing file (from old manifest mapping)
        if let Some((source_file, source_offset)) = reuse_map.get(&key) {
            let source_path = install_dir_clone.join(source_file);
            if let Ok(mut existing_file) = File::open(&source_path).await {
                let mut buffer = vec![0u8; cp.size as usize];
                existing_file
                    .seek(std::io::SeekFrom::Start(*source_offset))
                    .await?;
                existing_file.read_exact(&mut buffer).await?;

                file.write_all(&buffer).await?;

                // Global progress update
                {
                    let mut global = downloaded_bytes.lock().await;
                    *global += buffer.len() as u64;
                    let _ = tx
                        .send(ProgressUpdate {
                            filename: fm.filename.clone(),
                            downloaded_bytes: *global,
                            total_bytes,
                            total_files,
                        })
                        .await;
                }

                continue;
            }
        }

        // Download the chunk, then slice the part we need
        let chunk = chunk_map.get(&guid).ok_or(DownloadError::ChunkMissing)?;
        let chunk_key = format!(
            "ChunksV4/{:02}/{:016X}_{}.chunk",
            chunk.group_num,
            chunk.hash,
            chunk
                .guid
                .iter()
                .map(|g| format!("{:08X}", g))
                .collect::<String>()
        );

        let chunk_data = download_chunk_with_cancellation(bucket, &chunk_key, control.clone()).await?;

        let start = cp.offset as usize;
        let end = start + cp.size as usize;
        if end > chunk_data.len() {
            return Err(DownloadError::ChunkCorrupt(format!(
                "Chunk part out of bounds: {} (chunk len = {}, start = {}, end = {})",
                chunk_key,
                chunk_data.len(),
                start,
                end
            )));
        }

        let slice = &chunk_data[start..end];

        file.write_all(slice).await?;

        // Global progress update
        {
            let mut global = downloaded_bytes.lock().await;
            *global += slice.len() as u64;
            let _ = tx
                .send(ProgressUpdate {
                    filename: fm.filename.clone(),
                    downloaded_bytes: *global,
                    total_bytes,
                    total_files,
                })
                .await;
        }
    }

    if control.cancelled.load(Ordering::Relaxed) {
        return Err(DownloadError::Cancelled);
    }

    file.flush().await?;
    drop(file);

    // Verify final file hash
    let data = fs::read(&tmp_path).await?;
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = hasher.finalize();
    if hash.as_slice() != fm.hash.as_slice() {
        return Err(DownloadError::HashMismatch(fm.filename.clone()));
    }

    fs::rename(&tmp_path, &final_path).await?;
    Ok(())
}


// Wrapper around chunk download that can be cancelled more aggressively
async fn download_chunk_with_cancellation(
    bucket: &str,
    chunk_key: &str,
    control: Arc<DownloadControl>,
) -> Result<Vec<u8>, DownloadError> {
    // Set a reasonable timeout for chunk downloads (e.g., 30 seconds)
    const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);
    
    // Check if cancelled before starting
    if control.cancelled.load(Ordering::Relaxed) {
        return Err(DownloadError::Cancelled);
    }

    // Create a future that checks for cancellation every 100ms
    let cancellation_checker = async {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if control.cancelled.load(Ordering::Relaxed) {
                return Err(DownloadError::Cancelled);
            }
        }
    };

    // Race the download against the cancellation checker and timeout
    tokio::select! {
        result = tokio::time::timeout(CHUNK_TIMEOUT, download_chunk_from_r2_streaming(bucket, chunk_key)) => {
            match result {
                Ok(chunk_result) => chunk_result.map_err(|_| DownloadError::UnexpectedError),
                Err(_) => Err(DownloadError::Timeout),
            }
        }
        cancelled = cancellation_checker => {
            cancelled
        }
    }
}