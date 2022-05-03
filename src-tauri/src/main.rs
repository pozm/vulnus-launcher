#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::{self, OpenOptions, Permissions};
use std::io::{Cursor, Write};
use std::os::unix::prelude::{OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::process::Command;
use tauri::api::path::desktop_dir;
use tauri::Runtime;
use vulnus_launcher::utils::set_as_safe;
use vulnus_launcher::{
	utils::{download_item, get_vulnus_dir, install_symlinks, get_vulnus_download},
	UserSettings::USER_SETTINGS,
	Modding::{self, update_mods},
	DataHandler
};

fn main() {
    USER_SETTINGS.read().unwrap().save().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_vulnus_tag,
            remove_vulnus,
            install_vulnus_progress, 
            Modding::install_bepinex,
            Modding::check_bepinex,
            Modding::fetch_mods,
            Modding::install_mod,
            DataHandler::get_data,
            DataHandler::get_save_path,
            DataHandler::set_path,
            DataHandler::set_data,
            DataHandler::dir_exist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn check_vulnus_tag(tag: String) -> bool {
    let dir = get_vulnus_dir(Some(&tag));
	#[cfg(target_os="windows")]
    let path_to_vulnus = dir.join("vulnus.exe");
	#[cfg(target_os="macos")]
    let path_to_vulnus = dir.join("Vulnus.app");
    println!("EXISTS? {:?}", dir);
    path_to_vulnus.exists()
}

#[tauri::command]
async fn remove_vulnus(tag: String) -> bool {
    let path_to_vulnus = get_vulnus_dir(Some(&tag));
    fs::remove_dir_all(path_to_vulnus).is_ok()
}

#[tauri::command(async)]
async fn install_vulnus_progress<R: Runtime>(
    _app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    tag: String,
    desktop: bool,
) -> Result<(), String> {
    let download_url = get_vulnus_download(&tag);
    let vulnus_dir = get_vulnus_dir(Some(&tag));

	println!("install to {:?}", vulnus_dir);

    let zip_file = download_item(&download_url, format!("TAG<{}>", tag), &window).await?;

    let mut read = Cursor::new(zip_file);
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&vulnus_dir);

	
	#[cfg(target_os="macos")]
	{
		println!("running macos patches.");
		//this library needs to be put into local libs so that it can be found and loaded.
		fs::copy(vulnus_dir.join("BuildMac.app/Contents/Frameworks/UnityPlayer.dylib"), "/usr/local/lib/libUnityPlayer.dylib").unwrap();
		// by default, it will be marked as unsafe, and you will not be able to run it.
		set_as_safe("/usr/local/lib/libUnityPlayer.dylib")?;
		// allow it to be executed, making it "runable"
		fs::set_permissions(vulnus_dir.join("BuildMac.app/Contents/Macos/Vulnus"), Permissions::from_mode(111));
		// consistency with windows.
		fs::rename(vulnus_dir.join("BuildMac.app"), vulnus_dir.join("Vulnus.app"));
		println!("complete.")
	}
	println!("installing symlinks.");
    install_symlinks(&tag);
	#[cfg(target_os="windows")]
    if desktop {
        println!("make desktop shortcut.");
        let vulnus_exe = vulnus_dir.join("vulnus.exe");
        let vulnus_desktop = desktop_dir().unwrap().join(format!("vulnus_{}.exe", &tag));
        if !vulnus_desktop.exists() {
            symlink::symlink_file(vulnus_exe, &vulnus_desktop).unwrap();
        }
    }
    return Ok(());
}
