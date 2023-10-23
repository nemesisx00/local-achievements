#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::create_dir_all;
use ::directories::ProjectDirs;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

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
