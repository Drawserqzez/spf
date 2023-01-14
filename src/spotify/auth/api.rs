use std::collections::HashMap;

use reqwest::blocking::Client;
use chrono::Utc;
use serde::Deserialize;

pub fn authenticate() -> Result<Token, AuthApiError> {
    let client = Client::new();
    let mut params = HashMap::new();

    params.insert("grant_type", "client_credentials");

    let token_data: AuthResponse = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth("", Some("")) //TODO: Get client data in here
        .form(&params)
        .send()
        .unwrap() // TODO: Remove this and handle if we actually aren't auth'd
        .json::<AuthResponse>()
        .unwrap(); // TODO: Handle potential paring errors

    let now = Utc::now().timestamp();

    let token = Token {
        access_token: token_data.access_token,
        expiration_time: now + token_data.expires_in,
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

#[derive(Debug)]
pub enum AuthApiError {
    Unauthorized,
}
