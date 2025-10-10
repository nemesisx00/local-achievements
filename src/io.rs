use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use crate::data::{RetroAchievementsUser, SteamUser};
use crate::error;
use crate::platforms::retroachievements::data::RetroAchievementsAuth;
use crate::platforms::steam::SteamAuth;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

pub const Filename_GameIcon: &str = "game-icon";

pub const Path_Avatars: &str = "avatars";
pub const Path_Games: &str = "games";

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
Read the RetroAchievements API authorization data from file.
*/
pub fn loadAuthData_RetroAchievements() -> Result<RetroAchievementsAuth>
{
	if let Some(dir) = getConfigDir(false)
	{
		let path = Path::new(dir.as_str())
			.join(RetroAchievementsAuth::FileName);
		
		let file = File::open(&path)
			.context(format!(
				"Failed opening file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let reader = BufReader::new(file);
		
		return Ok(serde_json::from_reader(reader)
			.context("Failed parsing RetroAchievementsAuth file as JSON")?);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the Steam API authorization data from file.
*/
pub fn loadAuthData_Steam() -> Result<SteamAuth>
{
	if let Some(dir) = getConfigDir(false)
	{
		let path = Path::new(dir.as_str())
			.join(SteamAuth::FileName);
		
		let file = File::open(&path)
			.context(format!(
				"Failed opening file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let reader = BufReader::new(file);
		
		return Ok(serde_json::from_reader(reader)
			.context("Failed parsing SteamAuth file as JSON")?);
	}
	
	return Err(error!(ErrorKind::NotFound));
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
Read the Steam API user data from file.
*/
pub fn loadUserData_Steam() -> Result<SteamUser>
{
	if let Some(dir) = getDataDir(false)
	{
		let path = Path::new(dir.as_str()).join(SteamUser::Filename);
		
		let file = File::open(&path)
			.context(format!(
				"Failed opening file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let buffer = BufReader::new(file);
		
		return Ok(serde_json::from_reader(buffer)
			.context("Failed parsing User data file as JSON")?);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the RetroAchievements API user data from file.
*/
pub fn loadUserData_RetroAchievements() -> Result<RetroAchievementsUser>
{
	if let Some(dir) = getDataDir(false)
	{
		let path = Path::new(dir.as_str()).join(RetroAchievementsUser::Filename);
		
		let file = File::open(&path)
			.context(format!(
				"Failed opening file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let buffer = BufReader::new(file);
		
		return Ok(serde_json::from_reader(buffer)
			.context("Failed parsing User data file as JSON")?);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn saveAuthData_RetroAchievements(auth: &RetroAchievementsAuth) -> Result<()>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(RetroAchievementsAuth::FileName);
		
		let file = File::create(&path)
			.context(format!(
				"Failed creating or truncating the file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let writer = BufWriter::new(file);
		
		serde_json::to_writer(writer, auth)
			.context(format!(
				"serde_json failed writing RetroAchievementsAuth to BufWriter at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the Steam API authorization data to file.
*/
pub fn saveAuthData_Steam(auth: &SteamAuth) -> Result<()>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(SteamAuth::FileName);
		
		let file = File::create(&path)
			.context(format!(
				"Failed creating or truncating the file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let writer = BufWriter::new(file);
		
		serde_json::to_writer(writer, auth)
			.context(format!(
				"serde_json failed writing SteamAuth to BufWriter at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
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
			.join(group.to_lowercase());
		
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

pub fn saveUserData_RetroAchievements(user: &RetroAchievementsUser) -> Result<()>
{
	if let Some(dir) = getDataDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(RetroAchievementsUser::Filename);
		
		let file = File::create(&path)
			.context(format!(
				"Failed creating or truncating the file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let buffer = BufWriter::new(file);
		
		serde_json::to_writer(buffer, user)
			.context(format!(
				"serde_json failed writing User data to BufWriter at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

pub fn saveUserData_Steam(user: &SteamUser) -> Result<()>
{
	if let Some(dir) = getDataDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(SteamUser::Filename);
		
		let file = File::create(&path)
			.context(format!(
				"Failed creating or truncating the file at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		let buffer = BufWriter::new(file);
		
		serde_json::to_writer(buffer, user)
			.context(format!(
				"serde_json failed writing User data to BufWriter at: '{}'",
				path.as_path().to_str().unwrap()
			))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}
