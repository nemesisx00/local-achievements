#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;
use ::anyhow::{Error, Result};
use ::directories::ProjectDirs;
use crate::platforms::retroachievements::AuthObject;
use crate::platforms::steam::AuthData;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

pub fn getConfigDir(create: bool) -> Option<String>
{
	let mut path = None;
	if let Some(dirs) = ProjectDirs::from(Qualifier, Organization, Application)
	{
		let pathStr = dirs.config_dir().to_str().unwrap().to_string();
		if create
		{
			let _ = create_dir_all(pathStr.clone());
		}
		path = Some(pathStr);
	}
	
	return path;
}

pub fn readAuth_RetroAchievements() -> Result<AuthObject>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str()).join(AuthObject::FileName);
		let file = File::open(finalPath.as_path())?;
		let reader = BufReader::new(file);
		let ret = serde_json::from_reader(reader)?;
		return Ok(ret);
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}

pub fn readAuth_Steam() -> Result<AuthData>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str()).join(AuthData::FileName);
		let file = File::open(finalPath.as_path())?;
		let reader = BufReader::new(file);
		let ret = serde_json::from_reader(reader)?;
		return Ok(ret);
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}

pub fn writeAuth_RetroAchievements(auth: AuthObject) -> Result<()>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str()).join(AuthObject::FileName);
		let file = File::create(finalPath.as_path())?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)?;
		return Ok(());
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}

pub fn writeAuth_Steam(auth: AuthData) -> Result<()>
{
	if let Some(path) = getConfigDir(true)
	{
		let finalPath = Path::new(path.as_str()).join(AuthData::FileName);
		let file = File::create(finalPath.as_path())?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &auth)?;
		return Ok(());
	}
	
	return Err(Error::from(std::io::Error::from(ErrorKind::NotFound)));
}
