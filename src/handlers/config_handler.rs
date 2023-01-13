use crate::UserConfigure;
use crate::cfg;
use cfg::models::App;
use cfg::manager::CfgError;

pub fn update_config(new_cfg: &UserConfigure) -> Result<String, CfgError> {
    let cfg_path = match cfg::manager::get_config_path().ok() {
        Some(cfg) => cfg,
        None => "No path found".to_string()
    };

    if new_cfg.client_id.is_none() && 
        new_cfg.client_secret.is_none() && 
        new_cfg.redirect_port.is_none() {
        return Ok(format!("Printing config path: '{}'", cfg_path))
    }

    let old_cfg = cfg::manager::load_config()?;

    let updated_cfg = get_updated_cfg(&new_cfg, &old_cfg);

    cfg::manager::update_app_config(&updated_cfg)?;

    Ok(format!("Your config was updated. You can find it here: '{}'", cfg_path))
}

fn get_updated_cfg(new_cfg: &UserConfigure, old_cfg: &App) -> App {
    let client_id = if let Some(id) = &new_cfg.client_id {
        id.to_owned()
    } else {
        old_cfg.client_id.to_owned()
    };

    let client_secret = if let Some(secret) = &new_cfg.client_secret {
        secret.to_owned()
    } else {
        old_cfg.client_secret.to_owned()
    };

    let port = if let Some(redirect_port) = new_cfg.redirect_port {
        redirect_port.to_owned()
    } else {
        old_cfg.redirect_port.to_owned()
    };

    App::new(&client_id, &client_secret, port)
}
