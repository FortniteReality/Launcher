use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::discord::errors::DiscordError;

pub struct DiscordRpcUtils {
    client: DiscordIpcClient,
    connected: bool,
    app_id: String
}

impl DiscordRpcUtils {
    pub fn new(app_id: &str) -> Self {
        let client = DiscordIpcClient::new(app_id).expect("Failed to create Discord IPC client");

        Self {
            client,
            connected: false,
            app_id: app_id.to_string()
        }
    }

    pub fn connect(&mut self) -> Result<(), DiscordError> {
        if self.connected {
            return Ok(());
        }

        self.client.connect()?;
        self.connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), DiscordError> {
        if !self.connected {
            return Ok(());
        }

        self.client.close()?;
        self.connected = false;
        Ok(())
    }

    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn set_activity(&mut self,
        state: Option<&str>,
        details: Option<&str>,
        large_image: Option<&str>,
        large_text: Option<&str>,
        small_image: Option<&str>,
        small_text: Option<&str>) -> Result<(), DiscordError> {
        
        if !self.connected {
            return Err(DiscordError::NotConnected);
        }

        let mut activity_builder = activity::Activity::new();

        if let Some(state) = state {
            activity_builder = activity_builder.state(state);
        }

        if let Some(details) = details {
            activity_builder = activity_builder.details(details);
        }

        if large_image.is_some() || small_image.is_some() {
            let mut assets = activity::Assets::new();

            if let Some(large_image) = large_image {
                assets = assets.large_image(large_image);
            }

            if let Some(large_text) = large_text {
                assets = assets.large_text(large_text);
            }

            if let Some(small_image) = small_image {
                assets = assets.small_image(small_image);
            }

            if let Some(small_text) = small_text {
                assets = assets.small_text(small_text);
            }

            activity_builder = activity_builder.assets(assets);
        }

        // Set timestamp to current time
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let timestamps = activity::Timestamps::new().start(timestamp);
        activity_builder = activity_builder.timestamps(timestamps);

        let activity = activity_builder;
        self.client.set_activity(activity)?;
        Ok(())
    }

    pub fn set_activity_with_buttons(&mut self,
        state: Option<&str>,
        details: Option<&str>,
        large_image: Option<&str>,
        large_text: Option<&str>,
        small_image: Option<&str>,
        small_text: Option<&str>,
        buttons: Vec<(&str, &str)>) -> Result<(), DiscordError> {

        if !self.connected {
            return Err(DiscordError::NotConnected);
        }

        let mut activity_builder = activity::Activity::new();

        if let Some(state) = state {
            activity_builder = activity_builder.state(state);
        }

        if let Some(details) = details {
            activity_builder = activity_builder.details(details);
        }

        if large_image.is_some() || small_image.is_some() {
            let mut assets = activity::Assets::new();

            if let Some(large_image) = large_image {
                assets = assets.large_image(large_image);
            }

            if let Some(large_text) = large_text {
                assets = assets.large_text(large_text);
            }

            if let Some(small_image) = small_image {
                assets = assets.small_image(small_image);
            }

            if let Some(small_text) = small_text {
                assets = assets.small_text(small_text);
            }

            activity_builder = activity_builder.assets(assets);
        }

        // Add buttons
        if !buttons.is_empty() {
            let button_vec: Vec<activity::Button> = buttons
                .into_iter()
                .map(|(label, url)| activity::Button::new(label, url))
                .collect();
            activity_builder = activity_builder.buttons(button_vec);
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let timestamps = activity::Timestamps::new().start(timestamp);
        activity_builder = activity_builder.timestamps(timestamps);

        let activity = activity_builder;
        self.client.set_activity(activity)?;
        Ok(())
    }

    pub fn clear_activity(&mut self) -> Result<(), DiscordError> {
        if !self.connected {
            return Err(DiscordError::NotConnected);
        }

        self.client.clear_activity()?;
        Ok(())
    }

    pub fn set_playing(&mut self, game_name: &str) -> Result<(), DiscordError> {
        if !self.connected {
            return Err(DiscordError::NotConnected);
        }

        self.set_activity_with_buttons(
            Some("discord.gg/Reality"),
            Some(&format!("Currently playing {}", game_name)),
            Some("fortnite"),
            Some("discord.gg/Reality"),
            None,
            None,
            vec![("Join Reality", "https://discord.gg/reality")],
        )
    }

    pub fn set_idle(&mut self) -> Result<(), DiscordError> {
        if !self.connected {
            return Err(DiscordError::NotConnected);
        }

        self.set_activity_with_buttons(
            Some("discord.gg/Reality"),
            Some("Idle in launcher"),
            Some("fortnite"),
            Some("discord.gg/Reality"),
            None,
            None,
            vec![("Join Reality", "https://discord.gg/reality")],
        )
    }

    pub fn reconnect(&mut self) -> Result<(), DiscordError> {
        if self.connected {
            let _ = self.disconnect()?;
        }
        self.connect()
    }
}

impl Drop for DiscordRpcUtils {
    fn drop(&mut self) {
        let _ = self.disconnect();
    }
}