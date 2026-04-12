use std::io::ErrorKind;
use anyhow::{Result, anyhow};
use data::io::{getDataDir, readDataFromFile, writeDataToFile};
use tracing::warn;
use super::user::GogUser;

pub fn loadUserData_Gog() -> GogUser
{
	return match loadUserData()
	{
		Err(e) => {
			warn!("Failed loading GOG user data: {:?}", e);
			warn!("Attempting GOG user data lossy load");
			loadUserData_lossy()
				.unwrap_or_default()
		}
		Ok(user) => user,
	};
}

/**
Read the GOG API user data from file.
*/
fn loadUserData() -> Result<GogUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, GogUser::FileName.into()),
	};
}

/**
Read the GOG API user data from file.
*/
fn loadUserData_lossy() -> Result<GogUser>
{
	return match getDataDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => {
			let json = readDataFromFile(dir, GogUser::FileName.into())?;
			GogUser::parseJsonLossy(json)
		},
	};
}

pub fn saveUserData(user: &GogUser) -> Result<()>
{
	return match getDataDir(true)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, GogUser::FileName.into(), user),
	};
}
