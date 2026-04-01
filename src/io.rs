use std::fs::{self, File, create_dir_all};
use std::io::{BufReader, BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::constants::{SecretsKeyFileName, SecretsVaultFileName};
//use crate::battlenet::{BattleNetAuth, BattleNetUser};
use crate::data::AppSettings;
//use crate::egs::{EgsSettings, EgsUser};
use crate::gog::GogUser;
use crate::net::limiter::request::FileLocation;
use crate::retroachievements::{RetroAchievementsAuth, RetroAchievementsUser};
use crate::rpcs3::{Rpcs3Settings, Rpcs3User};
use crate::steam::{SteamAuth, SteamUser};

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

pub const FileName_GameIcon: &str = "game-icon";
pub const FileName_LogPrefix: &str = "app.log";

pub const Path_Avatars: &str = "avatars";
pub const Path_Logs: &str = "logs";
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

pub fn getSecretsKeyPath() -> Option<PathBuf>
{
	return Some(Path::new(&getConfigDir(true)?)
		.join(SecretsKeyFileName));
}

pub fn getSecretsVaultPath() -> Option<PathBuf>
{
	return Some(Path::new(&getConfigDir(true)?)
		.join(SecretsVaultFileName));
}

pub fn imagePathExists(location: &FileLocation) -> bool
{
	return match getCacheDir(false)
	{
		None => false,
		Some(dir) => Path::new(dir.as_str())
			.join(location.platform.to_lowercase())
			.join(location.group.clone())
			.join(location.fileName.clone())
			.exists()
	};
}

/**
Get the absolute path to a cached image, if it exists.
*/
pub fn getImagePath(location: &FileLocation) -> Option<String>
{
	return Some(
		Path::new(getCacheDir(false)?.as_str())
			.join(location.platform.to_lowercase())
			.join(location.group.clone())
			.join(location.fileName.clone())
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
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, AppSettings::FileName.into()),
	};
}

/**
Read the Battle.Net API authorization data from file.
*/
/*
pub fn loadAuthData_BattleNet() -> Result<BattleNetAuth>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, BattleNetAuth::FileName.into()),
	};
}
*/

/**
Read the RetroAchievements API authorization data from file.
*/
pub fn loadAuthData_RetroAchievements() -> Result<RetroAchievementsAuth>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, RetroAchievementsAuth::FileName.into()),
	};
}

/**
Read the Steam API authorization data from file.
*/
pub fn loadAuthData_Steam() -> Result<SteamAuth>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, SteamAuth::FileName.into()),
	};
}

/**
Read the Epic Games Store settings data from file.
*/
/*
pub fn loadSettings_EpicGamesStore() -> Result<EgsSettings>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, EgsSettings::FileName.into()),
	};
}
*/

/**
Read the RPCS3 settings data from file.
*/
pub fn loadSettings_Rpcs3() -> Result<Rpcs3Settings>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, Rpcs3Settings::FileName.into()),
	};
}

/**
Read the Battle.Net user data from file.
*/
/*
pub fn loadUserData_BattleNet() -> Result<BattleNetUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, BattleNetUser::FileName.into()),
	};
}
*/

/**
Read the Battle.Net user data from file.
*/
/*
pub fn loadUserData_BattleNet_lossy() -> Result<BattleNetUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, BattleNetUser::FileName.into())?;
			BattleNetUser::parseJsonLossy(json)
		},
	};
}
*/

/**
Read the Epic Games Store user data from file.
*/
/*
pub fn loadUserData_EpicGamesStore() -> Result<EgsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, EgsUser::FileName.into()),
	};
}
*/

/**
Read the Epic Games Store user data from file.
*/
/*
pub fn loadUserData_EpicGamesStore_lossy() -> Result<EgsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, EgsUser::FileName.into())?;
			EgsUser::parseJsonLossy(json)
		},
	};
}
*/

/**
Read the GOG API user data from file.
*/
pub fn loadUserData_Gog() -> Result<GogUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, GogUser::FileName.into()),
	};
}

/**
Read the GOG API user data from file.
*/
pub fn loadUserData_Gog_lossy() -> Result<GogUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readDataFromFile(dir, GogUser::FileName.into())?;
			GogUser::parseJsonLossy(json)
		},
	};
}

/**
Read the RetroAchievements API user data from file.
*/
pub fn loadUserData_RetroAchievements() -> Result<RetroAchievementsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, RetroAchievementsUser::FileName.into()),
	};
}

