use super::models::{Config, App};

const CONFY_APP_NAME: &str = "spf";

pub fn load_config() -> Result<Config, CfgError> {
    let config: Result<Config, CfgError> = confy::load(CONFY_APP_NAME, None).map_err(|_| CfgError::Load);

    return config;
}

fn save_config(cfg: &Config) -> Result<(), CfgError> {
    confy::store(CONFY_APP_NAME, None, cfg).map_err(|_| CfgError::Update)
}


pub fn update_app_config(app_cfg: &App) -> Result<Config, CfgError> {
    let existing_config = load_config()?;

    let new_config = Config {
        app: app_cfg.to_owned(),
        account: existing_config.account
    };

    save_config(&new_config)?;

    load_config()
}

#[derive(Debug)]
pub enum CfgError {
    Update,
    Load,
}
