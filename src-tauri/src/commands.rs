use base64::prelude::BASE64_STANDARD;
use tokio::sync::mpsc;
use win_msgbox::Okay;

use crate::auth::{generate_exchange, login_client, login_user, AccountInfo, AuthError, Services};
use crate::cache::{get_account_info, get_client_token};
use crate::config::drives::{get_install_locations, InstallLocation};
use crate::config::installed::{
    add_or_update_object, get_object_by_artifact_id, update_object_by_artifact_id, InstalledObject,
};
use crate::config::{fetch_saved_user_login, save_user_login, ConfigError};

use crate::discord::errors::DiscordError;
use crate::friends::errors::FriendError;
use crate::friends::online::{accept_friend, decline_friend, get_blocked_users, get_display_name_by_account_id, get_friends, get_incoming_friends, get_outgoing_friends};

use crate::game::errors::GameInfoError;
use crate::game::fetch_current_game_data;
use crate::game::responses::GameInfo;

use crate::manifest::downloader::downloader::download_game;
use crate::manifest::downloader::errors::DownloadError;
use crate::manifest::downloader::{get_second_latest_manifest_b64, mark_game_as_deleted};
use crate::manifest::downloader::progress_update::ProgressUpdate;
use crate::manifest::downloader::verifier::verify_and_repair_parallel;
use crate::manifest::{fetch_current_manifest_as_b64, mark_current_manifest_as_complete, parse_manifest, ManifestError, ParsedManifest};

use crate::launcher::errors::LaunchError;
use crate::launcher::game_launcher::{GameLauncher, LaunchConfig, ProcessUtils};

use base64::Engine;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tauri::State;
use tokio::fs;

use crate::discord::discord_rpc_utils::DiscordRpcUtils;

#[derive(Default)]
pub struct DownloadControl {
    pub cancelled: AtomicBool,
    pub progress: Arc<tokio::sync::Mutex<Vec<ProgressUpdate>>>,
}

// Add global state to track current operations
static CURRENT_DOWNLOAD_CONTROL: std::sync::OnceLock<Arc<tokio::sync::RwLock<Option<Arc<DownloadControl>>>>> = std::sync::OnceLock::new();
static CURRENT_UNINSTALL_CONTROL: std::sync::OnceLock<Arc<tokio::sync::RwLock<Option<Arc<DownloadControl>>>>> = std::sync::OnceLock::new();

static DISCORD_RPC: std::sync::OnceLock<Arc<tokio::sync::RwLock<Option<DiscordRpcUtils>>>> = std::sync::OnceLock::new();

fn get_global_control() -> &'static Arc<tokio::sync::RwLock<Option<Arc<DownloadControl>>>> {
    CURRENT_DOWNLOAD_CONTROL.get_or_init(|| Arc::new(tokio::sync::RwLock::new(None)))
}

fn get_global_uninstall_control() -> &'static Arc<tokio::sync::RwLock<Option<Arc<DownloadControl>>>> {
    CURRENT_UNINSTALL_CONTROL.get_or_init(|| Arc::new(tokio::sync::RwLock::new(None)))
}

fn get_global_discord_rpc() -> &'static Arc<tokio::sync::RwLock<Option<DiscordRpcUtils>>> {
    DISCORD_RPC.get_or_init(|| Arc::new(tokio::sync::RwLock::new(None)))
}

/// Initialize discord RPC
#[tauri::command]
pub async fn initialize_discord_rpc() -> Result<(), DiscordError> {
    let mut rpc = DiscordRpcUtils::new("1398820832834224209");
    rpc.connect()?;

    let global_rpc = get_global_discord_rpc();
    let mut guard = global_rpc.write().await;
    *guard = Some(rpc);

    Ok(())
}

/// Disconnect discord RPC
#[tauri::command]
pub async fn disconnect_discord_rpc() -> Result<(), DiscordError> {
    let global_rpc = get_global_discord_rpc();
    let mut guard = global_rpc.write().await;

    if let Some(ref mut rpc) = guard.as_mut() {
        rpc.disconnect()?;
    }

    *guard = None;
    Ok(())
}

/// Set idle activity in discord RPC
#[tauri::command]
pub async fn set_idle_activity() -> Result<(), DiscordError> {
    let global_rpc = get_global_discord_rpc();
    let mut guard = global_rpc.write().await;

    match guard.as_mut() {
        Some(rpc) => rpc.set_idle(),
        None => Err(DiscordError::NotInitialized)
    }
}