/**
Read the RetroAchievements API user data from file.
*/
pub fn loadUserData_RetroAchievements_lossy() -> Result<RetroAchievementsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, RetroAchievementsUser::FileName.into())?;
			RetroAchievementsUser::parseJsonLossy(json)
		},
	};
}

/**
Read the RPCS3 user data from file.
*/
pub fn loadUserData_Rpcs3() -> Result<Rpcs3User>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, Rpcs3User::FileName.into()),
	};
}

/**
Read the Steam API user data from file.
*/
pub fn loadUserData_Rpcs3_lossy() -> Result<Rpcs3User>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, Rpcs3User::FileName.into())?;
			Rpcs3User::parseJsonLossy(json)
		},
	};
}

/**
Read the Steam API user data from file.
*/
pub fn loadUserData_Steam() -> Result<SteamUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, SteamUser::FileName.into()),
	};
}

/**
Read the Steam API user data from file.
*/
pub fn loadUserData_Steam_lossy() -> Result<SteamUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, SteamUser::FileName.into())?;
			SteamUser::parseJsonLossy(json)
		},
	};
}

/**
Generic method to read data from file and deserialize it into a given type which
implements `DeserializeOwned`.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
*/
fn readDataFromFile<T>(directory: String, fileName: String) -> Result<T>
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
Read data from a file.

This is primarily used to preserve data when deserializing to a specific type
fails.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
*/
fn readRawDataFromFile(directory: String, fileName: String) -> Result<String>
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let json = fs::read_to_string(&path)
		.context(format!(
			"Failed opening file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	return Ok(json);
}

/**
Write the application's settings data to file.
*/
pub fn saveAppSettings(settings: &AppSettings) -> Result<()>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, AppSettings::FileName.into(), settings),
	};
}

/**
Write the Battle.Net API authorization data to file.
*/
/*
pub fn saveAuthData_BattleNet(auth: &BattleNetAuth) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, BattleNetAuth::FileName.into(), auth),
	};
}
*/

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn saveAuthData_RetroAchievements(auth: &RetroAchievementsAuth) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, RetroAchievementsAuth::FileName.into(), auth),
	};
}

/**
Write the Steam API authorization data to file.
*/
pub fn saveAuthData_Steam(auth: &SteamAuth) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamAuth::FileName.into(), auth),
	};
}

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn saveImageToCache(
		destination: &FileLocation,
		buffer: &[u8]
	) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let mut path = Path::new(dir.as_str())
			.join(destination.platform.to_lowercase())
			.join(destination.group.clone());
		
		if !path.exists()
		{
			_ = create_dir_all(&path);
		}
		
		path = path.join(destination.fileName.clone());
		
		let mut file = File::create(&path)
			.context(format!(
				"Error opening file for writing: {}/{}",
				destination.platform.to_lowercase(),
				destination.fileName
			))?;
		
		file.write_all(buffer)
			.context(format!(
				"Error writing to file: {}/{}",
				destination.platform.to_lowercase(),
				destination.fileName
			))?;
		
		return Ok(());
	}
	
	return Err(anyhow!(ErrorKind::NotFound));
}

/**
Write the Epic Games Store settings data to file.
*/
/*
pub fn saveSettings_EpicGamesStore(settings: &EgsSettings) -> Result<()>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, EgsSettings::FileName.into(), settings),
	};
}
*/

/**
Write the RPCS3 settings data to file.
*/
pub fn saveSettings_Rpcs3(settings: &Rpcs3Settings) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, Rpcs3Settings::FileName.into(), settings),
	};
}

/*
pub fn saveUserData_BattleNet(user: &BattleNetUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, BattleNetUser::FileName.into(), user),
	};
}
*/

/*
pub fn saveUserData_EpicGamesStore(user: &EgsUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, EgsUser::FileName.into(), user),
	};
}
*/

pub fn saveUserData_Gog(user: &GogUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, GogUser::FileName.into(), user),
	};
}

pub fn saveUserData_RetroAchievements(user: &RetroAchievementsUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, RetroAchievementsUser::FileName.into(), user),
	};
}

pub fn saveUserData_Rpcs3(user: &Rpcs3User) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, Rpcs3User::FileName.into(), user),
	};
}

pub fn saveUserData_Steam(user: &SteamUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamUser::FileName.into(), user),
	};
}

/**
Generic method to write a given type which implements `Serialize` to file.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
- data: `&T` The data to be written.
*/
fn writeDataToFile<T>(directory: String, fileName: String, data: &T) -> Result<()>
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
