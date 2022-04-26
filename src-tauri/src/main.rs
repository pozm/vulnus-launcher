#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Cursor, Write};
use tauri::api::path::{document_dir, desktop_dir};
use tauri::{Runtime, ShellScope,ShellScopeError};
use tokio::{fs::{File}, io::AsyncWriteExt};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install_vulnus,check_vulnus_tag])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_vulnus_dir(tag:Option<&str>) -> PathBuf {
	document_dir().unwrap().join("vulnus-launcher").join(tag.unwrap_or(""))
}
fn get_vulnus_download(tag:&str) -> String {
	format!("https://github.com/beat-game-dev/Vulnus/releases/download/{}/Vulnus_Beta_Win.zip",tag)
}

fn install_symlinks(tag:&str) {
	let launcher_dir = get_vulnus_dir(None);
	let tag_dir = get_vulnus_dir(Some(tag));

	let launcher_settings = launcher_dir.join("game_settings.json");
	let launcher_maps = launcher_dir.join("game_maps");

	if !launcher_maps.exists() {
		fs::create_dir_all(&launcher_maps).expect("failed to create launcher maps directory");
	}
	if !launcher_settings.exists() {
		fs::create_dir_all(&launcher_settings.parent().unwrap()).expect("failed to create launcher game settings directory");
		let mut f = fs::File::create(&launcher_settings).expect("failed to create launcher game settings file");
		let game_settings:&[u8] = include_bytes!("../default_game_settings.json");
		f.write_all(&game_settings).expect("failed to write default game settings file");
	}
	println!("{}->{}",launcher_settings.display(),tag_dir.join("settings.json").display());
	symlink::symlink_file(&launcher_settings, tag_dir.join("settings.json"));
	symlink::symlink_dir(&launcher_maps, tag_dir.join("maps"));
}

#[tauri::command]
async fn check_vulnus_tag(tag:String) -> bool {


    let path_to_vulnus = get_vulnus_dir(Some(&tag)).join("vulnus.exe");
    path_to_vulnus.exists()
}

#[tauri::command]
async fn install_vulnus(tag:String,desktop:bool) -> bool {

	let vulnus_dir = get_vulnus_dir(Some(&tag));

    // println!("install: {}\nremote:{}", &vulnus_dir,&url);

    let path_to_vulnus = Path::new(&vulnus_dir);

    tokio::fs::create_dir_all(&path_to_vulnus).await.unwrap();

	let download_url = get_vulnus_download(&tag);

    let bytes = reqwest::get(&download_url).await.unwrap().bytes().await.unwrap();
    let mut read = Cursor::new(bytes.to_vec());
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&path_to_vulnus);

	println!("installing symlinks.");

	install_symlinks(&tag);

	
	if desktop {
		println!("make desktop shortcut.");
		let vulnus_exe = path_to_vulnus.join("vulnus.exe");
		let vulnus_desktop = desktop_dir().unwrap().join(format!("vulnus_{}.exe",&tag));
		if !vulnus_desktop.exists() {
			symlink::symlink_file(vulnus_exe, &vulnus_desktop).unwrap();
		}
	}

    println!("done!!");
    return true
}