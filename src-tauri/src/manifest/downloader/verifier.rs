use futures::stream::{FuturesUnordered, StreamExt};
use sha1::{Digest, Sha1};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt};
use tokio::sync::{mpsc, Semaphore, Mutex};

use crate::manifest::chunk_data::ChunkInfo;
use crate::manifest::downloader::download_utils::{download_chunk_from_r2_streaming, guid_to_u128};
use crate::manifest::downloader::errors::DownloadError;
use crate::manifest::downloader::progress_update::ProgressUpdate; // same struct as downloader/installer
use crate::manifest::ParsedManifest;
use crate::DownloadControl;

// === Parallel verifier that mirrors downloader progress semantics ===
// - Uses a GLOBAL downloaded_bytes counter across all files
// - Sends ProgressUpdate { filename, downloaded_bytes, total_bytes, total_files } after every slice write or skip
// - Emits a final 100% tick at the end
pub async fn verify_and_repair_parallel(
    manifest: ParsedManifest,
    bucket: String,
    install_dir: PathBuf,
    tx: mpsc::Sender<ProgressUpdate>,
    control: Arc<DownloadControl>,
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

    // Global totals (match downloader)
    let total_files = manifest.file_manifest_list.elements.len();
    let total_bytes: u64 = manifest
        .file_manifest_list
        .elements
        .iter()
        .map(|fm| fm.file_size as u64)
        .sum();
    let downloaded_bytes = Arc::new(Mutex::new(0u64));

    println!("Starting verification: {} files, {} total bytes", total_files, total_bytes);

    // Concurrency similar to downloader (adjust if you want them identical)
    let semaphore = Arc::new(Semaphore::new(10));
    let mut tasks = FuturesUnordered::new();

    for fm in manifest.file_manifest_list.elements.clone() {
        if control.cancelled.load(Ordering::Relaxed) {
            return Err(DownloadError::Cancelled);
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let out_path = install_dir.join(&fm.filename);
        let tmp_path = install_dir.join(format!("{}.tmp", &fm.filename));
        let chunk_map = chunk_map.clone();
        let tx = tx.clone();
        let control = control.clone();
        let bucket = bucket.clone();
        let downloaded_bytes = downloaded_bytes.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = permit;

            const MAX_FILE_RETRIES: u32 = 3;
            let mut file_attempt = 0;

            loop {
                if control.cancelled.load(Ordering::Relaxed) {
                    return Err(DownloadError::Cancelled);
                }

                match verify_and_repair_file_attempt(
                    &fm,
                    &out_path,
                    &tmp_path,
                    &chunk_map,
                    &tx,
                    &control,
                    &bucket,
                    total_files,
                    total_bytes,
                    downloaded_bytes.clone(),
                )
                .await {
                    Ok(()) => break,
                    Err(DownloadError::Cancelled) => return Err(DownloadError::Cancelled),
                    Err(e) => {
                        file_attempt += 1;
                        if file_attempt > MAX_FILE_RETRIES {
                            let _ = fs::remove_file(&tmp_path).await;
                            return Err(e);
                        }
                        eprintln!(
                            "File verification/repair attempt {} failed for {}: {}. Retrying...",
                            file_attempt, fm.filename, e
                        );
                        let _ = fs::remove_file(&tmp_path).await;
                        tokio::time::sleep(Duration::from_millis(1000 * file_attempt as u64)).await;
                    }
                }
            }

            Ok::<(), DownloadError>(())
        }));
    }

    let mut failed_files = Vec::new();
    while let Some(res) = tasks.next().await {
        if let Err(e) = res? {
            failed_files.push(e);
        }
    }

    // Check final downloaded bytes before determining result
    let final_downloaded = {
        let guard = downloaded_bytes.lock().await;
        *guard
    };
    
    println!("Verification complete: {}/{} bytes processed", final_downloaded, total_bytes);

    // Determine the result but always send final tick
    let result = if !failed_files.is_empty() {
        Err(DownloadError::Multiple(failed_files))
    } else {
        Ok(())
    };

    // CRITICAL: Ensure the final progress update gets sent and processed
    // Send multiple completion signals to guarantee the UI receives one
    for i in 0..3 {
        let send_result = tx
            .send(ProgressUpdate {
                filename: if result.is_ok() { 
                    format!("Verification complete ({})", i + 1)
                } else { 
                    format!("Verification failed ({})", i + 1)
                },
                downloaded_bytes: total_bytes, // Force 100%
                total_bytes,
                total_files,
            })
            .await;
            
        if send_result.is_err() {
            eprintln!("Warning: Failed to send final progress update {}", i + 1);
        } else {
            println!("Sent final progress update {}: {}/{} bytes", i + 1, total_bytes, total_bytes);
        }
        
        // Small delay between sends
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Give the progress updates time to be processed
    tokio::time::sleep(Duration::from_millis(500)).await;

    result
}

