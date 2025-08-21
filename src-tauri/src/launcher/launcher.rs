use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::manifest::downloader::download_utils::download_from_bucket_streaming;
use crate::manifest::errors::ChunkLoadError;

/// Download the game's launcher. This will be called on every launch.
pub async fn download_launcher(path: &str) -> Result<(), ChunkLoadError> {
    let base_path = PathBuf::from(path);
    let binaries_path = base_path.join("FortniteGame/Binaries/Win64");
    let launcher_path = binaries_path.join("RealityLauncher.exe");

    // Download the launcher buffer
    let launcher_buffer: Vec<u8> = download_from_bucket_streaming("reality-manifest", "RealityLauncher.exe").await?;

    // Remove existing anticheat file in Reality directory if it exists
    if launcher_path.exists() {
        fs::remove_file(&launcher_path).await?;
    }

    // Write the buffer to the launcher path
    let mut file = fs::File::create(&launcher_path).await?;
    file.write_all(&launcher_buffer).await?;
    file.flush().await?;

    Ok(())
}