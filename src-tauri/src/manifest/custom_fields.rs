use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

use crate::manifest::manifest_utils::read_fstring;

#[derive(Serialize, Debug)]
pub struct CustomFields {
    pub fields: HashMap<String, String>,
    pub version: u8,
}

pub fn read_custom_fields<R: Read + Seek>(r: &mut R) -> std::io::Result<CustomFields> {
    let start = r.stream_position()?;
    let size = r.read_u32::<LittleEndian>()?;
    let version = r.read_u8()?;
    let count = r.read_u32::<LittleEndian>()?;

    let mut keys = Vec::with_capacity(count as usize);
    for _ in 0..count {
        keys.push(read_fstring(r)?);
    }

    let mut values = Vec::with_capacity(count as usize);
    for _ in 0..count {
        values.push(read_fstring(r)?);
    }
    let map = keys.into_iter().zip(values.into_iter()).collect();

    // skip padding
    let read_bytes = r.stream_position()? - start;
    if read_bytes < size as u64 {
        r.seek(SeekFrom::Current(size as i64 - read_bytes as i64))?;
    }

    Ok(CustomFields {
        fields: map,
        version,
    })
}
