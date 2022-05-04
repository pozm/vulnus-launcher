use std::io::Cursor;

use tauri::Runtime;

use crate::{user_settings::{USER_SETTINGS, ModData}, utils::{download_item, get_vulnus_dir, BEPINEX_ZIP}};

const MOD_LIST_FILE:&str = "https://gist.githubusercontent.com/pozm/36652eea0e7652b76eb26d8abf71e939/raw/temp_mod_list.json";


pub async fn update_mods() -> Result<(),String> {
	let mod_list = reqwest::get(MOD_LIST_FILE).await.or(Err("Unable update mod lists"))?.json::<Vec<ModData>>().await.or(Err("Unable to parse mod lists"))?;

	let mut set = USER_SETTINGS.write().or(Err("Unable to open settings"))?;
	(*set).modding.mods = mod_list;
	Ok(())
}

#[tauri::command]
pub async fn fetch_mods() -> Result<(), String> {

	update_mods().await?;

  	Ok(())
}

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
    zip.extract(&vulnus_dir).or(Err("Unable to extract zip"))?;
	Ok(())
}
#[tauri::command]
pub async fn check_bepinex() -> Result<bool,String> {
	let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
	let vulnus_dir = get_vulnus_dir(Some(&set.vulnus.version.current));
	Ok(vulnus_dir.join("winhttp.dll").exists() && vulnus_dir.join("BepInEx").exists())
}
#[tauri::command(async)]
pub async fn install_mod<R: Runtime>(_app: tauri::AppHandle<R>, window: tauri::Window<R>,idx:u32) -> Result<(), String> {
	let mut set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
	set.modding.mods[idx as usize].clone()
		.install(&window).await?;
	set.modding.mods[idx as usize]
		.installed = Some(true);
	set.save()?;
	(*USER_SETTINGS.write().or(Err("unable to open settings"))?) = set;

  	Ok(())
}