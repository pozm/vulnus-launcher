use std::io::Cursor;

use tauri::Runtime;

use crate::{UserSettings::USER_SETTINGS, utils::{download_item, get_vulnus_dir, BEPINEX_ZIP}};

#[tauri::command]
pub async fn install_bepinex<R: Runtime>(
    _app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
	let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
	let vulnus_dir = get_vulnus_dir(Some(&set.vulnus.version.current));


	let zip_file = download_item(BEPINEX_ZIP, format!("BEPINEX"), &window).await?;
	let mut read = Cursor::new(zip_file);
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&vulnus_dir);
	Ok(())
}
#[tauri::command]
pub async fn check_bepinex() -> Result<bool,String> {
	let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
	let vulnus_dir = get_vulnus_dir(Some(&set.vulnus.version.current));
	Ok(vulnus_dir.join("winhttp.dll").exists() && vulnus_dir.join("BepInEx").exists())
}