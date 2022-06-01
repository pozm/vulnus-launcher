use std::io::Cursor;

use tauri::Runtime;

use crate::{user_settings::{ModData, USER_SETTINGS}, utils::{get_vulnus_dir, download_item, BEPINEX_ZIP}};



pub async fn update_mods() -> Result<(),String> {
	let set_ro = USER_SETTINGS.read().or(Err("Unable to open settings"))?.clone();
	let mod_list = reqwest::get(set_ro.modding.source_list.clone()).await.or(Err("Unable update mod lists"))?.json::<Vec<ModData>>().await.or(Err("Unable to parse mod lists"))?;
	
	let mut set = USER_SETTINGS.write().or(Err("Unable to open settings"))?;
	let old_mods : Vec<ModData> = (*set).modding.mods.clone();

	let new_mods : Vec<ModData> = mod_list.iter().filter(|m| !old_mods.iter().any(|om| om.name == m.name || m.last_updated <= om.last_updated)).cloned().collect();

	println!("old : {:#?} \nnew : {:#?}",old_mods,new_mods);

	(*set).modding.mods = old_mods.iter().cloned().chain(new_mods).collect();

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
	let mut set = USER_SETTINGS.write().or(Err("unable to open settings"))?.clone();
	set.modding.mods[idx as usize].clone()
		.install(&window).await?;
	set.modding.mods[idx as usize]
		.installed = true;
	set.save()?;
	(*USER_SETTINGS.write().or(Err("unable to open settings"))?) = set;

  	Ok(())
}
#[tauri::command(async)]
pub async fn remove_mod<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>,idx:u32) -> Result<(), String> {
	let mut set = USER_SETTINGS.write().or(Err("unable to open settings"))?.clone();
	set.modding.mods[idx as usize].clone()
		.remove().await?;
	set.modding.mods[idx as usize]
		.installed = false;
	set.save()?;
	(*USER_SETTINGS.write().or(Err("unable to open settings"))?) = set;

  	Ok(())
}