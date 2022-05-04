use std::path::PathBuf;

use crate::user_settings::{UserSettings, USER_SETTINGS};

#[tauri::command]
pub async fn get_data(
) -> Result<UserSettings, String> {
    let data = USER_SETTINGS
        .read()
        .or(Err("unable to open settings"))?;
    Ok((*data).clone())
}
#[tauri::command]
pub async fn set_data(
    new: UserSettings,
) -> Result<(), String> {
    new.save()?;
    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    *dat = new;
    Ok(())
}
#[tauri::command]
pub async fn set_path(
    path_to: PathBuf,
) -> Result<(), String> {
    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    (*dat).vulnus.path = path_to;
    dat.save()?;
    Ok(())
}
#[tauri::command]
pub async fn get_save_path() -> Result<PathBuf, String> {
	let save_to = UserSettings::get_save_dir()?;
	let parent = save_to.parent().ok_or("unable to get parent of save dir")?;
    Ok(parent.to_path_buf())
}
#[tauri::command]
pub async fn dir_exist(dir:PathBuf) -> bool {
  dir.exists()
}