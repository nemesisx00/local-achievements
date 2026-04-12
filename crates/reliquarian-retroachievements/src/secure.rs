use anyhow::Result;
use data::Secrets;
use crate::api::RetroAchievementsAuth;

pub fn getRetroAchievementsAuth() -> Result<RetroAchievementsAuth>
{
	let secrets = Secrets.blocking_lock();
	let key = secrets.get(RetroAchievementsAuth::ApiKeySecretKey)?;
	let username = secrets.get(RetroAchievementsAuth::UsernameSecretKey)?;
	return Ok(RetroAchievementsAuth::new(key, username));
}

#[allow(unused)]
pub fn removeRetroAchievementsApiKey() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(RetroAchievementsAuth::ApiKeySecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

#[allow(unused)]
pub fn removeRetroAchievementsUsername() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(RetroAchievementsAuth::UsernameSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn setRetroAchievementsApiKey(key: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(RetroAchievementsAuth::ApiKeySecretKey, key);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setRetroAchievementsUsername(username: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(RetroAchievementsAuth::UsernameSecretKey, username);
	_ = secrets.save()?;
	return Ok(());
}
