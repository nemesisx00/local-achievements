#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::{create_dir_all, File};
use std::io::{ErrorKind, Write};
use std::path::Path;
use ::anyhow::{Context, Result};
use ::directories::ProjectDirs;
use crate::error;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

pub const Path_Avatars: &str = "avatars";
pub const Path_Games: &str = "games";

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn cacheImage(platform: String, group: String, fileName: String, buffer: &[u8]) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let mut path = Path::new(dir.as_str())
			.join(platform.to_lowercase())
			.join(group);
		
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
