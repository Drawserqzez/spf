use super::config::Config;

pub fn load_config() -> Option<Config> {
    let config = confy::load("spf", None);

    match config {
        Ok(cfg) => Some(cfg),
        Err(_e) => None
    }
}
