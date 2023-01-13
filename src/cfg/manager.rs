use super::models::App;

const CONFY_APP_NAME: &str = "spf";
const CONFY_FILE_NAME: &str = "app";

pub fn load_config() -> Result<App, CfgError> {
    let config: Result<App, CfgError> = confy::load(CONFY_APP_NAME, CONFY_FILE_NAME)
        .map_err(|_| CfgError::Load);

    return config;
}

fn save_config(cfg: &App) -> Result<(), CfgError> {
    confy::store(CONFY_APP_NAME, CONFY_FILE_NAME, cfg)
        .map_err(|_| CfgError::Update)
}


pub fn update_app_config(app_cfg: &App) -> Result<App, CfgError> {
    let existing_config = load_config()?;

    save_config(&app_cfg)?;

    load_config()
}

#[derive(Debug)]
pub enum CfgError {
    Update,
    Load,
}
