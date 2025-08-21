use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, SeekFrom};

use crate::manifest::manifest_utils::read_fstring;

#[derive(Clone, Serialize, Debug)]
pub struct ChunkPart {
    pub guid: [u32; 4],
    pub offset: u32,
    pub size: u32,
    pub file_offset: u32,
}

#[derive(Clone, Serialize, Debug)]
pub struct FileManifest {
    pub filename: String,
    pub symlink_target: String,
    pub hash: Vec<u8>,
    pub flags: u8,
    pub install_tags: Vec<String>,
    pub chunk_parts: Vec<ChunkPart>,
    pub file_size: u64,
    pub hash_md5: Option<Vec<u8>>,
    pub mime_type: Option<String>,
    pub hash_sha256: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Debug)]
pub struct FileManifestList {
    pub version: u8,
    pub elements: Vec<FileManifest>,
}

pub fn read_file_manifest_list<R: Read + Seek>(r: &mut R) -> std::io::Result<FileManifestList> {
    let start = r.stream_position()?;
    let size = r.read_u32::<LittleEndian>()?;
    let version = r.read_u8()?;
    let count = r.read_u32::<LittleEndian>()?;

    let mut elements: Vec<FileManifest> = (0..count)
        .map(|_| FileManifest {
            filename: String::new(),
            symlink_target: String::new(),
            hash: vec![0; 20],
            flags: 0,
            install_tags: vec![],
            chunk_parts: vec![],
            file_size: 0,
            hash_md5: None,
            mime_type: None,
            hash_sha256: None,
        })
        .collect();

    for fm in &mut elements {
        fm.filename = read_fstring(r)?;
    }

    for fm in &mut elements {
        fm.symlink_target = read_fstring(r)?;
    }

    for fm in &mut elements {
        let mut buf = vec![0; 20];
        r.read_exact(&mut buf)?;
        fm.hash = buf;
    }

    for fm in &mut elements {
        fm.flags = r.read_u8()?;
    }

    for fm in &mut elements {
        let tag_count = r.read_u32::<LittleEndian>()?;
        for _ in 0..tag_count {
            fm.install_tags.push(read_fstring(r)?);
        }
    }

    for fm in &mut elements {
        let part_count = r.read_u32::<LittleEndian>()?;
        let mut offset_tracker = 0u32;

        for _ in 0..part_count {
            let _declared_size = r.read_u32::<LittleEndian>()?;
            let mut guid = [0u32; 4];
            for i in 0..4 {
                guid[i] = r.read_u32::<LittleEndian>()?;
            }
            let offset = r.read_u32::<LittleEndian>()?;
            let size = r.read_u32::<LittleEndian>()?;

            fm.chunk_parts.push(ChunkPart {
                guid,
                offset,
                size,
                file_offset: offset_tracker,
            });

            offset_tracker += size;
        }
    }

    if version >= 1 {
        for fm in &mut elements {
            let has_md5 = r.read_u32::<LittleEndian>()?;
            if has_md5 != 0 {
                let mut md5 = vec![0; 16];
                r.read_exact(&mut md5)?;
                fm.hash_md5 = Some(md5);
            }
        }

        for fm in &mut elements {
            fm.mime_type = Some(read_fstring(r)?);
        }
    }

    if version >= 2 {
        for fm in &mut elements {
            let mut sha256 = vec![0; 32];
            r.read_exact(&mut sha256)?;
            fm.hash_sha256 = Some(sha256);
        }
    }

    for fm in &mut elements {
        fm.file_size = fm.chunk_parts.iter().map(|c| c.size as u64).sum();
    }

    let read_bytes = r.stream_position()? - start;
    if read_bytes < size as u64 {
        r.seek(SeekFrom::Current(size as i64 - read_bytes as i64))?;
    }

    Ok(FileManifestList { version, elements })
}
