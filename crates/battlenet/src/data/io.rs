use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getConfigDir, getDataDir, readDataFromFile, readRawDataFromFile,
	writeDataToFile};
use tracing::warn;
use crate::api::BattleNetSettings;
use super::user::BattleNetUser;

/**
Read the Battle.Net settings data from file.
*/
pub fn loadSettings_BattleNet() -> BattleNetSettings
{
	return loadSettings()
		.unwrap_or_default();
}

/**
Read the Battle.Net user data from file.
*/
pub fn loadUserData_BattleNet() -> BattleNetUser
{
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading Battle.Net user data: {:?}", e);
			warn!("Attempting Battle.Net user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		},
		Ok(user) => user,
	};
}

/**
Read the Battle.Net settings data from file.
*/
fn loadSettings() -> Result<BattleNetSettings>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, BattleNetSettings::FileName.into()),
	};
}

/**
Read the Battle.Net user data from file.
*/
fn loadUserData() -> Result<BattleNetUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, BattleNetUser::FileName.into()),
	};
}

/**
Read the Battle.Net user data from file.
*/
fn loadUserData_lossy() -> Result<BattleNetUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, BattleNetUser::FileName.into())?;
			BattleNetUser::parseJsonLossy(json)
		},
	};
}

/**
Write the Battle.Net settings data to file.
*/
pub fn saveSettings(auth: &BattleNetSettings) -> Result<()>
{
	return match getConfigDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, BattleNetSettings::FileName.into(), auth),
	};
}

/**
Write the Battle.Net user data to file.
*/
pub fn saveUserData(user: &BattleNetUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, BattleNetUser::FileName.into(), user),
	};
}
