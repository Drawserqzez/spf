use chrono::{DateTime, Utc};
use std::env;
use crate::spotify::auth::api::authenticate;

const SPF_EXPIRATION_TIME: &str = "SPF_EXPIRATION_TIME";
const SPF_ACCESS_TOKEN: &str = "SPF_ACCESS_TOKEN";
const SPF_REFRESH_TOKEN: &str = "SPF_REFRESH_TOKEN";

fn is_authenticated() -> bool {
    let now = Utc::now();

    let exp_time = match env::var(SPF_EXPIRATION_TIME) {
        Ok(var) => DateTime::parse_from_rfc3339(&var)
            .unwrap().naive_utc(),
        Err(_e) => DateTime::<Utc>::MIN_UTC.naive_utc(),
    };

    exp_time.timestamp() > now.timestamp()
}

fn try_refresh_token() -> Result<String, SpotifyError> {
    let refresh_token = env::var(SPF_REFRESH_TOKEN)
        .map_err(|_| SpotifyError::RefreshError)?;

    Ok(refresh_token)
}

fn get_existing_token() -> Result<String, SpotifyError> {
    env::var(SPF_ACCESS_TOKEN)
        .map_err(|_| SpotifyError::AuthError)
}

pub fn get_auth_token() -> Result<String, SpotifyError> {
    return Ok(authenticate().unwrap().access_token);
    //if !is_authenticated() {
        //if let Some(refresh_token) = try_refresh_token().ok() {
            //return Ok(refresh_token);
        //} else {
            //let tokens = authenticate();
//
            //Ok("".to_string())
        //}
    //} else {
        //get_existing_token()
    //}
}

pub enum SpotifyError {
    AuthError,
    RefreshError,
}
