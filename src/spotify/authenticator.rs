use chrono::Utc;

use crate::cfg::models::App;

// TODO: We probably wanna do this with env-variables instead?
fn is_authenticated() -> bool {
    //let now = Utc::now();

    //account.expiration_time.timestamp() > now.timestamp()
    false
}

fn get_config() -> Option<App> {
    match crate::cfg::manager::load_config() {
        Ok(cfg) => Some(cfg),
        Err(_e) => None,
    }
}

pub fn authenticate() -> Result<(), SpotifyError> {
    let cfg = match get_config() {
        Some(val) => val,
        None => return Err(SpotifyError::MissingConfig)
    };

    if is_authenticated() {
        Ok(())
    } else {
        // TODO: See if we can refresh
        Err(SpotifyError::AuthError)
    }
}

pub enum SpotifyError {
    AuthError,
    MissingConfig,
}
