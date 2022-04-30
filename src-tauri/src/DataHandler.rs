use crate::UserSettings::{UserSettings, USER_SETTINGS};
use tauri::Runtime;

#[tauri::command]
pub async fn get_data<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<UserSettings, String> {
    Ok(USER_SETTINGS
        .read()
        .or(Err("unable to open settings"))?
        .clone())
}
#[tauri::command]
pub async fn set_data<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    new: UserSettings,
) -> Result<(), String> {
    new.save()?;
    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    *dat = new;
    Ok(())
}
