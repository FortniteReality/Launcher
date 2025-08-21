pub mod downloader;

pub mod chunk_data;
pub mod custom_fields;
pub mod errors;
pub mod file_manifest;
pub mod manifest_data;
pub mod manifest_utils;

use base64::{prelude::BASE64_STANDARD, Engine};
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read};

pub use chunk_data::read_chunk_datalist;
pub use custom_fields::read_custom_fields;
pub use file_manifest::read_file_manifest_list;
pub use manifest_data::{read_manifest_meta, ParsedManifest};

pub use errors::ManifestError;

/// Parses the manifest data from a byte vector. NOTE: This function assumes the manifest is using the ChunkV4 format.
pub async fn parse_manifest(data: Vec<u8>) -> Result<ParsedManifest, ManifestError> {
    let mut cursor = Cursor::new(data);
    let header_magic = cursor.read_u32::<LittleEndian>()?;
    if header_magic != 0x44BEC00C {
        return Err(ManifestError::InvalidMagic);
    }

    let _header_size = cursor.read_u32::<LittleEndian>()?;
    let size_uncompressed = cursor.read_u32::<LittleEndian>()?;
    let size_compressed = cursor.read_u32::<LittleEndian>()?;
    let mut sha_hash = [0u8; 20];
    cursor.read_exact(&mut sha_hash)?;
    let stored_as = cursor.read_u8()?;
    let version = cursor.read_u32::<LittleEndian>()?;

    let compressed = stored_as & 0x1 != 0;
    let mut data = vec![0u8; size_compressed as usize];
    cursor.read_exact(&mut data)?;

    if compressed {
        let mut decoder = ZlibDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        let mut hasher = Sha1::new();
        hasher.update(&decompressed);
        let dec_hash = hasher.finalize();
        if dec_hash[..] != sha_hash[..] {
            return Err(ManifestError::HashMismatch);
        }

        cursor = Cursor::new(decompressed);
    } else {
        cursor = Cursor::new(data);
    }

    if cursor.get_ref().iter().len() != size_uncompressed as usize {
        return Err(ManifestError::SizeMismatch);
    }

    let meta = read_manifest_meta(&mut cursor)?;
    let chunk_data_list = read_chunk_datalist(&mut cursor)?;
    let file_manifest_list = read_file_manifest_list(&mut cursor)?;
    let custom_fields = read_custom_fields(&mut cursor)?;

    Ok(ParsedManifest {
        version,
        compressed,
        meta,
        chunk_data_list,
        file_manifest_list,
        custom_fields,
    })
}

/// Fetches the current manifest as a Base64 encoded string.
pub async fn fetch_current_manifest_as_b64() -> Result<String, ManifestError> {
    let manifest = downloader::download_manifest().await?;
    Ok(BASE64_STANDARD.encode(manifest))
}

/// Marks the current manifest as complete by completing the download process.
pub async fn mark_current_manifest_as_complete(installed_location: &String) -> Result<(), ManifestError> {
    downloader::complete_manifest_download(installed_location).await?;
    Ok(())
}

/// Marks the game as deleted and deletes the downloaded manifests.
pub async fn mark_game_as_deleted() -> Result<(), ManifestError> {
    downloader::mark_game_as_deleted().await?;
    Ok(())
}