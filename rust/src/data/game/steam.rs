use ::serde::{Deserialize, Serialize};
use crate::platforms::steam::{SteamGame, SteamAchievementMetadata, SteamAchievementData};

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamInfo
{
	pub id: usize,
	pub iconHash: String,
	pub lastPlayed: usize,
	pub playtime: SteamPlaytime,
}

impl SteamInfo
{
	pub fn new(info: SteamGame) -> Self
	{
		let mut instance = Self::default();
		instance.update(info);
		return instance;
	}
	
	pub fn update(&mut self, info: SteamGame)
	{
		self.id = info.appid;
		self.iconHash = info.img_icon_url.to_owned();
		self.lastPlayed = info.rtime_last_played;
		self.playtime.update(info);
	}
}

/**

*/
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamPlaytime
{
	pub linux: usize,
	pub mac: usize,
	pub offline: usize,
	pub total: usize,
	pub windows: usize,
}

impl SteamPlaytime
{
	pub fn update(&mut self, info: SteamGame)
	{
		self.linux = info.playtime_linux_forever;
		self.mac = info.playtime_mac_forever;
		self.offline = info.playtime_disconnected;
		self.total = info.playtime_forever;
		self.windows = info.playtime_windows_forever;
	}
}

/**
Achievement data specific to the Steam platform.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamAchievement
{
	/// The human-readable description of this achievement.
	pub description: String,
	
	/// The percentage of users on this platform who have unlocked this achievement.
	pub globalPercentage: Option<f64>,
	
	/// The platform-specific ID of this achievement.
	pub id: String,
	
	/// The human-readable name of this achievement.
	pub name: String,
	
	/// The timestamp at which the achievement was unlocked.
	pub timestamp: Option<usize>,
}

impl SteamAchievement
{
	/**
	
	*/
	pub fn new(id: String, name: String, description: String) -> Self
	{
		return Self
		{
			description,
			globalPercentage: None,
			id,
			name,
			timestamp: None,
		}
	}
	
	pub fn update(&mut self, achievement: SteamAchievementData)
	{
		if achievement.unlocktime > 0
		{
			self.timestamp = Some(achievement.unlocktime * 1000);
		}
	}
	
	pub fn updateMetadata(&mut self, achievement: SteamAchievementMetadata)
	{
		self.description = match achievement.description
		{
			Some(d) => d,
			None => String::default(),
		};
		self.id = achievement.name.to_owned();
		self.name = achievement.displayName.to_owned();
	}
	
	/**
	Is this achievement unlocked on this platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.timestamp.is_some();
	}
}

impl From<SteamAchievementData> for SteamAchievement
{
	fn from(value: SteamAchievementData) -> Self
	{
		let mut instance = Self::new(
			value.apiname,
			value.name,
			value.description.unwrap_or_default()
		);
		
		if value.unlocktime > 0
		{
			instance.timestamp = Some(value.unlocktime * 1000);
		}
		
		return instance;
	}
}

impl From<SteamAchievementMetadata> for SteamAchievement
{
	fn from(value: SteamAchievementMetadata) -> Self
	{
		return Self::new(
			value.name,
			value.displayName,
			value.description.unwrap_or_default()
		);
	}
}


#[cfg(test)]
mod tests
{
    use super::*;
	
	fn setupAchievement(name: &str, unlockTime: Option<usize>) -> SteamAchievement
	{
		let achievement = SteamAchievement
		{
			description: String::default(),
			globalPercentage: None,
			id: String::default(),
			name: name.to_string(),
			timestamp: unlockTime,
		};
		
		return achievement;
	}
}
