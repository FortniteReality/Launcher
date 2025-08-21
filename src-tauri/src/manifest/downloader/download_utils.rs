use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, Config};
use std::time::Duration;
use tokio::time::sleep;

use crate::manifest::chunk_data::load_chunk;
use crate::manifest::errors::ChunkLoadError;

pub fn hardcoded_s3_client() -> Client {
    let creds = Credentials::new(
        "e693383e28bf67df0edb35dc9b90ff87", // I don't mind hardcoding these because they have READ ONLY perms
        "8e0ae7ee2acd40bb667f93222a06e08d6eb780830ca746f558fa017a103f7e13",
        None,
        None,
        "static-launcher-provider",
    );

    let config = Config::builder()
        .region(Region::new("auto"))
        .endpoint_url("https://7f1c1cf93e513d24cf739065fe65c3da.r2.cloudflarestorage.com")
        .credentials_provider(creds)
        .behavior_version_latest()
        .timeout_config(
            aws_sdk_s3::config::timeout::TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(60))
                .operation_attempt_timeout(Duration::from_secs(30))
                .build(),
        )
        .build();

    Client::from_conf(config)
}

pub async fn download_chunk_from_r2(bucket: &str, key: &str) -> Result<Vec<u8>, ChunkLoadError> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

    let mut attempt = 0;

    loop {
        match download_chunk_attempt(bucket, key).await {
            Ok(data) => return Ok(data),
            Err(e) => {
                attempt += 1;

                if attempt > MAX_RETRIES {
                    return Err(e);
                }

                // Exponential backoff
                let backoff = INITIAL_BACKOFF * (2_u32.pow(attempt - 1));
                eprintln!(
                    "Download attempt {} failed for {}: {}. Retrying in {:?}...",
                    attempt, key, e, backoff
                );

                sleep(backoff).await;
            }
        }
    }
}

async fn download_chunk_attempt(bucket: &str, key: &str) -> Result<Vec<u8>, ChunkLoadError> {
    let resp = hardcoded_s3_client()
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| {
            ChunkLoadError::DownloadFailed(key.to_string(), format!("Request failed: {}", e))
        })?;

    let raw = resp.body.collect().await.map_err(|e| {
        ChunkLoadError::DownloadFailed(key.to_string(), format!("Body read error: {}", e))
    })?;

    load_chunk(&raw.into_bytes())
}

// Alternative approach with manual streaming for better control
pub async fn download_chunk_from_r2_streaming(
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, ChunkLoadError> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

    let mut attempt = 0;

    loop {
        match download_chunk_streaming_attempt(bucket, key).await {
            Ok(data) => return Ok(data),
            Err(e) => {
                attempt += 1;

                if attempt > MAX_RETRIES {
                    return Err(e);
                }

                let backoff = INITIAL_BACKOFF * (2_u32.pow(attempt - 1));
                eprintln!(
                    "Streaming download attempt {} failed for {}: {}. Retrying in {:?}...",
                    attempt, key, e, backoff
                );

                sleep(backoff).await;
            }
        }
    }
}

async fn download_chunk_streaming_attempt(
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, ChunkLoadError> {
    use tokio::time::timeout;

    let resp = hardcoded_s3_client()
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| {
            ChunkLoadError::DownloadFailed(key.to_string(), format!("Request failed: {}", e))
        })?;

    let content_length = resp.content_length().unwrap_or(0) as usize;
    let mut buffer = Vec::with_capacity(content_length);

    let mut stream = resp.body.into_async_read();
    let mut chunk_buffer = [0u8; 8192]; // 8KB chunks

    loop {
        match timeout(
            Duration::from_secs(10),
            tokio::io::AsyncReadExt::read(&mut stream, &mut chunk_buffer),
        )
        .await
        {
            Ok(Ok(0)) => break, // EOF
            Ok(Ok(n)) => {
                buffer.extend_from_slice(&chunk_buffer[..n]);
            }
            Ok(Err(e)) => {
                return Err(ChunkLoadError::DownloadFailed(
                    key.to_string(),
                    format!("Read error: {}", e),
                ));
            }
            Err(_) => {
                return Err(ChunkLoadError::DownloadFailed(
                    key.to_string(),
                    "Read timeout".to_string(),
                ));
            }
        }
    }

    load_chunk(&buffer)
}

// Manual streaming to download a file from the bucket
pub async fn download_from_bucket_streaming(
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, ChunkLoadError> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF: Duration = Duration::from_millis(500);

    let mut attempt = 0;

    loop {
        match download_from_bucket_streaming_attempt(bucket, key).await {
            Ok(data) => return Ok(data),
            Err(e) => {
                attempt += 1;

                if attempt > MAX_RETRIES {
                    return Err(e);
                }

                let backoff = INITIAL_BACKOFF * (2_u32.pow(attempt - 1));
                eprintln!(
                    "Streaming download attempt {} failed for {}: {}. Retrying in {:?}...",
                    attempt, key, e, backoff
                );

                sleep(backoff).await;
            }
        }
    }
}

async fn download_from_bucket_streaming_attempt(
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, ChunkLoadError> {
    use tokio::time::timeout;

    let resp = hardcoded_s3_client()
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| {
            ChunkLoadError::DownloadFailed(key.to_string(), format!("Request failed: {}", e))
        })?;

    let content_length = resp.content_length().unwrap_or(0) as usize;
    let mut buffer = Vec::with_capacity(content_length);

    let mut stream = resp.body.into_async_read();
    let mut chunk_buffer = [0u8; 8192]; // 8KB chunks

    loop {
        match timeout(
            Duration::from_secs(10),
            tokio::io::AsyncReadExt::read(&mut stream, &mut chunk_buffer),
        )
        .await
        {
            Ok(Ok(0)) => break, // EOF
            Ok(Ok(n)) => {
                buffer.extend_from_slice(&chunk_buffer[..n]);
            }
            Ok(Err(e)) => {
                return Err(ChunkLoadError::DownloadFailed(
                    key.to_string(),
                    format!("Read error: {}", e),
                ));
            }
            Err(_) => {
                return Err(ChunkLoadError::DownloadFailed(
                    key.to_string(),
                    "Read timeout".to_string(),
                ));
            }
        }
    }

    Ok(buffer)
}

pub fn guid_to_u128(guid: &[u32; 4]) -> u128 {
    (guid[0] as u128) << 96 | (guid[1] as u128) << 64 | (guid[2] as u128) << 32 | (guid[3] as u128)
}
