use std::{path::{PathBuf, Path}, io::Cursor,io, fs};

// ありがとうございました　https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// end

fn find_maps(path: &PathBuf, maps: &mut Vec<PathBuf>) {
	if path.join("meta.json").exists() {
		maps.push(path.to_path_buf());
		return;
	}
	for file_us in fs::read_dir(path).unwrap() {
		if let Ok(entry) = file_us {
			if entry.metadata().unwrap().is_dir() {
				find_maps(&entry.path(), maps);
			}
		}
	}
}

pub async fn install_map_remote<S:Into<PathBuf> + Copy>(install_path: S, map_remote_location: &str) -> Result<String, String> {
	let map_local_location:PathBuf = install_path.into().join("maps");
	let zip_data = reqwest::get(map_remote_location).await
		.or(Err("unable to download map"))?
		.bytes().await.or(Err("unable to download map"))?;

	let mut read = Cursor::new(zip_data);
	let mut zip = zip::ZipArchive::new(&mut read).unwrap();
	let temp_dir : PathBuf = install_path.into().join("maps_temp");
	fs::create_dir_all(&temp_dir).or(Err("unable to create temp dir"))?;
	let uuid_bind = uuid::Uuid::new_v4().to_string();
	zip.extract(&temp_dir.join(&uuid_bind)).or(Err("unable to extract map"))?;
	
	let mut map_folders:Vec<PathBuf> = vec![];
	find_maps(&temp_dir.join(&uuid_bind), &mut map_folders);

	println!("{:?}", map_folders);

	install_maps(&map_local_location,&map_folders)?;

	Ok(format!("{}",map_folders.len()))
}
pub fn install_maps(install_path: &PathBuf, maps:&Vec<PathBuf>) -> Result<(), String> {
	for map in maps {
		copy_dir_all(map, install_path.join(map.file_name().unwrap())).or(Err("unable to copy map"))?;
	}
	Ok(())
}