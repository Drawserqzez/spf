use chrono::Utc;
use std::env;
use crate::spotify::auth::api::authenticate;

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

fn get_existing_token() -> Result<String, SpotifyError> {
    env::var(SPF_ACCESS_TOKEN)
        .map_err(|_| SpotifyError::AuthError)
}

pub fn get_auth_token() -> Result<String, SpotifyError> {
    if is_authenticated() {
        get_existing_token()
    } else {
        Ok(authenticate().unwrap().access_token)
    }
}

pub enum SpotifyError {
    AuthError,
}
