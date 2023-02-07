use chrono::Utc;
use std::env;
use crate::spotify::auth::api::authenticate;
use crate::spotify::error::SpotifyError;

use super::api::Token;

const SPF_EXPIRATION_TIME: &str = "SPF_EXPIRATION_TIME";
const SPF_ACCESS_TOKEN: &str = "SPF_ACCESS_TOKEN";

fn is_authenticated() -> bool {
    let now = Utc::now();

    let exp_time = match env::var(SPF_EXPIRATION_TIME) {
        Ok(t) => match t.parse::<i64>() {
            Ok(u) => u,
            Err(_e) => 0
        },
        Err(_e) => 0
    };

    exp_time > now.timestamp()
}

fn get_existing_token() -> Result<Token, env::VarError> {
   let token = env::var(SPF_ACCESS_TOKEN)?;

   Ok(Token { 
       access_token: token, 
       expiration_time: chrono::Utc::now().timestamp() + 10
    })
}

pub fn get_auth_token() -> Result<Token, SpotifyError> {
    if is_authenticated() {
        get_existing_token().map_err(|e| SpotifyError::ConfigError(format!("{:?}", e)))
    } else {
        authenticate().map_err(|e| SpotifyError::AuthError(format!("{:?}", e)))
    }
}

