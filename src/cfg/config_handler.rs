use super::config::Config;

pub fn load_config() -> Option<Config> {
    let config = confy::load("spf", None);

    match config {
        Ok(cfg) => Some(cfg),
        Err(_e) => None
    }
}

pub fn update_config(cfg: &Config) -> Result<Config, ConfigError> {
    let existing_config = confy::load("spf", None).map_err(|_| ConfigError::Load)?;

    Ok(existing_config)
}

pub enum ConfigError {
    Update(Config),
    Load,
}
