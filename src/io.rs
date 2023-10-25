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

pub const Path_Images: &str = "/images/";

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn cacheImage(platform: String, fileName: String, buffer: &[u8]) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(Path_Images)
			.join(format!("/{}/", platform.to_lowercase()))
			.join(fileName);
		
		let mut file = File::create(path)
			.context("")?;
		file.write_all(buffer)
			.context("")?;
		
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
		let _ = create_dir_all(path.clone());
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
