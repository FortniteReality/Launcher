use byteorder::{LittleEndian, ReadBytesExt};
use flate2::bufread::ZlibDecoder;
use serde::Serialize;
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    vec,
};

use crate::manifest::errors::ChunkLoadError;
use sha1::{Digest, Sha1};

#[derive(Clone, Serialize, Debug)]
pub struct ChunkInfo {
    pub guid: [u32; 4],
    pub hash: u64,
    pub sha_hash: Vec<u8>,
    pub group_num: u8,
    pub window_size: u32,
    pub file_size: i64,
}

#[derive(Serialize, Debug)]
pub struct ChunkDataList {
    pub version: u8,
    pub elements: Vec<ChunkInfo>,
}

#[derive(Debug)]
pub struct ChunkHeader {
    pub version: u32,
    pub header_size: u32,
    pub compressed_size: u32,
    pub guid: [u32; 4],
    pub rolling_hash: u64,
    pub stored_as: u8,
    pub sha1_hash: Option<[u8; 20]>,
    pub hash_type: Option<u8>,
    pub uncompressed_size: Option<u32>,
}

impl ChunkHeader {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, ChunkLoadError> {
        let magic = cursor.read_u32::<LittleEndian>()?;
        if magic != 0xB1FE3AA2 {
            return Err(ChunkLoadError::InvalidMagic);
        }

        let version = cursor.read_u32::<LittleEndian>()?;
        let header_size = cursor.read_u32::<LittleEndian>()?;
        let compressed_size = cursor.read_u32::<LittleEndian>()?;

        let mut guid = [0u32; 4];
        for g in guid.iter_mut() {
            *g = cursor.read_u32::<LittleEndian>()?;
        }

        let rolling_hash = cursor.read_u64::<LittleEndian>()?;
        let stored_as = cursor.read_u8()?;

        let (sha1_hash, hash_type, uncompressed_size) = match version {
            1 => (None, None, None),
            2 => {
                let mut sha = [0u8; 20];
                cursor.read_exact(&mut sha)?;
                let hash_type = cursor.read_u8()?;
                (Some(sha), Some(hash_type), None)
            }
            3 => {
                let mut sha = [0u8; 20];
                cursor.read_exact(&mut sha)?;
                let hash_type = cursor.read_u8()?;
                let uncompressed_size = cursor.read_u32::<LittleEndian>()?;
                (Some(sha), Some(hash_type), Some(uncompressed_size))
            }
            v => return Err(ChunkLoadError::UnknownVersion(v)),
        };

        Ok(Self {
            version,
            header_size,
            compressed_size,
            guid,
            rolling_hash,
            stored_as,
            sha1_hash,
            hash_type,
            uncompressed_size,
        })
    }
}

pub fn load_chunk(data: &[u8]) -> Result<Vec<u8>, ChunkLoadError> {
    let mut cursor = Cursor::new(data);
    let header = ChunkHeader::parse(&mut cursor)?;

    let file_size = header.header_size as usize + header.compressed_size as usize;
    if file_size > data.len() {
        return Err(ChunkLoadError::IncorrectFileSize);
    }

    if header.stored_as & 0x02 != 0 {
        return Err(ChunkLoadError::UnsupportedStorage);
    }

    if header.hash_type.is_none() {
        return Err(ChunkLoadError::MissingHashInfo);
    }

    // Read compressed data
    cursor.seek(SeekFrom::Start(header.header_size as u64))?;
    let mut compressed = vec![0u8; header.compressed_size as usize];
    cursor.read_exact(&mut compressed)?;

    let mut data_out = Vec::new();

    if header.stored_as & 0x01 != 0 {
        // Compressed
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        decoder
            .read_to_end(&mut data_out)
            .map_err(|_| ChunkLoadError::DecompressFailure)?;

        // If decompressed size doesn't match
        if let Some(expected) = header.uncompressed_size {
            if data_out.len() != expected as usize {
                return Err(ChunkLoadError::DecompressFailure);
            }
        }
    } else {
        data_out = compressed;
    }

    // Validate SHA1
    if let Some(sha_expected) = header.sha1_hash {
        let mut hasher = Sha1::new();
        hasher.update(&data_out);
        let actual = hasher.finalize();
        if actual[..] != sha_expected[..] {
            return Err(ChunkLoadError::HashCheckFailed);
        }
    }

    Ok(data_out)
}

pub fn read_chunk_datalist<R: Read + Seek>(r: &mut R) -> std::io::Result<ChunkDataList> {
    let start_pos = r.stream_position()?;
    let size = r.read_u32::<LittleEndian>()?;
    let version = r.read_u8()?;
    let count = r.read_u32::<LittleEndian>()?;

    let mut elements: Vec<ChunkInfo> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        elements.push(ChunkInfo {
            guid: [0; 4],
            hash: 0,
            sha_hash: vec![0u8; 20],
            group_num: 0,
            window_size: 0,
            file_size: 0,
        });
    }

    for chunk in elements.iter_mut() {
        for i in 0..4 {
            chunk.guid[i] = r.read_u32::<LittleEndian>()?;
        }
    }

    for chunk in elements.iter_mut() {
        chunk.hash = r.read_u64::<LittleEndian>()?;
    }

    for chunk in elements.iter_mut() {
        r.read_exact(&mut chunk.sha_hash)?;
    }

    for chunk in elements.iter_mut() {
        chunk.group_num = r.read_u8()?;
    }

    for chunk in elements.iter_mut() {
        chunk.window_size = r.read_u32::<LittleEndian>()?;
    }

    for chunk in elements.iter_mut() {
        chunk.file_size = r.read_i64::<LittleEndian>()?;
    }

    let bytes_read = r.stream_position()? - start_pos;
    if bytes_read < size as u64 {
        r.seek(std::io::SeekFrom::Current(size as i64 - bytes_read as i64))?;
    }

    Ok(ChunkDataList { version, elements })
}
