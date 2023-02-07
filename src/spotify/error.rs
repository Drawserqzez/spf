use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SpotifyError {
    AuthError(String),
    ConfigError(String),
    RequestError(String),
}

impl fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uh oh! Looks like something went wrong when trying to access spotify!")
    }
}

impl Error for SpotifyError {}
