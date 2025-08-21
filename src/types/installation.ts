export interface InstallLocation {
  path: string;
  label: string;
  free: string;
  total: string;
}

export interface InstalledObject {
    installation_location: string;
    namespace_id: string;
    item_id: string;
    artifact_id: string;
    app_version: string;
    app_name: string;
}

export interface ProgressUpdate {
    filename: string;
    downloaded_bytes: number;
    total_bytes: number;
    total_files: number;
}