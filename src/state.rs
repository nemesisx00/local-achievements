#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;
use ::anyhow::{Context, Result};
use ::fermi::AtomRef;
use crate::error;
use crate::data::User;
use crate::io::{getConfigDir, getDataDir};
use crate::platforms::retroachievements::AuthObject;
use crate::platforms::steam::AuthData;

pub static RetroAchievementsAuth: AtomRef<AuthObject> = AtomRef(|_| match readAuth_RetroAchievements() { Ok(auth) => auth, Err(_) => AuthObject::default() });
pub static SteamAuth: AtomRef<AuthData> = AtomRef(|_| match readAuth_Steam() { Ok(auth) => auth, Err(_) => AuthData::default() });
pub static User: AtomRef<User> = AtomRef(|_| match loadUserData() { Ok(user) => user, Err(_) => User::default() });

/**

*/
pub fn loadUserData() -> Result<User>
{
	if let Some(dir) = getDataDir(false)
	{
		let path = Path::new(dir.as_str()).join(User::Filename);
		let file = File::open(&path.as_path())
			.context(format!("Failed opening file at: '{}'", path.as_path().to_str().unwrap()))?;
		let buffer = BufReader::new(file);
		let instance: User = serde_json::from_reader(buffer)
			.context("Failed parsing User data file as JSON!")?;
		return Ok(instance);
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
		let file = File::create(&path.as_path())
			.context(format!("Failed creating or truncating the file at: '{}'", path.as_path().to_str().unwrap()))?;
		let buffer = BufWriter::new(file);
		serde_json::to_writer(buffer, &user)
			.context("serde_json failed writing User data to BufWriter")?;
		return Ok(());
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Read the RetroAchievements API authorization data from file.
*/
pub fn readAuth_RetroAchievements() -> Result<AuthObject>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(AuthObject::FileName);
		let file = File::open(path.as_path())
			.context(format!("Failed opening file at: '{}'", path.as_path().to_str().unwrap()))?;
		let reader = BufReader::new(file);
		let instance = serde_json::from_reader(reader)
			.context("Failed parsing RetroAchievements AuthObject file as JSON")?;
		return Ok(instance);
	}
	
	return Err(error!(ErrorKind::NotFound));
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
		let instance = serde_json::from_reader(reader)
			.context("Failed parsing Steam AuthData file as JSON")?;
		return Ok(instance);
	}
	
	return Err(error!(ErrorKind::NotFound));
}

/**
Write the RetroAchievements API authorization data to file.
*/
pub fn writeAuth_RetroAchievements(auth: AuthObject) -> Result<()>
{
	if let Some(dir) = getConfigDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(AuthObject::FileName);
		let file = File::create(path.as_path())
			.context(format!("Failed creating or truncating the file at: '{}'", path.as_path().to_str().unwrap()))?;
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
