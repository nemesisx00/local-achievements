use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::data::AppSettings;
use crate::error;
use crate::retroachievements::{RetroAchievementsAuth, RetroAchievementsUser};
use crate::rpcs3::{Rpcs3Settings, Rpcs3User};
use crate::steam::{SteamAuth, SteamUser};

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

pub const Filename_GameIcon: &str = "game-icon";

pub const Path_Avatars: &str = "avatars";
pub const Path_Games: &str = "games";

pub fn generateImageCacheDir(
	platform: &String,
	group: &String
) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(platform.to_lowercase())
			.join(group);
		
		if !path.exists()
		{
			create_dir_all(&path)?;
		}
	}
	
	return Ok(());
}

/**
Get the cache directory specific to this application.
*/
pub fn getCacheDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.cache_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

/**
Get the configuration directory specific to this application.
*/
pub fn getConfigDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.config_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

/**
Get the data directory specific to this application.
*/
pub fn getDataDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.data_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

/**
Get the absolute path to a cached image, if it exists.
*/
pub fn getImagePath(platform: &String, group: &String, fileName: &String) -> Option<String>
{
	return Some(
		Path::new(getCacheDir(false)?.as_str())
			.join(platform.to_lowercase())
			.join(group)
			.join(fileName)
			.to_str()?
			.into()
	);
}

/**
Read the application's settings data from file.
*/
pub fn loadAppSettings() -> Result<AppSettings>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, AppSettings::FileName),
	};
}

/**
Read the RetroAchievements API authorization data from file.
*/
pub fn loadAuthData_RetroAchievements() -> Result<RetroAchievementsAuth>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, RetroAchievementsAuth::FileName),
	};
}

/**
Read the Steam API authorization data from file.
*/
pub fn loadAuthData_Steam() -> Result<SteamAuth>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, SteamAuth::FileName),
	};
}

pub fn loadImageToBytes(
		platform: &String,
		group: &String,
		fileName: &String
	) -> Result<Vec<u8>>
{
	if let Some(path) = getImagePath(platform, group, fileName)
	{
		let file = File::open(path)?;
		let mut reader = BufReader::new(file);
		let mut buffer = vec![];
		reader.read_to_end(&mut buffer)?;
		
		return Ok(buffer);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the RPCS3 settings data from file.
*/
pub fn loadSettings_Rpcs3() -> Result<Rpcs3Settings>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, Rpcs3Settings::FileName),
	};
}

/**
Read the RPCS3 user data from file.
*/
pub fn loadUserData_Rpcs3() -> Result<Rpcs3User>
{
	return match getDataDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, Rpcs3User::FileName),
	};
}

/**
Read the Steam API user data from file.
*/
pub fn loadUserData_Steam() -> Result<SteamUser>
{
	return match getDataDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, SteamUser::FileName),
	};
}

/**
Read the RetroAchievements API user data from file.
*/
pub fn loadUserData_RetroAchievements() -> Result<RetroAchievementsUser>
{
	return match getDataDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, RetroAchievementsUser::FileName),
	};
}

/**
Generic method to read data from file and deserialize it into a given type which
implements `DeserializeOwned`.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `&'static str` File name with extension of the desired file.
*/
fn readDataFromFile<T>(directory: String, fileName: &'static str) -> Result<T>
	where T: DeserializeOwned
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let file = File::open(&path)
		.context(format!(
			"Failed opening file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	let reader = BufReader::new(file);
	
	return Ok(serde_json::from_reader(reader)
		.context(format!(
			"Failed parsing as JSON file at: '{}'",
			path.as_path().to_str().unwrap()
		))?);
}

/**
Write the application's settings data to file.
*/
pub fn saveAppSettings(settings: &AppSettings) -> Result<()>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, AppSettings::FileName, settings),
	};
}

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn saveAuthData_RetroAchievements(auth: &RetroAchievementsAuth) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, RetroAchievementsAuth::FileName, auth),
	};
}

/**
Write the Steam API authorization data to file.
*/
pub fn saveAuthData_Steam(auth: &SteamAuth) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamAuth::FileName, auth),
	};
}

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn saveImageToCache(
		platform: &String,
		group: &String,
		fileName: &String,
		buffer: &[u8]
	) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let mut path = Path::new(dir.as_str())
			.join(platform.to_lowercase())
			.join(group);
		
		if !path.exists()
		{
			_ = create_dir_all(&path);
		}
		
		path = path.join(fileName);
		
		let mut file = File::create(&path)
			.context(format!("Error opening file for writing: {}/{}", platform.to_lowercase(), fileName))?;
		
		file.write_all(buffer)
			.context(format!("Error writing to file: {}/{}", platform.to_lowercase(), fileName))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the RPCS3 settings data to file.
*/
pub fn saveSettings_Rpcs3(settings: &Rpcs3Settings) -> Result<()>
{
	return match getConfigDir(false)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, Rpcs3Settings::FileName, settings),
	};
}

pub fn saveUserData_RetroAchievements(user: &RetroAchievementsUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, RetroAchievementsUser::FileName, user),
	};
}

pub fn saveUserData_Rpcs3(user: &Rpcs3User) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, Rpcs3User::FileName, user),
	};
}

pub fn saveUserData_Steam(user: &SteamUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(error!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamUser::FileName, user),
	};
}

/**
Generic method to write a given type which implements `Serialize` to file.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `&'static str` File name with extension of the desired file.
- data: `&T` The data to be written.
*/
fn writeDataToFile<T>(directory: String, fileName: &'static str, data: &T) -> Result<()>
	where T: Serialize
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let file = File::create(&path)
		.context(format!(
			"Failed creating or truncating the file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	let buffer = BufWriter::new(file);
	
	serde_json::to_writer(buffer, data)
		.context(format!(
			"serde_json failed writing data to BufWriter at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	return Ok(());
}
