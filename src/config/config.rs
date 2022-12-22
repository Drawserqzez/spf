use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub app: App,
    pub account: Account,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub music_service_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_port: u32,
}

impl Default for App {
    fn default() -> Self {
        App { 
            music_service_url: "https://api.spotify.com/v1".to_string(), 
            client_id: "".to_string(), 
            client_secret: "".to_string(), 
            redirect_port: 1337
        }
    }
}

