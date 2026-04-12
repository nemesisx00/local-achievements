use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getDataDir, readDataFromFile, readRawDataFromFile,
	writeDataToFile};
use tracing::warn;
use super::user::EgsUser;

pub fn loadUserData_EpicGamesStore() -> EgsUser
{
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading Epic Games Store user data: {:?}", e);
			warn!("Attempting Epic Games Store user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		},
		Ok(user) => user,
	};
}

/**
Read the Epic Games Store user data from file.
*/
fn loadUserData() -> Result<EgsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, EgsUser::FileName.into()),
	};
}

/**
Read the Epic Games Store user data from file.
*/
fn loadUserData_lossy() -> Result<EgsUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readRawDataFromFile(dir, EgsUser::FileName.into())?;
			EgsUser::parseJsonLossy(json)
		},
	};
}

pub fn saveUserData(user: &EgsUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, EgsUser::FileName.into(), user),
	};
}
