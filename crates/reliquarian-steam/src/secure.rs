use anyhow::Result;
use data::Secrets;
use crate::api::{SteamApi, SteamAuth};

pub fn getSteamAuth() -> Result<SteamAuth>
{
	let secrets = Secrets.blocking_lock();
	let id = secrets.get(SteamAuth::UserIdSecretKey)?;
	let key = secrets.get(SteamAuth::ApiKeySecretKey)?;
	return Ok(SteamAuth::new(id, key));
}

pub fn getSteamWebToken() -> Result<String>
{
	let secrets = Secrets.blocking_lock();
	let token = secrets.get(SteamApi::WebTokenSecretKey)?;
	return Ok(token);
}

#[allow(unused)]
pub fn removeSteamApiKey() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(SteamAuth::ApiKeySecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeSteamWebToken() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(SteamApi::WebTokenSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

#[allow(unused)]
pub fn removeSteamId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(SteamAuth::UserIdSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn setSteamApiKey(key: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(SteamAuth::ApiKeySecretKey, key);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setSteamWebToken(token: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(SteamApi::WebTokenSecretKey, token);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setSteamId(id: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(SteamAuth::UserIdSecretKey, id);
	_ = secrets.save()?;
	return Ok(());
}
