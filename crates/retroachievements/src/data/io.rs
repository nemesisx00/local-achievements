use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getDataDir, readDataFromFile, readRawDataFromFile,
	writeDataToFile};
use tracing::warn;
use crate::data::user::RetroAchievementsUser;

pub fn loadUserData_RetroAchievements() -> RetroAchievementsUser
{
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading RetroAchievements user data: {:?}", e);
			warn!("Attempting RetroAchievements user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		},
		Ok(user) => user,
	};
}

/**
Read the RetroAchievements API user data from file.
*/
fn loadUserData() -> Result<RetroAchievementsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, RetroAchievementsUser::FileName.into()),
	};
}

/**
Read the RetroAchievements API user data from file.
*/
fn loadUserData_lossy() -> Result<RetroAchievementsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, RetroAchievementsUser::FileName.into())?;
			RetroAchievementsUser::parseJsonLossy(json)
		},
	};
}

pub fn saveUserData(user: &RetroAchievementsUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, RetroAchievementsUser::FileName.into(), user),
	};
}
