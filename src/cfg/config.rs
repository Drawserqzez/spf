use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub app: App,
    pub account: Account,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_service_url: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expiration_time: DateTime<Utc>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            account_service_url: "https://accounts.spotify.com".to_string(),
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            expiration_time: DateTime::<Utc>::MIN_UTC
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    pub music_service_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_port: u32,
}

impl App {
    pub fn from_old(old: &App) -> Self {
        Self {
            music_service_url: old.music_service_url.to_owned(),
            client_id: old.client_id.to_owned(),
            client_secret: old.client_secret.to_owned(),
            redirect_port: old.redirect_port
        }
    }

    pub fn new(client_id: &str, client_secret: &str, redirect_port: u32) -> Self {
        Self {
            music_service_url: "https://api.spotify.com/v1".to_string(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            redirect_port,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new("", "", 1337)
    }
}

