use anyhow::Result;
use data::Secrets;
use crate::api::{BattleNetAuth, BattleNetSession};

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

#[allow(unused)]
pub fn removeBattleNetClientId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(BattleNetAuth::ClientIdKey)?;
	_ = secrets.save()?;
	return Ok(());
}

#[allow(unused)]
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