/// Set playing activity in discord RPC
#[tauri::command]
pub async fn set_playing_activity() -> Result<(), DiscordError> {
    let global_rpc = get_global_discord_rpc();
    let mut guard = global_rpc.write().await;

    match guard.as_mut() {
        Some(rpc) => rpc.set_playing("Reality"),
        None => Err(DiscordError::NotInitialized),
    }
}

/// Generate a user access code for the launcher client with password grant type.
/// NOTE: We do not want any public facing client to have access to password grant type, so we do need to change this to a public launcher API with a captcha ASAP.
#[tauri::command]
pub async fn loginu(email: String, password: String) -> Result<AccountInfo, AuthError> {
    login_user(email, password).await
}

/// Generate a client credentials access code for the launcher client.
#[tauri::command]
pub async fn loginc() -> Result<String, AuthError> {
    login_client().await
}

/// Caches user login data
#[tauri::command]
pub async fn saveu(enable_remember_me: bool, refresh_token: String) -> Result<(), ConfigError> {
    save_user_login(enable_remember_me, refresh_token).await
}

/// Gets the cached user login data
#[tauri::command]
pub async fn fetchsu() -> Result<AccountInfo, ConfigError> {
    fetch_saved_user_login().await
}

/// Gets or initializes the cached user login data, but in memory as well
#[tauri::command]
pub async fn fetchcu() -> Result<&'static AccountInfo, ConfigError> {
    get_account_info().await
}

/// Gets or initializes the cached client credentials data, but in memory as well
#[tauri::command]
pub async fn fetchcc() -> Result<&'static String, AuthError> {
    get_client_token().await
}

/// Gets the progress of a download/verification/uninstall
#[tauri::command]
pub async fn get_progress(
    _control: State<'_, Arc<DownloadControl>>, // Keep for compatibility but don't use
) -> Result<Vec<ProgressUpdate>, DownloadError> {
    // First check if there's an active download/verification
    let global_control = get_global_control().read().await;
    if let Some(ref control) = *global_control {
        return Ok(control.progress.lock().await.clone());
    }
    drop(global_control);
    
    // Then check if there's an active uninstall
    let global_uninstall_control = get_global_uninstall_control().read().await;
    if let Some(ref control) = *global_uninstall_control {
        return Ok(control.progress.lock().await.clone());
    }
    
    // No active operations
    Ok(Vec::new())
}

/// Cancels a download or verification
#[tauri::command]
pub async fn cancel_download(install_dir: String, _control: State<'_, Arc<DownloadControl>>) -> Result<(), ManifestError> {
    // Cancel the current download
    let global_control = get_global_control().read().await;
    if let Some(ref control) = *global_control {
        control.cancelled.store(true, Ordering::Relaxed);
        eprintln!("Download cancellation requested");
    }
    drop(global_control);
    
    // Wait a bit for the download to actually cancel
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Now clean up with a separate control for uninstall
    let uninstall_control = Arc::new(DownloadControl::default());
    start_uninstall_internal(Some(install_dir), uninstall_control).await.map_err(|_| ManifestError::UnexpectedError)?;
    uninstall_complete().await
}

/// Cancels an uninstall operation
#[tauri::command]
pub async fn cancel_uninstall(_control: State<'_, Arc<DownloadControl>>) -> Result<(), DownloadError> {
    let global_uninstall_control = get_global_uninstall_control().read().await;
    if let Some(ref control) = *global_uninstall_control {
        control.cancelled.store(true, Ordering::Relaxed);
        eprintln!("Uninstall cancellation requested");
        Ok(())
    } else {
        Err(DownloadError::UnexpectedError)
    }
}

/// Resumes a download or verification
#[tauri::command]
pub fn resume_download(_control: State<'_, Arc<DownloadControl>>) {
    // This doesn't really make sense with the current architecture
    eprintln!("Resume download called - this operation is not supported");
}

