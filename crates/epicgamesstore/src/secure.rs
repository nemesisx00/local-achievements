use anyhow::Result;
use data::Secrets;
use crate::data::settings::EgsSettings;

pub fn getEpicGamesStoreAccountId() -> Result<String>
{
	let secrets = Secrets.blocking_lock();
	let id = secrets.get(EgsSettings::AccountIdKey)?;
	return Ok(id);
}

#[allow(unused)]
pub fn removeEpicGamesStoreAccountId() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(EgsSettings::AccountIdKey)?;
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