// Per-file verification/repair. Sends global-style progress just like downloader.
async fn verify_and_repair_file_attempt(
    fm: &crate::manifest::file_manifest::FileManifest,
    out_path: &PathBuf,
    tmp_path: &PathBuf,
    chunk_map: &Arc<HashMap<u128, ChunkInfo>>,
    tx: &mpsc::Sender<ProgressUpdate>,
    control: &Arc<DownloadControl>,
    bucket: &str,
    total_files: usize,
    total_bytes: u64,
    downloaded_bytes: Arc<Mutex<u64>>,
) -> Result<(), DownloadError> {
    // If the existing file is already valid, count its bytes toward global progress and return.
    if out_path.exists() {
        let existing = fs::read(&out_path).await?;
        let mut hasher = Sha1::new();
        hasher.update(&existing);
        let hash = hasher.finalize();
        if hash.as_slice() == fm.hash.as_slice() {
            // CRITICAL FIX: Add the ACTUAL file size, not the file size from manifest
            // This ensures consistency between valid files and repaired files
            let actual_size = existing.len() as u64;
            let mut global = downloaded_bytes.lock().await;
            *global += actual_size;
            let current_progress = *global;
            drop(global); // Release lock before sending
            
            // Log the progress update
            println!("File {} already valid: {}/{} bytes total", fm.filename, current_progress, total_bytes);
            
            let send_result = tx
                .send(ProgressUpdate {
                    filename: fm.filename.clone(),
                    downloaded_bytes: current_progress,
                    total_bytes,
                    total_files,
                })
                .await;
                
            if send_result.is_err() {
                eprintln!("Warning: Failed to send progress for valid file {}", fm.filename);
            }
            
            return Ok(());
        }
    }

    // Otherwise, repair into a temporary path
    if let Some(parent) = tmp_path.parent() { fs::create_dir_all(parent).await?; }
    let mut file = File::create(&tmp_path).await?;
    let mut file_bytes_written = 0u64; // Track bytes for this specific file

    for cp in &fm.chunk_parts {
        if control.cancelled.load(Ordering::Relaxed) { return Err(DownloadError::Cancelled); }

        let guid = guid_to_u128(&cp.guid);
        let chunk = chunk_map
            .get(&guid)
            .ok_or_else(|| DownloadError::ChunkMissing)?;
        let key = format!(
            "ChunksV4/{:02}/{:016X}_{}.chunk",
            chunk.group_num,
            chunk.hash,
            chunk
                .guid
                .iter()
                .map(|g| format!("{:08X}", g))
                .collect::<String>()
        );

        // Download the chunk (with simple retry) and write the requested slice to the tmp file
        let chunk_data = download_chunk_with_retry(&bucket, &key).await?;
        let start = cp.offset as usize;
        let end = start + cp.size as usize;
        if end > chunk_data.len() {
            return Err(DownloadError::ChunkCorrupt(format!(
                "Chunk part out of bounds: {} (chunk len = {}, start = {}, end = {})",
                key, chunk_data.len(), start, end
            )));
        }

        let slice = &chunk_data[start..end];
        file.write_all(slice).await?;
        file_bytes_written += slice.len() as u64;

        // Global progress update (match downloader): add slice bytes and emit update
        let mut global = downloaded_bytes.lock().await;
        *global += slice.len() as u64;
        let current_progress = *global;
        drop(global); // Release lock before sending
        
        let send_result = tx
            .send(ProgressUpdate {
                filename: fm.filename.clone(),
                downloaded_bytes: current_progress,
                total_bytes,
                total_files,
            })
            .await;
            
        if send_result.is_err() {
            eprintln!("Warning: Failed to send progress for chunk in file {}", fm.filename);
        }
    }

    file.flush().await?;
    drop(file);

    // Verify final file hash
    let final_data = fs::read(&tmp_path).await?;
    let mut hasher = Sha1::new();
    hasher.update(&final_data);
    let final_hash = hasher.finalize();
    if final_hash.as_slice() != fm.hash.as_slice() {
        return Err(DownloadError::RepairFailed(fm.filename.clone()));
    }

    // Consistency check: ensure the bytes we wrote match the actual file size
    let actual_file_size = final_data.len() as u64;
    if file_bytes_written != actual_file_size {
        eprintln!(
            "Warning: File {} size mismatch. Written: {}, Actual: {}, Expected: {}",
            fm.filename, file_bytes_written, actual_file_size, fm.file_size
        );
    }

    fs::rename(&tmp_path, &out_path).await?;
    println!("Repaired file: {} ({} bytes)", fm.filename, actual_file_size);
    Ok(())
}

