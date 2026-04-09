#![allow(unused)]

use anyhow::Result;
use crate::Secrets;
use crate::battlenet::{BattleNetAuth, BattleNetSession};
use crate::egs::EgsSettings;
use crate::gog::GogSession;
use crate::retroachievements::RetroAchievementsAuth;
use crate::steam::SteamAuth;

// ----- Get

pub fn getBattleNetClientAuth() -> Result<BattleNetAuth>
{
	let secrets = Secrets.blocking_lock();
	let id = secrets.get(BattleNetAuth::ClientIdKey)?;
	let key = secrets.get(BattleNetAuth::ClientSecretKey)?;
	return Ok(BattleNetAuth::new(id, key));
}

pub fn getBattleNetSession() -> Result<BattleNetSession>
{
	let secrets = Secrets.blocking_lock();
	let json = secrets.get(BattleNetSession::SecretKey)?;
	return Ok(serde_json::from_str(&json)?);
}

pub fn getEpicGamesStoreAccountId() -> Result<String>
{
	let secrets = Secrets.blocking_lock();
	let id = secrets.get(EgsSettings::AccountIdKey)?;
	return Ok(id);
}

pub fn getGogSession() -> Result<GogSession>
{
	let secrets = Secrets.blocking_lock();
	let json = secrets.get(GogSession::SecretKey)?;
	return Ok(serde_json::from_str(&json)?);
}

pub fn getRetroAchievementsAuth() -> Result<RetroAchievementsAuth>
{
	let secrets = Secrets.blocking_lock();
	let key = secrets.get(RetroAchievementsAuth::ApiKeySecretKey)?;
	let username = secrets.get(RetroAchievementsAuth::UsernameSecretKey)?;
	return Ok(RetroAchievementsAuth::new(key, username));
}

pub fn getSteamAuth() -> Result<SteamAuth>
{
	let secrets = Secrets.blocking_lock();
	let id = secrets.get(SteamAuth::UserIdSecretKey)?;
	let key = secrets.get(SteamAuth::ApiKeySecretKey)?;
	return Ok(SteamAuth::new(id, key));
}

// ----- Remove

pub fn removeBattleNetClientId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(BattleNetAuth::ClientIdKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeBattleNetClientSecret() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(BattleNetAuth::ClientSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeBattleNetSession() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(BattleNetSession::SecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeEpicGamesStoreAccountId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(EgsSettings::AccountIdKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeGogSession() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(GogSession::SecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeRetroAchievementsApiKey() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(RetroAchievementsAuth::ApiKeySecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeRetroAchievementsUsername() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(RetroAchievementsAuth::UsernameSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeSteamId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(SteamAuth::UserIdSecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

pub fn removeSteamApiKey() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(SteamAuth::ApiKeySecretKey)?;
	_ = secrets.save()?;
	return Ok(());
}

// ----- Set

pub fn setBattleNetClientId(id: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(BattleNetAuth::ClientIdKey, id);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setBattleNetClientSecret(secret: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(BattleNetAuth::ClientSecretKey, secret);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setBattleNetSession(session: BattleNetSession) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(BattleNetSession::SecretKey, serde_json::to_string(&session)?);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setEpicGamesStoreAccountId(id: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(EgsSettings::AccountIdKey, id);
	_ = secrets.save()?;
	return Ok(());
}

pub fn setGogSession(session: GogSession) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(GogSession::SecretKey, serde_json::to_string(&session)?);
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

pub fn setSteamId(id: String) -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.set(SteamAuth::UserIdSecretKey, id);
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
