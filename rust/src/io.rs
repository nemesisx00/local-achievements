use std::fs::{create_dir_all, File};
use std::io::{ErrorKind, Write, BufReader, BufWriter};
use std::path::Path;
use ::anyhow::{Context, Result};
use ::directories::ProjectDirs;
use crate::data::User;
use crate::error;
use crate::platforms::retroachievements::RetroAchievementsAuth;
use crate::platforms::steam::SteamAuth;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

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
		let _ = create_dir_all(&path);
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
		let _ = create_dir_all(path.clone());
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
		let _ = create_dir_all(path.clone());
	}
	
	return Some(path);
}

/**
Get the absolute path to a cached image, if it exists.
*/
pub fn getImagePath(platform: String, group: String, fileName: String) -> Option<String>
{
	return Some(Path::new(getCacheDir(false)?.as_str())
		.join(platform.to_lowercase())
		.join(group)
		.join(fileName)
		.to_str()?
		.into());
}

/**

*/
pub fn loadUserData() -> Result<User>
{
	if let Some(dir) = getDataDir(false)
	{
		let path = Path::new(dir.as_str()).join(User::Filename);
		let file = File::open(&path)
			.context(format!("Failed opening file at: '{}'", path.as_path().to_str().unwrap()))?;
		let buffer = BufReader::new(file);
		let instance: User = serde_json::from_reader(buffer)
			.context("Failed parsing User data file as JSON!")?;
		return Ok(instance);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the RetroAchievements API authorization data from file.
*/
pub fn readAuth_RetroAchievements() -> Result<RetroAchievementsAuth>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(RetroAchievementsAuth::FileName);
		let file = File::open(&path)
			.context(format!("Failed opening file at: '{}'", path.as_path().to_str().unwrap()))?;
		let reader = BufReader::new(file);
		let instance = serde_json::from_reader(reader)
			.context("Failed parsing RetroAchievementsAuth file as JSON")?;
		return Ok(instance);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the Steam API authorization data from file.
*/
pub fn readAuth_Steam() -> Result<SteamAuth>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(SteamAuth::FileName);
		let file = File::open(&finalPath)
			.context(format!("Failed opening file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let reader = BufReader::new(file);
		let instance = serde_json::from_reader(reader)
			.context("Failed parsing SteamAuth file as JSON")?;
		return Ok(instance);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn saveImageToCache(platform: String, group: String, fileName: String, buffer: &[u8]) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let mut path = Path::new(dir.as_str())
			.join(platform.to_lowercase())
			.join(group.to_lowercase());
		
		if !path.exists()
		{
			let _ = create_dir_all(&path);
		}
		
		path = path.join(&fileName);
		
		let mut file = File::create(&path)
			.context(format!("Error opening file for writing: {}/{}", platform.to_lowercase(), fileName))?;
		file.write_all(buffer)
			.context(format!("Error writing to file: {}/{}", platform.to_lowercase(), fileName))?;
		
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**

*/
pub fn saveUserData(user: User) -> Result<()>
{
	if let Some(dir) = getDataDir(true)
	{
		let path = Path::new(dir.as_str()).join(User::Filename);
		let file = File::create(&path)
			.context(format!("Failed creating or truncating the file at: '{}'", path.as_path().to_str().unwrap()))?;
		let buffer = BufWriter::new(file);
		serde_json::to_writer(buffer, &user)
			.context("serde_json failed writing User data to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn writeAuth_RetroAchievements(auth: RetroAchievementsAuth) -> Result<()>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(RetroAchievementsAuth::FileName);
		let file = File::create(&path)
			.context(format!("Failed creating or truncating the file at: '{}'", path.as_path().to_str().unwrap()))?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)
			.context("serde_json failed writing RetroAchievementsAuth to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the Steam API authorization data to file.
*/
pub fn writeAuth_Steam(auth: SteamAuth) -> Result<()>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(SteamAuth::FileName);
		let file = File::create(&finalPath)
			.context(format!("Failed creating or truncating the file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)
			.context("serde_json failed writing SteamAuth to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}
