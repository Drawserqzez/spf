use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_port: u16,
}

impl App {
    pub fn from_old(old: &App) -> Self {
        Self {
            client_id: old.client_id.to_owned(),
            client_secret: old.client_secret.to_owned(),
            redirect_port: old.redirect_port
        }
    }

    pub fn new(client_id: &str, client_secret: &str, redirect_port: u16) -> Self {
        Self {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            redirect_port,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new("", "", 1337)
    }
}

