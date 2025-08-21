// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{Parser, Subcommand};
use std::fs;
use std::sync::Arc;

use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use aws_sdk_s3::Config;

use reality_lib::commands::{start_download_internal, start_verify_internal, DownloadControl};

#[derive(Parser)]
#[command(name = "reality-manifest")]
#[command(about = "A tool for managing chunk file manifests with CloudFlare R2")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Upload {
        /// Source directory containing chunks
        #[arg(short, long)]
        cloud_dir: PathBuf,

        /// R2 Bucket name
        #[arg(short, long)]
        bucket: String,

        /// Access key for bucket
        #[arg(long)]
        access_key: String,

        // Secret key for bucket
        #[arg(long)]
        secret_key: String,
    },

    Remove {
        /// Source directory containing chunks to remove
        #[arg(short, long)]
        cloud_dir: PathBuf,

        /// R2 Bucket name
        #[arg(short, long)]
        bucket: String,

        /// Access key for bucket
        #[arg(long)]
        access_key: String,

        // Secret key for bucket
        #[arg(long)]
        secret_key: String,
    },

    Download {
        /// Install directory
        #[arg(short, long)]
        install_dir: String,

        /// R2 Bucket name
        #[arg(short, long)]
        bucket: String,

        /// Manifest file
        #[arg(short, long)]
        manifest: String,
    },

    Verify {
        /// Install directory
        #[arg(short, long)]
        install_dir: String,

        /// R2 Bucket name
        #[arg(short, long)]
        bucket: String,

        /// Manifest file
        #[arg(short, long)]
        manifest: String,
    },
}

