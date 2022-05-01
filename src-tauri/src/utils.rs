use std::{path::PathBuf, cmp::min, fs, io::Write};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::Runtime;
use crate::UserSettings::USER_SETTINGS;

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


pub fn get_vulnus_dir(tag: Option<&str>) -> PathBuf {
    USER_SETTINGS.read().unwrap().get_launcher_dir()
        .join(tag.unwrap_or(""))
}
pub fn get_vulnus_download(tag: &str) -> String {
    format!(
        "https://github.com/beat-game-dev/Vulnus/releases/download/{}/Vulnus_Beta_{}.zip",
        tag,
		if cfg!(macos) {"Mac"} else {"Win"}
    )
}
pub const BEPINEX_ZIP : &str = "https://cdn.discordapp.com/attachments/812076013285801985/969323588706517042/UnityIL2CPP_x64.zip";

pub fn install_symlinks(tag: &str) {
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

pub async fn download_item<S, R:Runtime>(url:&str,identifier:S, window: &tauri::Window<R>) -> Result<Vec<u8>,String> where S : Into<String> {
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
