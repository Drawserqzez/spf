use chrono::{DateTime, Utc};

use crate::cfg::config::Account;

fn is_authenticated(account: &Account) -> bool {
    let now = Utc::now();

    account.expiration_time.timestamp() > now.timestamp()
}

fn get_config() -> Account {
    Account {
        account_service_url: "".to_string(),
        access_token: "hello world".to_string(),
        refresh_token: "it's a secret to everybody".to_string(),
        expiration_time: DateTime::<Utc>::MIN_UTC
    }
}

pub fn authenticate() -> Result<(), SpotifyError> {
    let account = get_config();

    if is_authenticated(&account) {
        Ok(())
    } else {
        // TODO: See if we can refresh
        Err(SpotifyError::AuthError)
    }
}

pub enum SpotifyError {
    AuthError
}
