#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::{self};
use std::io::{Cursor};
use tauri::Runtime;
use vulnus_launcher::{
	utils::{download_item, get_vulnus_dir, install_symlinks, get_vulnus_download},
	user_settings::USER_SETTINGS,
	modding::{self},
	data_handler
};

fn main() {
    USER_SETTINGS.read().unwrap().save().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_vulnus_tag,
            remove_vulnus,
            install_vulnus_progress, 
            modding::install_bepinex,
            modding::check_bepinex,
            modding::fetch_mods,
            modding::install_mod,
            modding::remove_mod,
            data_handler::get_data,
            data_handler::get_save_path,
            data_handler::set_path,
            data_handler::set_data,
            data_handler::dir_exist
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
    zip.extract(&vulnus_dir).or(Err("unable to extract zip"))?;

	
	#[cfg(target_os="macos")]
	{
		use vulnus_launcher::utils::set_as_safe;
		use std::fs::Permissions;
		use std::os::unix::fs::PermissionsExt;

		println!("running macos patches.");
		//this library needs to be put into local libs so that it can be found and loaded.
		fs::copy(vulnus_dir.join("BuildMac.app/Contents/Frameworks/UnityPlayer.dylib"), "/usr/local/lib/libUnityPlayer.dylib").unwrap();
		// by default, it will be marked as unsafe, and you will not be able to run it.
		set_as_safe("/usr/local/lib/libUnityPlayer.dylib")?;
		// allow it to be executed, making it "runable"
		fs::set_permissions(vulnus_dir.join("BuildMac.app/Contents/Macos/Vulnus"), Permissions::from_mode(111)).or(Err("unable to set permissions"))?;
		// consistency with windows.
		fs::rename(vulnus_dir.join("BuildMac.app"), vulnus_dir.join("Vulnus.app")).or(Err("unable to rename app"))?;
		println!("complete.")
	}
	println!("installing symlinks.");
    if let Err(sme) = install_symlinks(&tag) {
		eprintln!("unable to install symlinks: {}", sme);
	}
	#[cfg(target_os="windows")]
    if desktop {
		use tauri::api::path::desktop_dir;
        println!("make desktop shortcut.");
        let vulnus_exe = vulnus_dir.join("vulnus.exe");
        let vulnus_desktop = desktop_dir().unwrap().join(format!("vulnus_{}.exe", &tag));
        if !vulnus_desktop.exists() {
            symlink::symlink_file(vulnus_exe, &vulnus_desktop).unwrap();
        }
    }
	#[cfg(unix)]
	if desktop {
		println!("impl")
	}

    return Ok(());
}
