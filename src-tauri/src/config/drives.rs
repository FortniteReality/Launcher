use serde::Serialize;
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Serialize, Debug)]
pub struct InstallLocation {
    pub path: String,
    pub label: String,
    pub free: String,
    pub total: String
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    if bytes >= TB {
        format!("{:.1} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{:.1} B", bytes)
    }
}

pub fn get_install_locations() -> Vec<InstallLocation> {
    let mut sys = System::new_all();
    sys.refresh_disks_list();

    let mut locations = Vec::new();

    let system_disk_name = sys.disks().first().map(|d| d.name());

    for disk in sys.disks() {
        if disk.is_removable() {
            continue;
        }

        let mount_point = disk.mount_point();
        let drive_letter = mount_point
            .to_string_lossy()
            .chars()
            .next()
            .unwrap_or('?');

        let custom_path = format!(r"{}:\Program Files\Reality", drive_letter);

        let is_recommended = Some(disk.name()) == system_disk_name;
        let label = if is_recommended {
            format!("{}: Drive (Recommended)", drive_letter)
        } else {
            format!("{}: Drive", drive_letter)
        };

        let location = InstallLocation {
            path: custom_path,
            label,
            free: format_bytes(disk.available_space()),
            total: format_bytes(disk.total_space())
        };

        locations.push(location);
    }

    locations
}