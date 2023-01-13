use crate::cfg;
use cfg::models::App;
use cfg::manager::CfgError;

pub fn update_config(new_cfg: &App) -> Result<&str, CfgError> {
    let old_cfg = cfg::manager::load_config();
    let res = cfg::manager::update_app_config(new_cfg);

    Ok("Config updated :DDDD")
}