async fn upload_single_file_with_retry(
    client: &Client,
    bucket: &str,
    path: &PathBuf,
    key: &str,
    max_retries: usize,
) -> Result<()> {
    let mut attempt = 0;
    let base_delay_ms = 250; // Start with 250ms
    let max_delay_ms = 30_000; // Cap at 30 seconds
    
    loop {
        attempt += 1;
        
        match upload_single_file(client, bucket, path, key).await {
            Ok(_) => {
                if attempt > 1 {
                    println!("Successfully uploaded {} on attempt {}", path.display(), attempt);
                } else {
                    println!("Uploaded {}", path.display());
                }
                return Ok(());
            }
            Err(e) => {
                if attempt > max_retries {
                    eprintln!("Failed to upload {} after {} attempts: {}", path.display(), max_retries, e);
                    return Err(e);
                }
                
                // Exponential backoff: 250ms, 500ms, 1s, 2s, 4s, etc. (capped at 30s)
                let delay_ms = std::cmp::min(base_delay_ms * (1 << (attempt - 1)), max_delay_ms);
                eprintln!("Upload attempt {} failed for {}: {}. Retrying in {}ms...", 
                         attempt, path.display(), e, delay_ms);
                
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

async fn upload_single_file(
    client: &Client,
    bucket: &str,
    path: &PathBuf,
    key: &str,
) -> Result<()> {
    let body = ByteStream::from_path(path).await?;
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    Ok(())
}

async fn upload_files(
    cloud_dir: &PathBuf,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<()> {
    let creds = Credentials::new(access_key, secret_key, None, None, "admin");

    let config = Config::builder()
        .region(Region::new("auto"))
        .endpoint_url("https://7f1c1cf93e513d24cf739065fe65c3da.r2.cloudflarestorage.com")
        .credentials_provider(creds)
        .behavior_version_latest()
        .build();

    let client = Client::from_conf(config);
    let chunks_dir = cloud_dir.join("ChunksV4");

    let mut total_files = 0;
    let mut skipped_files = 0;
    let mut uploaded_files = 0;
    let mut failed_files = 0;

    for group in fs::read_dir(&chunks_dir)? {
        let group_path = group?.path();
        for file in fs::read_dir(&group_path)? {
            let path = file?.path();
            if path.extension().map(|e| e == "chunk").unwrap_or(false) {
                total_files += 1;
                
                let metadata = fs::metadata(&path)?;
                if metadata.len() == 0 {
                    println!("Skipping empty file: {}", path.display());
                    skipped_files += 1;
                    continue;
                }

                let key = format!(
                    "ChunksV4/{}/{}",
                    group_path.file_name().unwrap().to_string_lossy(),
                    path.file_name().unwrap().to_string_lossy()
                );

                // Check if file already exists
                match client
                    .head_object()
                    .bucket(bucket)
                    .key(&key)
                    .send()
                    .await
                {
                    Ok(head_output) => {
                        if head_output.content_length().unwrap_or(0) > 0 {
                            println!("File already exists and is not empty: {}", key);
                            skipped_files += 1;
                            continue;
                        }
                        println!("File exists but is empty in bucket, uploading: {}", key);
                    }
                    Err(_) => {
                        println!("File doesn't exist in bucket, uploading: {}", key);
                    }
                }

                // Upload with retry logic
                match upload_single_file_with_retry(&client, bucket, &path, &key, 5).await {
                    Ok(_) => uploaded_files += 1,
                    Err(_) => failed_files += 1,
                }
            }
        }
    }

    println!("\nUpload Summary:");
    println!("Total files processed: {}", total_files);
    println!("Files uploaded: {}", uploaded_files);
    println!("Files skipped: {}", skipped_files);
    println!("Files failed: {}", failed_files);
    
    if failed_files > 0 {
        return Err(anyhow::anyhow!("{} files failed to upload", failed_files));
    }

    Ok(())
}

async fn remove_files(
    cloud_dir: &PathBuf,
    bucket: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<()> {
    let creds = Credentials::new(access_key, secret_key, None, None, "admin");

    let config = Config::builder()
        .region(Region::new("auto"))
        .endpoint_url("https://7f1c1cf93e513d24cf739065fe65c3da.r2.cloudflarestorage.com")
        .credentials_provider(creds)
        .behavior_version_latest()
        .build();

    let client = Client::from_conf(config);
    let chunks_dir = cloud_dir.join("ChunksV4");

    let mut total_files = 0;
    let mut removed_files = 0;
    let mut not_found_files = 0;
    let mut failed_files = 0;

    for group in fs::read_dir(&chunks_dir)? {
        let group_path = group?.path();
        for file in fs::read_dir(&group_path)? {
            let path = file?.path();
            if path.extension().map(|e| e == "chunk").unwrap_or(false) {
                total_files += 1;

                let key = format!(
                    "ChunksV4/{}/{}",
                    group_path.file_name().unwrap().to_string_lossy(),
                    path.file_name().unwrap().to_string_lossy()
                );

                println!("Attempting to remove: {}", key);

                match client
                    .delete_object()
                    .bucket(bucket)
                    .key(&key)
                    .send()
                    .await
                {
                    Ok(_) => {
                        println!("Removed {}", key);
                        removed_files += 1;
                    }
                    Err(e) => {
                        // Check if it's a not found error
                        if e.to_string().contains("NoSuchKey") || e.to_string().contains("404") {
                            println!("File not found in bucket: {}", key);
                            not_found_files += 1;
                        } else {
                            eprintln!("Failed to remove {}: {}", key, e);
                            failed_files += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\nRemoval Summary:");
    println!("Total files processed: {}", total_files);
    println!("Files removed: {}", removed_files);
    println!("Files not found: {}", not_found_files);
    println!("Files failed to remove: {}", failed_files);
    
    if failed_files > 0 {
        return Err(anyhow::anyhow!("{} files failed to be removed", failed_files));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Upload {
            cloud_dir,
            bucket,
            access_key,
            secret_key,
        }) => {
            upload_files(&cloud_dir, &bucket, &access_key, &secret_key).await?;
        }

        Some(Commands::Remove {
            cloud_dir,
            bucket,
            access_key,
            secret_key,
        }) => {
            remove_files(&cloud_dir, &bucket, &access_key, &secret_key).await?;
        }

        Some(Commands::Download {
            install_dir,
            bucket,
            manifest,
        }) => {
            let file_bytes = fs::read(manifest)?;
            let manifest_b64 = BASE64_STANDARD.encode(&file_bytes);

            let control = Arc::new(DownloadControl::default());

            start_download_internal(manifest_b64, None, bucket, install_dir, control).await?;
        }

        Some(Commands::Verify {
            install_dir,
            bucket,
            manifest,
        }) => {
            let file_bytes = fs::read(manifest)?;
            let manifest_b64 = BASE64_STANDARD.encode(&file_bytes);

            let control = Arc::new(DownloadControl::default());

            start_verify_internal(manifest_b64, bucket, install_dir, control).await?;
        }

        None => {
            reality_lib::run();
        }
    }

    Ok(())
}