// Simple retry wrapper for chunk downloads
async fn download_chunk_with_retry(bucket: &str, key: &str) -> Result<Vec<u8>, DownloadError> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);
    let mut attempt = 0;

    loop {
        match download_chunk_from_r2_streaming(bucket, key).await {
            Ok(data) => return Ok(data),
            Err(e) => {
                attempt += 1;
                if attempt > MAX_RETRIES {
                    return Err(DownloadError::ChunkDownloadFailed(
                        key.to_string(),
                        format!("{}", e),
                    ));
                }
                let backoff = INITIAL_BACKOFF * (2_u32.pow(attempt - 1));
                eprintln!(
                    "Chunk download attempt {} failed for {}: {}. Retrying in {:?}...",
                    attempt, key, e, backoff
                );
                tokio::time::sleep(backoff).await;
            }
        }
    }
}

// === Optional sequential variant updated to mirror downloader progress ===
pub async fn verify_and_repair_sequential(
    manifest: ParsedManifest,
    bucket: String,
    install_dir: PathBuf,
    tx: mpsc::Sender<ProgressUpdate>,
    control: Arc<DownloadControl>,
) -> Result<(), DownloadError> {
    let chunk_map: HashMap<u128, ChunkInfo> = manifest
        .chunk_data_list
        .elements
        .iter()
        .map(|c| (guid_to_u128(&c.guid), c.clone()))
        .collect();

    let total_files = manifest.file_manifest_list.elements.len();
    let total_bytes: u64 = manifest
        .file_manifest_list
        .elements
        .iter()
        .map(|fm| fm.file_size as u64)
        .sum();
    let downloaded_bytes = Arc::new(Mutex::new(0u64));

    println!("Starting sequential verification: {} files, {} total bytes", total_files, total_bytes);

    let mut overall_success = true;
    let mut last_error = None;

    for fm in manifest.file_manifest_list.elements.iter() {
        if control.cancelled.load(Ordering::Relaxed) { return Err(DownloadError::Cancelled); }

        let out_path = install_dir.join(&fm.filename);
        let tmp_path = install_dir.join(format!("{}.tmp", &fm.filename));

        let mut file_attempt = 0;
        const MAX_FILE_RETRIES: u32 = 3;
        let mut success = false;

        while file_attempt < MAX_FILE_RETRIES && !success {
            file_attempt += 1;

            match repair_file_sequential(
                fm,
                &out_path,
                &tmp_path,
                &chunk_map,
                &tx,
                &control,
                &bucket,
                total_files,
                total_bytes,
                downloaded_bytes.clone(),
            ).await {
                Ok(()) => {
                    success = true;
                }
                Err(DownloadError::Cancelled) => return Err(DownloadError::Cancelled),
                Err(e) => {
                    eprintln!("Sequential repair attempt {} failed for {}: {}", file_attempt, fm.filename, e);
                    let _ = fs::remove_file(&tmp_path).await;
                    last_error = Some(e);
                    if file_attempt < MAX_FILE_RETRIES {
                        tokio::time::sleep(Duration::from_millis(1000 * file_attempt as u64)).await;
                    }
                }
            }
        }

        if !success { 
            overall_success = false;
            break;
        }
    }

    // Check final progress
    let final_downloaded = {
        let guard = downloaded_bytes.lock().await;
        *guard
    };
    
    println!("Sequential verification complete: {}/{} bytes processed", final_downloaded, total_bytes);

    // Determine result but always send final tick
    let result = if overall_success {
        Ok(())
    } else {
        Err(last_error.unwrap_or_else(|| DownloadError::RepairFailed("Unknown file".to_string())))
    };

    // Send multiple final ticks to ensure delivery
    for i in 0..3 {
        let send_result = tx
            .send(ProgressUpdate {
                filename: if result.is_ok() {
                    format!("Sequential verification complete ({})", i + 1)
                } else {
                    format!("Sequential verification failed ({})", i + 1)
                },
                downloaded_bytes: total_bytes, // Force 100% completion
                total_bytes,
                total_files,
            })
            .await;
            
        if send_result.is_err() {
            eprintln!("Warning: Failed to send final sequential progress update {}", i + 1);
        } else {
            println!("Sent final sequential progress update {}: {}/{} bytes", i + 1, total_bytes, total_bytes);
        }
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Give time for processing
    tokio::time::sleep(Duration::from_millis(500)).await;

    result
}

async fn repair_file_sequential(
    fm: &crate::manifest::file_manifest::FileManifest,
    out_path: &PathBuf,
    tmp_path: &PathBuf,
    chunk_map: &HashMap<u128, ChunkInfo>,
    tx: &mpsc::Sender<ProgressUpdate>,
    control: &Arc<DownloadControl>,
    bucket: &str,
    total_files: usize,
    total_bytes: u64,
    downloaded_bytes: Arc<Mutex<u64>>,
) -> Result<(), DownloadError> {
    // If the existing file is already valid, count its bytes and return
    if out_path.exists() {
        let existing = fs::read(&out_path).await?;
        let mut hasher = Sha1::new();
        hasher.update(&existing);
        let hash = hasher.finalize();
        if hash.as_slice() == fm.hash.as_slice() {
            // Use actual file size for consistency
            let actual_size = existing.len() as u64;
            let mut global = downloaded_bytes.lock().await;
            *global += actual_size;
            let current_progress = *global;
            drop(global);
            
            let _ = tx
                .send(ProgressUpdate {
                    filename: fm.filename.clone(),
                    downloaded_bytes: current_progress,
                    total_bytes,
                    total_files,
                })
                .await;
            return Ok(());
        }
    }

    if let Some(parent) = tmp_path.parent() { fs::create_dir_all(parent).await?; }
    let mut file = File::create(&tmp_path).await?;

    for cp in &fm.chunk_parts {
        if control.cancelled.load(Ordering::Relaxed) { return Err(DownloadError::Cancelled); }
        let guid = guid_to_u128(&cp.guid);
        let chunk = chunk_map
            .get(&guid)
            .ok_or_else(|| DownloadError::ChunkMissing)?;
        let key = format!(
            "ChunksV4/{:02}/{:016X}_{}.chunk",
            chunk.group_num,
            chunk.hash,
            chunk
                .guid
                .iter()
                .map(|g| format!("{:08X}", g))
                .collect::<String>()
        );
        let chunk_data = download_chunk_with_retry(&bucket, &key).await?;
        let start = cp.offset as usize;
        let end = start + cp.size as usize;
        if end > chunk_data.len() {
            return Err(DownloadError::ChunkCorrupt(format!(
                "Chunk part out of bounds: {} (chunk len = {}, start = {}, end = {})",
                key, chunk_data.len(), start, end
            )));
        }
        let slice = &chunk_data[start..end];
        file.write_all(slice).await?;

        let mut global = downloaded_bytes.lock().await;
        *global += slice.len() as u64;
        let current_progress = *global;
        drop(global);
        
        let _ = tx
            .send(ProgressUpdate {
                filename: fm.filename.clone(),
                downloaded_bytes: current_progress,
                total_bytes,
                total_files,
            })
            .await;
    }

    file.flush().await?;
    drop(file);

    // Verify and finalize
    let final_data = fs::read(&tmp_path).await?;
    let mut hasher = Sha1::new();
    hasher.update(&final_data);
    let final_hash = hasher.finalize();
    if final_hash.as_slice() != fm.hash.as_slice() {
        return Err(DownloadError::RepairFailed(fm.filename.clone()));
    }
    fs::rename(&tmp_path, &out_path).await?;
    Ok(())
}