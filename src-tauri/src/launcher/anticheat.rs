use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::manifest::downloader::download_utils::download_from_bucket_streaming;
use crate::manifest::errors::ChunkLoadError;

/// Download the game's anticheat. This will be called every time before injection | Yes I know that solution is not IDEAL, but it should work.
pub async fn download_anticheat(path: &str) -> Result<(), ChunkLoadError> {
    let base_path = PathBuf::from(path);
    let reality_path = base_path.join("FortniteGame/Binaries/Win64/Reality");
    let anticheat_path = reality_path.join("Equinox.dll");

    // Create the Reality directory next to the EAC and BE folders
    fs::create_dir_all(&reality_path).await?;

    // Download the anticheat buffer
    let anticheat_buffer: Vec<u8> = download_from_bucket_streaming("reality-manifest", "Equinox.dll").await?;

    // Remove existing anticheat file in Reality directory if it exists
    if anticheat_path.exists() {
        fs::remove_file(&anticheat_path).await?;
    }

    // Write the buffer to the anticheat path
    let mut file = fs::File::create(&anticheat_path).await?;
    file.write_all(&anticheat_buffer).await?;
    file.flush().await?;

    Ok(())
}