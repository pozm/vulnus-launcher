use bincode::de::read::Reader;
use bincode::enc::write::Writer;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use tauri::Runtime;
use tauri::api::path::{data_dir, local_data_dir};
use tokio::fs as fs_tokio;
use std::fs::{File, create_dir_all, self, remove_file};
use std::{sync::RwLock, path::PathBuf, fs::OpenOptions};
use std::io::prelude::*;

use crate::utils::{get_vulnus_dir, download_item};

lazy_static!{
	pub static ref USER_SETTINGS: RwLock<UserSettings> = RwLock::new(UserSettings::read().unwrap());
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UserSettings {
	pub vulnus:VulnusSettingData,
	pub launcher:LauncherSettings,
	pub modding:ModdingData
}
#[derive(Debug, Serialize, Deserialize,Clone)]

pub struct VulnusSettingData {
	pub version : VulnusVersionSettings,
	pub path : PathBuf
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct LauncherSettings {
	pub latest_version:String
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ModData {
	pub name:String,
	pub available_versions:Vec<String>,
	pub current_version:String,
	pub last_updated:DateTime<Utc>,
	pub download_url:String,
	#[serde(default)]
	pub installed:bool,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ModdingData {
	pub mods: Vec<ModData>,
	pub custom_source : bool,
	pub source_list : String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct VulnusVersionSettings {
	pub current:String,
	pub latest:String,
	pub versions:Vec<String>,
	pub last_check:DateTime<Utc>
}

const SAVE_TO_PATH: &str = "vulnus-launcher/data.bin";


impl ModData {
	pub async fn install<R:Runtime>(&mut self, window: &tauri::Window<R>) -> Result<(), String> {
		let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
		let vulnus_dir = get_vulnus_dir(Some(&set.vulnus.version.current));
		let dll_file = download_item(&self.download_url, format!("MOD<{}>",self.name), window).await?;
		println!("writing mod.");
		OpenOptions::new().write(true).truncate(true).create(true)
			.open(vulnus_dir.join("BepInEx/plugins/").join(format!("{}.dll",&self.name))).or(Err("unable to open mod dll"))?
				.write_all(&dll_file).or(Err("Unable to write dll"))?;
		Ok(())
	}
	pub async fn remove(&mut self) -> Result<(),String> {
		let set = USER_SETTINGS.read().or(Err("unable to open settings"))?.clone();
		let vulnus_dir = get_vulnus_dir(Some(&set.vulnus.version.current));
		fs_tokio::remove_file(vulnus_dir.join("BepInEx/plugins/").join(format!("{}.dll",&self.name))).await.or(Err("unable to remove mod dll"))?;
		Ok(())
	}
}

struct FileWrapper(File);

impl Writer for FileWrapper {
	fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
		self.0.write(bytes).unwrap();
		Ok(())
	}
}
impl Reader for FileWrapper {
	fn read(&mut self, bytes: &mut [u8]) -> Result<(), bincode::error::DecodeError> {
		self.0.read(bytes).unwrap();
		Ok(())
	}
}


impl UserSettings{
	pub fn new() -> Self {
		let local_dir = local_data_dir().expect("unable to get document directory");
		fs::create_dir_all(&local_dir.join("vulnus-launcher")).expect("failed to create launcher directory");
		Self{
			// version: VersionSettings{
			// 	current: "".to_string(),
			// 	latest: "".to_string(),
			// 	versions: vec![],
			// 	last_check: Utc::now() - chrono::Duration::days(1)
			// },
			// launcher: LauncherSettings{
			// 	latest: "".to_string()
			// }
			vulnus: VulnusSettingData { 
				version: VulnusVersionSettings{
						current: "".to_string(),
						latest: "".to_string(),
						versions: vec![],
						last_check: Utc::now() - chrono::Duration::days(1)
					}, 
				path: local_dir
			},
			launcher: LauncherSettings{
				latest_version: "".to_string()
			},
			modding: ModdingData { mods: vec![],custom_source:false,source_list:"https://gist.githubusercontent.com/pozm/36652eea0e7652b76eb26d8abf71e939/raw/temp_mod_list.json".to_string() }
		}
	}
	pub fn get_save_dir() -> Result<PathBuf,String> {

		let data = data_dir().ok_or("Could not get data dir")?;
	
		Ok(data.join(SAVE_TO_PATH))
	}

	pub fn get_launcher_dir(&self) -> PathBuf {
		self.vulnus.path.join("vulnus-launcher")
	}

	pub fn read() -> Result<Self,String> {
		let data_path = Self::get_save_dir()?;

		create_dir_all(data_path.parent().unwrap()).or(Err("Could not create launcher dir".to_string()))?;

		let data_file = OpenOptions::new().read(true).create(true).write(true).open(data_path).or(Err("Unable to make data file"))?;
		let config = bincode::config::standard();
		let mut return_data : Self = match bincode::serde::decode_from_reader(FileWrapper(data_file), config) {
			Ok(ret_data) => ret_data,
			Err(e) => {
				println!("unable to read data file: {} | returning defaults.",e);
				Self::new() //
			},
		};
		// prevent bincode from reading another value by accident
		if !return_data.modding.custom_source {
			return_data.modding.source_list = "https://gist.githubusercontent.com/pozm/36652eea0e7652b76eb26d8abf71e939/raw/temp_mod_list.json".to_string();
		}
		// println!("got data {:#?}",return_data);
		Ok(return_data)
	}

	pub fn save(&self) -> Result<(),String> {
		
		let data_path = Self::get_save_dir()?;

		// println!("save data {:#?}",&self);

		create_dir_all(data_path.parent().unwrap()).or(Err("Could not create launcher dir".to_string()))?;

		let data_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(data_path).or(Err("Unable to make data file"))?;
		let config = bincode::config::standard();
		bincode::serde::encode_into_writer(&self, FileWrapper(data_file), config).or(Err("Unable to write data file"))?;

		Ok(())
	}
	pub fn reset() -> Result<(),String> {
		let data_path = Self::get_save_dir()?;
		remove_file(data_path).or(Err("Unable to remove data file"))?;
		Ok(())

	}
}