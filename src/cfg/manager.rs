use super::models::App;

const CONFY_APP_NAME: &str = "spf";
const CONFY_FILE_NAME: &str = "app";

pub fn load_config() -> Result<App, confy::ConfyError> {
    confy::load(CONFY_APP_NAME, CONFY_FILE_NAME)
}

fn save_config(cfg: &App) -> Result<(), confy::ConfyError> {
    confy::store(CONFY_APP_NAME, CONFY_FILE_NAME, cfg)
}


pub fn update_app_config(app_cfg: &App) -> Result<App, confy::ConfyError> {
    save_config(&app_cfg)?;

    load_config()
}

pub fn get_config_path() -> Result<String, confy::ConfyError> {
    let path = confy::get_configuration_file_path(CONFY_APP_NAME, CONFY_FILE_NAME)?;

    path.into_os_string()
        .into_string()
        .map_err(|_| confy::ConfyError::BadConfigDirectory("Os string to path invalid".to_string()))
}

