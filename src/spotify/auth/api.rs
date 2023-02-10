use std::collections::HashMap;

use reqwest::blocking::Client;
use chrono::Utc;
use serde::Deserialize;
use oauth2::{
    AuthorizationCode, AuthUrl, 
    ClientId, ClientSecret, CsrfToken, 
    EmptyExtraTokenFields, RedirectUrl, 
    Scope, StandardTokenResponse, 
    TokenResponse, TokenUrl
};
use oauth2::basic::{BasicClient, BasicTokenType};

use crate::cfg::models::App;

fn auth(settings: &App) -> Result<Token, Box<dyn std::error::Error>> {
    let client = BasicClient::new(
        ClientId::new(settings.client_id.to_string()),
        Some(ClientSecret::new(settings.client_secret.to_string())),
        AuthUrl::new("https://accounts.spotify.com/authorize".to_string())?,
        Some(TokenUrl::new("https://accounts.spotify.com/api/token".to_string())?)
    ).set_redirect_uri(RedirectUrl::new(format!("http://localhost:{}", settings.redirect_port))?);

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-read-private".to_string()))
        .add_scope(Scope::new("user-read-email".to_string()))
        .url();

    println!("Please open in browser in order to authorize the app: {}", auth_url);

    // TODO: Listen for response on redirect url -> maybe use tiny-http? 

    let token_result: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> = client
        .exchange_code(AuthorizationCode::new("".to_string()))
        .request(oauth2::reqwest::http_client)?;

    let now = Utc::now().timestamp();

    let time = match token_result.expires_in() {
        Some(exp) => exp.as_secs(),
        None => 0
    };

    let expiration_time = now + time as i64;

    let token = Token {
        access_token: token_result.access_token().secret().to_owned(),
        expiration_time,
        refresh_token: match token_result.refresh_token() {
            Some(refresh) => Some(refresh.secret().to_owned()),
            None => None
        },
    };

    Ok(token)
}

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
        refresh_token: Some("helloworld".to_string()),
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
    pub refresh_token: Option<String>,
    pub expiration_time: i64
}