/// This handles the game installation internally with a control to return progress updates
pub async fn start_download_internal(
    manifest_b64: String,
    old_manifest_b64: Option<String>,
    bucket: String,
    install_dir: String,
    control: Arc<DownloadControl>,
) -> Result<(), DownloadError> {
    // Set this as the current download control
    *get_global_control().write().await = Some(control.clone());
    
    control.cancelled.store(false, Ordering::Relaxed);
    control.progress.lock().await.clear();

    let path = PathBuf::from(&install_dir);
    let (tx, mut rx) = mpsc::channel(128);

    let progress_handle = control.progress.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            progress_handle.lock().await.push(progress);
        }
    });

    let mut buf = Vec::<u8>::new();
    BASE64_STANDARD.decode_vec(manifest_b64, &mut buf)?;
    let parsed_manifest: ParsedManifest = parse_manifest(buf).await?;

    let old_parsed_manifest: Option<ParsedManifest> = match old_manifest_b64 {
        Some(b64) => {
            buf = Vec::<u8>::new();
            BASE64_STANDARD.decode_vec(b64, &mut buf)?;
            Some(parse_manifest(buf).await?)
        }
        None => None,
    };

    let result = download_game(
        parsed_manifest,
        bucket,
        path,
        tx,
        control.clone(),
        old_parsed_manifest,
    )
    .await;
    
    // Clear the global control when done
    *get_global_control().write().await = None;
    
    result
}

/// Download a game
#[tauri::command]
pub async fn start_download(
    install_dir: String
) -> Result<(), DownloadError> {
    // Create a new control specifically for this download
    let download_control = Arc::new(DownloadControl::default());
    
    match get_second_latest_manifest_b64().await {
        Ok(old_manifest_b64) => {
            start_download_internal(
                fetch_current_manifest_as_b64().await?,
                Some(old_manifest_b64),
                "reality-manifest".to_string(),
                install_dir,
                download_control,
            )
            .await
        }
        Err(_) => {
            start_download_internal(
                fetch_current_manifest_as_b64().await?,
                None,
                "reality-manifest".to_string(),
                install_dir,
                download_control,
            )
            .await
        }
    }
}

/// Saves the downloaded data to the disk and marks the download as complete
/// This is used when the download is complete and we want to finalize the installation
#[tauri::command]
pub async fn download_complete(
    install_dir: String
) -> Result<(), ManifestError> {
    mark_current_manifest_as_complete(&install_dir).await
}

/// Sends a Win32 message box with an OK button
#[tauri::command]
pub fn message_box_okay(message: String) {
    let _ = win_msgbox::show::<Okay>(message.as_str());
}

/// Marks the game as uninstalled by removing everything
#[tauri::command]
pub async fn uninstall_complete() -> Result<(), ManifestError> {
    mark_game_as_deleted().await
}

/// This handles the game verification internally with a control to return progress updates
pub async fn start_verify_internal(
    manifest_b64: String,
    bucket: String,
    install_dir: String,
    control: Arc<DownloadControl>,
) -> Result<(), DownloadError> {
    // Set this as the current operation control
    *get_global_control().write().await = Some(control.clone());
    
    control.cancelled.store(false, Ordering::Relaxed);
    control.progress.lock().await.clear();

    let path = PathBuf::from(&install_dir);
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let progress_handle = control.progress.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            progress_handle.lock().await.push(progress);
        }
    });

    let mut buf = Vec::<u8>::new();
    BASE64_STANDARD.decode_vec(manifest_b64, &mut buf)?;
    let parsed_manifest: ParsedManifest = parse_manifest(buf).await?;

    let result = verify_and_repair_parallel(parsed_manifest, bucket, path, tx, control).await;
    
    // Clear the global control when done
    *get_global_control().write().await = None;
    
    result
}

/// Verify the game files
#[tauri::command]
pub async fn start_verify() -> Result<(), DownloadError> {
    // Create a new control specifically for this verification
    let verify_control = Arc::new(DownloadControl::default());
    
    start_verify_internal(
        fetch_current_manifest_as_b64().await?, 
        "reality-manifest".to_string(), 
        get_object_by_artifact_id(Services::CATALOG_ID).await.map_err(|_| DownloadError::UnexpectedError)?.installation_location, 
        verify_control
    ).await
}

