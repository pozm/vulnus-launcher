#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::Path;
use std::io::Cursor;
use tauri::Runtime;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![install_vulnus,check_vulnus_tag])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn check_vulnus_tag(vulnus_dir:String) -> bool {


  let path_to_vulnus = Path::new(&vulnus_dir).join("vulnus.exe");
	path_to_vulnus.exists()
}

#[tauri::command]
async fn install_vulnus(vulnus_dir:String,url:String) -> bool {

  println!("install: {}\nremote:{}", &vulnus_dir,&url);

  let path_to_vulnus = Path::new(&vulnus_dir);

  fs::create_dir_all(&path_to_vulnus).await.unwrap();

	let bytes = reqwest::get(&url).await.unwrap().bytes().await.unwrap();
	let mut read = Cursor::new(bytes.to_vec());
	let mut zip = zip::ZipArchive::new(&mut read).unwrap();
  println!("extracting.");
	zip.extract(&path_to_vulnus);

  println!("done!!");
	return true
}