#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use tauri::api::path::{desktop_dir, document_dir};
use tauri::{Runtime, ShellScope, ShellScopeError};
use tokio::{fs::File, io::AsyncWriteExt};
use futures_util::StreamExt;
use vulnus_launcher::DataHandler;
use vulnus_launcher::UserSettings::USER_SETTINGS;
use std::cmp::min;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
enum InstallProgressState {
	Start,
	Downloading,
	Done
}
#[derive(Serialize, Deserialize, Clone)]
struct InstallProgressData {
	total: u64,
	current: u64,
	state:InstallProgressState,
	identifier:String
}
impl InstallProgressData {
	pub fn new(total:u64,identifier:&str) -> Self {
		Self {
			total,
			current:0,
			state:InstallProgressState::Start,
			identifier:identifier.to_string()
		}
	}
	pub fn update(&mut self,current:u64) {
		self.current = current;
	}
	pub fn done(&mut self) {
		self.state = InstallProgressState::Done;
	}
	pub fn downloading(&mut self) {
		self.state = InstallProgressState::Downloading;
	}

}


fn main() {
	USER_SETTINGS.read().unwrap().save().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_vulnus,
            check_vulnus_tag,
            remove_vulnus,
			install_bepinex,
			install_vulnus_progress,
			DataHandler::get_data,
			DataHandler::set_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_vulnus_dir(tag: Option<&str>) -> PathBuf {
    document_dir()
        .unwrap()
        .join("vulnus-launcher")
        .join(tag.unwrap_or(""))
}
fn get_vulnus_download(tag: &str) -> String {
    format!(
        "https://github.com/beat-game-dev/Vulnus/releases/download/{}/Vulnus_Beta_Win.zip",
        tag
    )
}
const BEPINEX_ZIP : &str = "https://cdn.discordapp.com/attachments/812076013285801985/969323588706517042/UnityIL2CPP_x64.zip";

fn install_symlinks(tag: &str) {
    let launcher_dir = get_vulnus_dir(None);
    let tag_dir = get_vulnus_dir(Some(tag));

    let launcher_settings = launcher_dir.join("game_settings.json");
    let launcher_maps = launcher_dir.join("game_maps");

    if !launcher_maps.exists() {
        fs::create_dir_all(&launcher_maps).expect("failed to create launcher maps directory");
    }
    if !launcher_settings.exists() {
        fs::create_dir_all(&launcher_settings.parent().unwrap())
            .expect("failed to create launcher game settings directory");
        let mut f = fs::File::create(&launcher_settings)
            .expect("failed to create launcher game settings file");
        let game_settings: &[u8] = include_bytes!("../default_game_settings.json");
        f.write_all(&game_settings)
            .expect("failed to write default game settings file");
    }
    println!(
        "{}->{}",
        launcher_settings.display(),
        tag_dir.join("settings.json").display()
    );
    symlink::symlink_file(&launcher_settings, tag_dir.join("settings.json"));
    symlink::symlink_dir(&launcher_maps, tag_dir.join("maps"));
}

async fn download_item<S, R:Runtime>(url:&str,identifier:S, window: &tauri::Window<R>) -> Result<Vec<u8>,String> where S : Into<String> {
	let client = reqwest::Client::new();

    let res = client.get(url)
		.send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
		
	let mut file_data:Vec<u8> = vec![];
	
	let mut download_state = InstallProgressData::new(total_size,&identifier.into());
	
	window.emit("server://install-progress", &download_state);

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

	download_state.downloading();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
		file_data.extend(chunk);
        downloaded = new;
		download_state.update(downloaded);
        window.emit("server://install-progress", &download_state);
    }
	download_state.done();
	window.emit("server://install-progress", &download_state);

	Ok(file_data)
}



#[tauri::command]
async fn check_vulnus_tag(tag: String) -> bool {
    let path_to_vulnus = get_vulnus_dir(Some(&tag)).join("vulnus.exe");
    path_to_vulnus.exists()
}

#[tauri::command]
async fn remove_vulnus(tag: String) -> bool {
    let path_to_vulnus = get_vulnus_dir(Some(&tag));
    fs::remove_dir_all(path_to_vulnus).is_ok()
}
#[tauri::command]
async fn install_bepinex<R: Runtime>(
    _app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
	let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
	let vulnus_dir = get_vulnus_dir(Some(&set.version.current));


	let zip_file = download_item(BEPINEX_ZIP, format!("BEPINEX"), &window).await?;
	let mut read = Cursor::new(zip_file);
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&vulnus_dir);
	Ok(())
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

    // Reqwest setup

    // pb.finish_with_message(&format!("Downloaded {} to {}", url, path));

	let zip_file = download_item(&download_url, format!("TAG<{}>",tag), &window).await?;

	let mut read = Cursor::new(zip_file);
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&vulnus_dir);

	println!("installing symlinks.");

    install_symlinks(&tag);

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

#[tauri::command]
async fn install_vulnus(tag: String, desktop: bool) -> bool {
    let vulnus_dir = get_vulnus_dir(Some(&tag));

    // println!("install: {}\nremote:{}", &vulnus_dir,&url);

    let path_to_vulnus = Path::new(&vulnus_dir);

    tokio::fs::create_dir_all(&path_to_vulnus).await.unwrap();

    let download_url = get_vulnus_download(&tag);

    let bytes = reqwest::get(&download_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let mut read = Cursor::new(bytes.to_vec());
    let mut zip = zip::ZipArchive::new(&mut read).unwrap();
    println!("extracting.");
    zip.extract(&path_to_vulnus);

    println!("installing symlinks.");

    install_symlinks(&tag);

    if desktop {
        println!("make desktop shortcut.");
        let vulnus_exe = path_to_vulnus.join("vulnus.exe");
        let vulnus_desktop = desktop_dir().unwrap().join(format!("vulnus_{}.exe", &tag));
        if !vulnus_desktop.exists() {
            symlink::symlink_file(vulnus_exe, &vulnus_desktop).unwrap();
        }
    }

    println!("done!!");
    return true;
}
