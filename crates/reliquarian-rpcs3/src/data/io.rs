use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getDataDir, getSettingsDir, readDataFromFile,
	readRawDataFromFile, writeDataToFile};
use tracing::warn;
use crate::data::settings::Rpcs3Settings;
use crate::data::user::Rpcs3User;

pub fn loadSettings_Rpcs3() -> Rpcs3Settings
{
	return loadSettings()
		.unwrap_or_default();
}

pub fn loadUserData_Rpcs3() -> Rpcs3User
{	
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading RPCS3 user data: {:?}", e);
			warn!("Attempting RPCS3 user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		},
		Ok(user) => user,
	};
}

/**
Read the RPCS3 settings data from file.
*/
fn loadSettings() -> Result<Rpcs3Settings>
{
	return match getSettingsDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(
			dir,
			Rpcs3Settings::FileName.into()
		),
	};
}

/**
Read the RPCS3 user data from file.
*/
fn loadUserData() -> Result<Rpcs3User>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(
			dir,
			Rpcs3User::FileName.into()
		),
	};
}

/**
Read the RPCS3 API user data from file.
*/
fn loadUserData_lossy() -> Result<Rpcs3User>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(
				dir,
				Rpcs3User::FileName.into()
			)?;
			
			Rpcs3User::parseJsonLossy(json)
		},
	};
}

/**
Write the RPCS3 settings data to file.
*/
pub fn saveSettings(settings: &Rpcs3Settings) -> Result<()>
{
	return match getSettingsDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(
			dir,
			Rpcs3Settings::FileName.into(),
			settings
		),
	};
}

pub fn saveUserData(user: &Rpcs3User) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(
			dir,
			Rpcs3User::FileName.into(),
			user
		),
	};
}
