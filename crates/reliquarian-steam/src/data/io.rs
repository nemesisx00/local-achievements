use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getConfigDir, getDataDir, readDataFromFile, readRawDataFromFile,
	writeDataToFile};
use tracing::warn;
use crate::data::settings::SteamSettings;
use crate::data::user::SteamUser;

/**
Read the Steam settings data from file.
*/
pub fn loadSettings_Steam() -> SteamSettings
{
	return loadSettings()
		.unwrap_or_default();
}

/**
Read the Steam settings data from file.
*/
fn loadSettings() -> Result<SteamSettings>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, SteamSettings::FileName.into()),
	};
}

/**
Read the Steam API user data from file.
*/
fn loadUserData() -> Result<SteamUser>
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
fn loadUserData_lossy() -> Result<SteamUser>
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

pub fn loadUserData_Steam() -> SteamUser
{
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading Steam user data: {:?}", e);
			warn!("Attempting Steam user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		},
		Ok(user) => user,
	};
}

/**
Write the Steam settings data to file.
*/
pub fn saveSettings(settings: &SteamSettings) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamSettings::FileName.into(), settings),
	};
}

/**
Write the Steam user data to file.
*/
pub fn saveUserData(user: &SteamUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, SteamUser::FileName.into(), user),
	};
}
