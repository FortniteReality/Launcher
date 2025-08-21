use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek};

pub use crate::manifest::chunk_data::ChunkDataList;
pub use crate::manifest::custom_fields::CustomFields;
pub use crate::manifest::file_manifest::FileManifestList;

use crate::manifest::manifest_utils::read_fstring;

#[derive(Serialize)]
pub struct ManifestMeta {
    app_id: u32,
    app_name: String,
    build_version: String,
    launch_exe: String,
    launch_command: String,
    prereq_ids: Vec<String>,
    prereq_name: String,
    prereq_path: String,
    prereq_args: String,
    build_id: String,
    uninstall_action_path: String,
    uninstall_action_args: String,
}

#[derive(Serialize)]
pub struct ParsedManifest {
    pub version: u32,
    pub compressed: bool,
    pub meta: ManifestMeta,
    pub chunk_data_list: ChunkDataList,
    pub file_manifest_list: FileManifestList,
    pub custom_fields: CustomFields,
}

pub fn read_manifest_meta<R: Read + Seek>(r: &mut R) -> std::io::Result<ManifestMeta> {
    let _meta_size = r.read_u32::<LittleEndian>()?;
    let data_version = r.read_u8()?;
    let _feature_level = r.read_u32::<LittleEndian>()?;
    let _is_file_data = r.read_u8()? != 0;
    let app_id = r.read_u32::<LittleEndian>()?;
    let app_name = read_fstring(r)?;
    let build_version = read_fstring(r)?;
    let launch_exe = read_fstring(r)?;
    let launch_command = read_fstring(r)?;
    let prereq_count = r.read_u32::<LittleEndian>()?;
    let mut prereq_ids = Vec::with_capacity(prereq_count as usize);
    for _ in 0..prereq_count {
        prereq_ids.push(read_fstring(r)?);
    }
    let prereq_name = read_fstring(r)?;
    let prereq_path = read_fstring(r)?;
    let prereq_args = read_fstring(r)?;

    let build_id = if data_version >= 1 {
        read_fstring(r)?
    } else {
        String::new()
    };

    let uninstall_action_path = if data_version >= 2 {
        read_fstring(r)?
    } else {
        String::new()
    };

    let uninstall_action_args = if data_version >= 2 {
        read_fstring(r)?
    } else {
        String::new()
    };

    Ok(ManifestMeta {
        app_id,
        app_name,
        build_version,
        launch_exe,
        launch_command,
        prereq_ids,
        prereq_name,
        prereq_path,
        prereq_args,
        build_id,
        uninstall_action_path,
        uninstall_action_args,
    })
}
