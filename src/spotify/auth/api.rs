use std::collections::HashMap;

use reqwest::blocking::Client;
use chrono::Utc;
use serde::Deserialize;

use crate::cfg::models::App;

pub fn authenticate(settings: &App) -> Result<Token, reqwest::Error> {
    let client = Client::new();
    let mut params = HashMap::new();

    params.insert("grant_type", "client_credentials");
    params.insert("scope", "user-read-private user-read-email");

    // TODO: We need user to authorize in browser first -> use hyper for callback
    let token_data: AuthResponse = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(&settings.client_id, Some(&settings.client_secret))
        .form(&params)
        .send()?
        .json::<AuthResponse>()?; // TODO: Handle potential parsing errors

    let now = Utc::now().timestamp();

    let token = Token {
        access_token: token_data.access_token,
        expiration_time: now + token_data.expires_in + 10,
    };

    Ok(token)
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expiration_time: i64
}

