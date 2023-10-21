#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;
use ::anyhow::{Context, Error, Result};
use ::directories::ProjectDirs;
use crate::error;
use crate::platforms::retroachievements::AuthObject;
use crate::platforms::steam::AuthData;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

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
Get the cache directory specific to this application.
*/
pub fn getCacheDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.cache_dir().to_str()?.to_string();
	
	if create
	{
		let _ = create_dir_all(path.clone());
	}
	
	return Some(path);
}

/**
Read the RetroAchievements API authorization data from file.
*/
pub fn readAuth_RetroAchievements() -> Result<AuthObject>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(AuthObject::FileName);
		let file = File::open(finalPath.as_path())
			.context(format!("Failed opening file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let reader = BufReader::new(file);
		let ret = serde_json::from_reader(reader)
			.context("Failed parsing RetroAchievements AuthObject file as JSON")?;
		return Ok(ret);
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}

/**
Read the Steam API authorization data from file.
*/
pub fn readAuth_Steam() -> Result<AuthData>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(AuthData::FileName);
		let file = File::open(finalPath.as_path())
			.context(format!("Failed opening file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let reader = BufReader::new(file);
		let ret = serde_json::from_reader(reader)
			.context("Failed parsing Steam AuthData file as JSON")?;
		return Ok(ret);
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn writeAuth_RetroAchievements(auth: AuthObject) -> Result<()>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(AuthObject::FileName);
		let file = File::create(finalPath.as_path())
			.context(format!("Failed creating or truncating the file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)
			.context("serde_json failed writing RetroAchievements AuthObject to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the Steam API authorization data to file.
*/
pub fn writeAuth_Steam(auth: AuthData) -> Result<()>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str())
			.join(AuthData::FileName);
		let file = File::create(finalPath.as_path())
			.context(format!("Failed creating or truncating the file at: '{}'", finalPath.as_path().to_str().unwrap()))?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)
			.context("serde_json failed writing Steam AuthData to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}
