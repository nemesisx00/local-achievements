use anyhow::Result;
use crate::Secrets;
use crate::gog::GogSession;

pub fn getGogSession() -> Result<GogSession>
{
	let secrets = Secrets.blocking_lock();
	let json = secrets.get(GogSession::SecretKey)?;
	return Ok(serde_json::from_str(&json)?);
}

pub fn removeGogSession() -> Result<()>
{
	let mut secrets = Secrets.blocking_lock();
	secrets.remove(GogSession::SecretKey)?;
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
