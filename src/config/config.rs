use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    app: App,
    account: Account,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Account {
    account_service_url: String,
    access_token: String,
    refresh_token: String,
    expiration_time: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct App {
    music_service_url: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

