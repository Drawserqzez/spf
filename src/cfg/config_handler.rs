use super::config::{Config, App};

const CONFY_APP_NAME: &str = "spf";

pub fn load_config() -> Option<Config> {
    let config = confy::load(CONFY_APP_NAME, None);

    match config {
        Ok(cfg) => Some(cfg),
        Err(_e) => None
    }
}

pub fn update_client_info(cfg: &Config) -> Result<Config, ConfigError> {
    let existing_config: Config = 
        confy::load("spf", None).map_err(|_| ConfigError::Load)?;

    let new_config = Config {
        app: get_latest_app_config(&cfg.app, &existing_config.app).to_owned(),
        account: existing_config.account
    };

    save_config(&new_config)?;

    Ok(new_config)
}

fn save_config(cfg: &Config) -> Result<(), ConfigError> {
    confy::store(CONFY_APP_NAME, None, cfg).map_err(|_| ConfigError::Update)
}

fn get_latest_app_config(new_config: &App, old_config: &App) -> App {
    let client_id = if new_config.client_id == old_config.client_id { 
        old_config.client_id.to_owned() 
    } else { 
        new_config.client_id.to_owned()
    };

    let client_secret = if new_config.client_secret == old_config.client_secret {
        old_config.client_secret.to_owned()
    } else {
        new_config.client_secret.to_owned()
    };

    let port = if new_config.redirect_port == old_config.redirect_port {
        old_config.redirect_port
    } else {
        new_config.redirect_port
    };

    App::new(&client_id, &client_secret, port)
}

#[derive(std::fmt::Debug)]
pub enum ConfigError {
    Update,
    Load,
}
