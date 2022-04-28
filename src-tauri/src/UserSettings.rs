use bincode::de::read::Reader;
use bincode::enc::write::Writer;
use chrono::{DateTime, Utc, serde::{ts_seconds}};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use tauri::api::path::data_dir;
use std::fs::{File, create_dir_all};
use std::ops::Index;
use std::{sync::RwLock, path::PathBuf, fs::OpenOptions};
use std::io::prelude::*;

lazy_static!{
	pub static ref USER_SETTINGS: RwLock<UserSettings> = RwLock::new(UserSettings::read().unwrap());
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UserSettings {
	version:VersionSettings,
	launcher:LauncherSettings
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct LauncherSettings {
	latest:String
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct VersionSettings {
	current:String,
	latest:String,
	versions:Vec<String>,
	last_check:DateTime<Utc>
}

const SAVE_TO_PATH: &str = "./vulnus-launcher/data.bin";


struct FileWrapper(File);

impl Writer for FileWrapper {
	fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
		self.0.write(bytes);
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
		Self{
			version: VersionSettings{
				current: "".to_string(),
				latest: "".to_string(),
				versions: vec![],
				last_check: Utc::now() - chrono::Duration::days(1)
			},
			launcher: LauncherSettings{
				latest: "".to_string()
			}
		}
	}
	pub fn get_launcher_dir() -> Result<PathBuf,String> {

		let data = data_dir().ok_or("Could not get data dir")?;
	
		Ok(data.join(SAVE_TO_PATH))
	}

	pub fn read() -> Result<Self,String> {
		let data_path = Self::get_launcher_dir()?;

		create_dir_all(data_path.parent().unwrap()).or(Err("Could not create launcher dir".to_string()))?;

		let data_file = OpenOptions::new().read(true).create(true).write(true).open(data_path).or(Err("Unable to make data file"))?;
		let config = bincode::config::standard();
		let return_data : Self = match bincode::serde::decode_from_reader(FileWrapper(data_file), config) {
			Ok(ret_data) => ret_data,
			Err(e) => {
				println!("unable to read data file: {} | returning defaults.",e);
				Self::new()
			},
		};

		Ok(return_data)
	}

	pub fn save(&self) -> Result<(),String> {
		
		let data_path = Self::get_launcher_dir()?;

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
}