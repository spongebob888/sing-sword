use std::fs;

use crate::config::{ISingBox,Sword};
use crate::utils;
use crate::service;

#[tauri::command]
pub fn run_config(config_str: &str) -> Result<(), String> {
    
    let sb : ISingBox = serde_json::from_str(config_str).map_err(|err| err.to_string())?;

    Sword::global().set_sing_box(sb).map_err(|err| err.to_string())?;

    service::Core::global()
        .run_core()
        .map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_config() -> Result<String, String> {
    let sb = Sword::global().sing_box.read();
    serde_json::to_string(&*sb).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn reset_proxy() -> Result<(), String> {
    utils::sysopt::Sysopt::global().reset_sysproxy().map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_default_config() -> Result<String, String> {
    let sing_box_config = utils::init::get_default_box_config()
    .map_err(|err|err.to_string())?;

    serde_json::to_string_pretty(&sing_box_config)
    .map_err(|err|err.to_string())
}

#[tauri::command]
pub fn get_profile_list() -> Result<Vec<String>,String> {
    utils::dirs::list_profile().map_err(|err|err.to_string())
}

#[tauri::command]
pub fn get_selected_profile() -> Result<String,String> {
    Sword::global().profile_name()
    .ok_or("selected profile not found".to_string())
}

#[tauri::command]
pub fn change_profile(name:&str) -> Result<(),String> {
    Sword::global().change_profile(name.to_string()).map_err(|err|err.to_string())
}
