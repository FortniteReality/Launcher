use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

pub fn read_fstring<R: Read + Seek>(r: &mut R) -> std::io::Result<String> {
    let len = r.read_i32::<LittleEndian>()?;
    if len == 0 {
        Ok(String::new())
    } else if len < 0 {
        let mut buf = vec![0u8; (-len * 2 - 2) as usize];
        r.read_exact(&mut buf)?;
        r.seek(SeekFrom::Current(2))?;
        Ok(String::from_utf16_lossy(
            &buf.chunks(2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect::<Vec<_>>(),
        ))
    } else {
        let mut buf = vec![0u8; (len - 1) as usize];
        r.read_exact(&mut buf)?;
        r.seek(SeekFrom::Current(1))?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}
