use chrono::Utc;

use crate::cfg::models::{Config, Account};

fn is_authenticated(account: &Account) -> bool {
    let now = Utc::now();

    account.expiration_time.timestamp() > now.timestamp()
}

fn get_config() -> Option<Config> {
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

    if is_authenticated(&cfg.account) {
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