/// Internal uninstall function that doesn't interfere with download cancellation
async fn start_uninstall_internal(
    install_dir: Option<String>,
    control: Arc<DownloadControl>,
) -> Result<(), DownloadError> {
    // Set this as the current uninstall control
    *get_global_uninstall_control().write().await = Some(control.clone());
    
    control.cancelled.store(false, Ordering::Relaxed);
    control.progress.lock().await.clear();

    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let progress_handle = control.progress.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            progress_handle.lock().await.push(progress);
        }
    });

    let path = match install_dir.is_some() {
        true => install_dir.unwrap(),
        false => {
            get_object_by_artifact_id(Services::CATALOG_ID)
            .await
            .map_err(|_| DownloadError::UnexpectedError)?
            .installation_location
        }
    };
    
    let install_path = PathBuf::from(&path);
    
    // Collect all files first to get total count and sizes
    let mut files_to_delete = Vec::new();
    let mut total_size = 0u64;
    
    if install_path.exists() {
        collect_files_recursive(&install_path, &mut files_to_delete, &mut total_size).await?;
    }
    
    let total_files = files_to_delete.len();
    let total_bytes = total_size;
    let mut deleted_bytes = 0u64;
    
    // If no files to delete, send completion immediately
    if total_files == 0 {
        tx.send(ProgressUpdate {
            filename: "No files to delete".to_string(),
            downloaded_bytes: 0,
            total_bytes: 0,
            total_files: 0,
        }).await.map_err(|_| DownloadError::UnexpectedError)?;
        
        tx.send(ProgressUpdate {
            filename: "Uninstall complete".to_string(),
            downloaded_bytes: 0,
            total_bytes: 0,
            total_files: 0,
        }).await.map_err(|_| DownloadError::UnexpectedError)?;
        
        // Give time for progress update to be processed
        tokio::time::sleep(Duration::from_millis(200)).await;
        *get_global_uninstall_control().write().await = None;
        return Ok(());
    }

    // Delete files sequentially with progress updates
    for (file_index, (file_path, file_size)) in files_to_delete.iter().enumerate() {
        // Check for cancellation
        if control.cancelled.load(Ordering::Relaxed) {
            *get_global_uninstall_control().write().await = None;
            return Err(DownloadError::Cancelled);
        }

        let filename = file_path
            .strip_prefix(&install_path)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        // Delete the file
        match fs::remove_file(file_path).await {
            Ok(()) => {
                deleted_bytes += file_size;
            }
            Err(e) => {
                eprintln!("Failed to delete file {}: {}", filename, e);
                // Still count it as processed
                deleted_bytes += file_size;
            }
        }

        // Send progress update after each file
        tx.send(ProgressUpdate {
            filename: filename.clone(),
            downloaded_bytes: deleted_bytes,
            total_bytes,
            total_files,
        }).await.map_err(|_| DownloadError::UnexpectedError)?;

        // Small delay to prevent overwhelming the progress channel
        if file_index % 10 == 0 {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
    
    // Remove empty directories
    if let Err(e) = remove_empty_dirs_recursive(&install_path).await {
        eprintln!("Warning: Failed to remove some empty directories: {}", e);
        // Don't fail the entire operation for this
    }
    
    // Ensure we're at 100% and send final completion signal
    deleted_bytes = std::cmp::max(deleted_bytes, total_bytes);
    
    tx.send(ProgressUpdate {
        filename: "Uninstall complete".to_string(),
        downloaded_bytes: deleted_bytes,
        total_bytes,
        total_files,
    }).await.map_err(|_| DownloadError::UnexpectedError)?;
    
    // Give time for the final progress update to be processed
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Clear the global uninstall control when done
    *get_global_uninstall_control().write().await = None;
    
    Ok(())
}

/// Uninstall the game by deleting all files
#[tauri::command]
pub async fn start_uninstall(install_dir: Option<String>) -> Result<(), DownloadError> {
    // Create a separate control for uninstall so it doesn't interfere with other operations
    let uninstall_control = Arc::new(DownloadControl::default());
    start_uninstall_internal(install_dir, uninstall_control).await
}

fn collect_files_recursive<'a>(
    dir: &'a PathBuf,
    files: &'a mut Vec<(PathBuf, u64)>,
    total_size: &'a mut u64,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DownloadError>> + Send + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_file() {
                let size = metadata.len();
                *total_size += size;
                files.push((path, size));
            } else if metadata.is_dir() {
                collect_files_recursive(&path, files, total_size).await?;
            }
        }
        
        Ok(())
    })
}

