use crate::auth::{login_user_refresh, AccountInfo};

pub mod drives;
pub mod errors;
pub mod installed;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub use errors::ConfigError;

fn get_game_user_config_path() -> io::Result<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "LOCALAPPDATA environment variable not found",
        )
    })?;

    Ok(PathBuf::from(local_app_data)
        .join("RealityLauncher")
        .join("Saved")
        .join("Config")
        .join("Windows")
        .join("GameUserSettings.ini"))
}

fn parse_ini_file(path: &PathBuf) -> io::Result<HashMap<String, HashMap<String, String>>> {
    let content = fs::read_to_string(path)?;
    let mut ini_map = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            ini_map
                .entry(current_section.clone())
                .or_insert_with(HashMap::new);
        } else if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();

            if !current_section.is_empty() {
                ini_map
                    .entry(current_section.clone())
                    .or_insert_with(HashMap::new)
                    .insert(key, value);
            }
        }
    }

    Ok(ini_map)
}

fn write_ini_file(
    path: &PathBuf,
    ini_content: &HashMap<String, HashMap<String, String>>,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;

    for (section_name, section_content) in ini_content {
        writeln!(file, "[{}]", section_name)?;
        for (key, value) in section_content {
            writeln!(file, "{}={}", key, value)?;
        }
        writeln!(file)? // Empty line between sections
    }

    Ok(())
}

pub async fn save_user_login(
    enable_remember_me: bool,
    refresh_token: String,
) -> Result<(), ConfigError> {
    let config_path = get_game_user_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Load existing INI or create a new one
    let mut ini_content = if config_path.exists() {
        parse_ini_file(&config_path)?
    } else {
        HashMap::new()
    };

    // Update the RememberMe section
    ini_content
        .entry("RememberMe".to_string())
        .or_insert_with(HashMap::new)
        .insert("Enabled".to_string(), enable_remember_me.to_string());

    if enable_remember_me {
        ini_content
            .entry("RememberMe".to_string())
            .or_insert_with(HashMap::new)
            .insert("Data".to_string(), refresh_token);
    }

    write_ini_file(&config_path, &ini_content)?;

    Ok(())
}

pub async fn fetch_saved_user_login() -> Result<AccountInfo, ConfigError> {
    let config_path = get_game_user_config_path()?;

    if !config_path.exists() {
        return Err(ConfigError::MissingConfigFile);
    }

    let ini_content = parse_ini_file(&config_path)?;

    if ini_content.contains_key("RememberMe")
        && ini_content["RememberMe"].contains_key("Enabled")
        && ini_content["RememberMe"]["Enabled"].eq_ignore_ascii_case("true")
        && ini_content["RememberMe"].contains_key("Data")
    {
        let account_info: AccountInfo =
            login_user_refresh(&ini_content["RememberMe"]["Data"]).await?;
        save_user_login(true, account_info.refresh_token.clone()).await?;
        return Ok(account_info);
    } else {
        return Err(ConfigError::MissingConfigSection);
    }
}

pub async fn get_remember_me_value() -> Result<bool, ConfigError> {
    let config_path = get_game_user_config_path()?;

    if !config_path.exists() {
        return Err(ConfigError::MissingConfigFile);
    }

    let ini_content = parse_ini_file(&config_path)?;

    if ini_content.contains_key("RememberMe")
        && ini_content["RememberMe"].contains_key("Enabled")
    {
        let enabled = ini_content["RememberMe"]["Enabled"]
            .eq_ignore_ascii_case("true");
        return Ok(enabled);
    }

    Ok(false)
}