fn remove_empty_dirs_recursive<'a>(
    dir: &'a PathBuf,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DownloadError>> + Send + 'a>> {
    Box::pin(async move {
        if !dir.exists() {
            return Ok(());
        }
        
        let mut entries = fs::read_dir(dir).await?;
        let mut subdirs = Vec::new();
        let mut has_files = false;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_dir() {
                subdirs.push(path);
            } else {
                has_files = true;
            }
        }
        
        // Recursively remove subdirectories first
        for subdir in subdirs {
            remove_empty_dirs_recursive(&subdir).await?;
        }
        
        // If directory is now empty, remove it
        if !has_files {
            if let Ok(mut entries) = fs::read_dir(dir).await {
                if entries.next_entry().await?.is_none() {
                    if let Err(e) = fs::remove_dir(dir).await {
                        eprintln!("Failed to remove directory {:?}: {}", dir, e);
                    }
                }
            }
        }
        
        Ok(())
    })
}

/// Returns a cached installed game via the artifact id
#[tauri::command]
pub async fn fetch_installed_object_by_artifact_id(
    artifact_id: String,
) -> Result<InstalledObject, ConfigError> {
    get_object_by_artifact_id(artifact_id.as_str()).await
}

/// Updates the information of a cached installed game via the artifact id
#[tauri::command]
pub async fn update_installed_object_by_artifact_id(
    object: InstalledObject,
) -> Result<(), ConfigError> {
    update_object_by_artifact_id(object).await
}

/// Caches an installed game so we know which games are installed and at what version
#[tauri::command]
pub async fn push_installed_object(object: InstalledObject) -> Result<(), ConfigError> {
    add_or_update_object(object).await
}

/// Get all drives on the computer
#[tauri::command]
pub fn get_drives() -> Vec<InstallLocation> {
    get_install_locations()
}

/// Launches Fortnite with the cached download path and account information (generating an exchange code)
#[tauri::command]
pub async fn launch_game() -> Result<(), LaunchError> {
    // Create launch configuration
    let config = LaunchConfig {
        game_path: PathBuf::from(get_object_by_artifact_id(Services::CATALOG_ID).await?.installation_location),
        launch_args: vec![
            "-AUTH_LOGIN=unused".to_string(),
            format!("-AUTH_PASSWORD={}", generate_exchange().await?),
            "-AUTH_TYPE=exchangecode".to_string()
        ],
        initialization_delay: Duration::from_millis(500)
    };

    // Create the launcher given the config then launch the game
    let mut launcher = GameLauncher::new(config)?;
    launcher.launch().await?;

    Ok(())
}

/// Checks if Fortnite is open
#[tauri::command]
pub async fn get_is_playing() -> Result<bool, LaunchError> {
    let is_running: bool = ProcessUtils::is_fortnite_running();
    if !is_running {
        ProcessUtils::kill_game_processes()?;
    }
    Ok(is_running)
}

/// Fetches the user's friends list
#[tauri::command]
pub async fn fetch_friends_list() -> Result<Vec<String>, FriendError> {
    let friends: Vec<String> = get_friends().await?;
    Ok(friends)
}

/// Fetches the user's incoming friends list
#[tauri::command]
pub async fn fetch_incoming_friends_list() -> Result<Vec<String>, FriendError> {
    let friends: Vec<String> = get_incoming_friends().await?;
    Ok(friends)
}

/// Fetches the user's outgoing friends list
#[tauri::command]
pub async fn fetch_outgoing_friends_list() -> Result<Vec<String>, FriendError> {
    let friends: Vec<String> = get_outgoing_friends().await?;
    Ok(friends)
}

/// Fetches the user's blocklist
#[tauri::command]
pub async fn fetch_blocklist() -> Result<Vec<String>, FriendError> {
    let users: Vec<String> = get_blocked_users().await?;
    Ok(users)
}

/// Fetches a user's display name by their account id
#[tauri::command]
pub async fn fetch_display_name_by_account_id(account_id: String) -> Result<String, FriendError> {
    let display_name: String = get_display_name_by_account_id(account_id).await?;
    Ok(display_name)
}

/// Accept or send a friend request by friend account id
#[tauri::command]
pub async fn accept_friend_request(friend_account_id: String) -> Result<(), FriendError> {
    accept_friend(friend_account_id).await?;
    Ok(())
}

/// Deny a friend request by friend account id
#[tauri::command]
pub async fn decline_friend_request(friend_account_id: String) -> Result<(), FriendError> {
    decline_friend(friend_account_id).await?;
    Ok(())
}

/// Gets the game information from the catalog API and current manifest
#[tauri::command]
pub async fn get_game_information() -> Result<GameInfo, GameInfoError> {
    fetch_current_game_data().await
}