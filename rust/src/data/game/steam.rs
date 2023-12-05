use std::collections::HashMap;
use ::godot::builtin::{Array, Dictionary, Variant};
use ::godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};
use ::serde::{Deserialize, Serialize};
use crate::platforms::steam::{SteamGame, SteamAchievementMetadata, SteamAchievementData};
use crate::{readVariant, readVariantOption};

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamPlatform
{
	/// Information specific to the Platform.
	pub info: SteamInfo,
	/// The list of achievements associated with this game.
	pub achievements: Vec<SteamAchievement>,
}

impl FromGodot for SteamPlatform
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::fromDict(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		return Self::fromVariant(variant);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_godot(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_variant(variant));
	}
}

impl GodotConvert for SteamPlatform
{
	type Via = Dictionary;
}

impl ToGodot for SteamPlatform
{
	fn into_godot(self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_variant(&self) -> Variant
	{
		return self.buildDict().to_variant();
	}
}

impl SteamPlatform
{
	pub fn isGlobalPercentageMissing(&self) -> bool
	{
		return self.achievements.iter()
			.any(|a| a.globalPercentage.is_none());
	}
	
	pub fn updateAchievements(&mut self, achievements: Vec<SteamAchievementData>)
	{
		for achievement in achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id == achievement.apiname)
			{
				Some(chievo) => chievo.update(achievement),
				None => self.achievements.push(SteamAchievement::from(achievement)),
			}
		}
	}
	
	pub fn updateAchievementMetadata(&mut self, achievements: Vec<SteamAchievementMetadata>)
	{
		for metadata in achievements
		{
			match self.achievements.iter_mut()
				.find(|a| a.id == metadata.name)
			{
				Some(chievo) => chievo.updateMetadata(metadata),
				None => self.achievements.push(SteamAchievement::from(metadata)),
			}
		}
	}
	
	pub fn updateGlobalPercentages(&mut self, percentages: HashMap<String, f64>)
	{
		for (id, percentage) in percentages
		{
			if let Some(achievement) = self.achievements.iter_mut()
				.find(|a| a.id == id)
			{
				achievement.globalPercentage = Some(percentage);
			}
		}
	}
	
	fn buildDict(&self) -> Dictionary
	{
		let mut arr = Array::new();
		for a in &self.achievements
		{
			arr.push(a.to_godot());
		}
		
		let mut dict = Dictionary::new();
		dict.insert("info", self.info.to_godot());
		dict.insert("achievements", arr);
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let mut achievements = vec![];
		let arr = readVariant!(dict.get("achievements"), Array::<Variant>);
		for v in arr.iter_shared()
		{
			let a = SteamAchievement::from_variant(&v);
			achievements.push(a);
		}
		
		let info = readVariant!(dict.get("info"), SteamInfo);
		
		return Self
		{
			info,
			achievements,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamInfo
{
	pub id: i64,
	pub iconHash: String,
	pub lastPlayed: i64,
	pub playtime: SteamPlaytime,
}

impl FromGodot for SteamInfo
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::fromDict(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		return Self::fromVariant(variant);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_godot(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_variant(variant));
	}
}

impl GodotConvert for SteamInfo
{
	type Via = Dictionary;
}

impl ToGodot for SteamInfo
{
	fn into_godot(self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_variant(&self) -> Variant
	{
		return self.buildDict().to_variant();
	}
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
	
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("id", self.id);
		dict.insert("iconHash", self.iconHash.to_godot());
		dict.insert("lastPlayed", self.lastPlayed);
		dict.insert("playtime", self.playtime);
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let id = readVariant!(dict.get("id"), i64);
		let iconHash = readVariant!(dict.get("iconHash"), String);
		let lastPlayed = readVariant!(dict.get("lastPlayed"), i64);
		let playtime = readVariant!(dict.get("playtime"), SteamPlaytime);
		
		return Self
		{
			id,
			iconHash,
			lastPlayed,
			playtime,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}

/**

*/
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamPlaytime
{
	pub linux: i64,
	pub mac: i64,
	pub offline: i64,
	pub total: i64,
	pub windows: i64,
}

impl FromGodot for SteamPlaytime
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::fromDict(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		return Self::fromVariant(variant);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_godot(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_variant(variant));
	}
}

impl GodotConvert for SteamPlaytime
{
	type Via = Dictionary;
}

impl ToGodot for SteamPlaytime
{
	fn into_godot(self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_variant(&self) -> Variant
	{
		return self.buildDict().to_variant();
	}
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
	
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("linux", self.linux);
		dict.insert("mac", self.mac);
		dict.insert("offline", self.offline);
		dict.insert("total", self.total);
		dict.insert("windows", self.windows);
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let linux = readVariant!(dict.get("linux"), i64);
		let mac = readVariant!(dict.get("mac"), i64);
		let offline = readVariant!(dict.get("offline"), i64);
		let total = readVariant!(dict.get("total"), i64);
		let windows = readVariant!(dict.get("windows"), i64);
		
		return Self
		{
			linux,
			mac,
			offline,
			total,
			windows,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
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
	pub timestamp: Option<i64>,
}

impl FromGodot for SteamAchievement
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::fromDict(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		return Self::fromVariant(variant);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_godot(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::from_variant(variant));
	}
}

impl GodotConvert for SteamAchievement
{
	type Via = Dictionary;
}

impl ToGodot for SteamAchievement
{
	fn into_godot(self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return self.buildDict();
	}
	
	fn to_variant(&self) -> Variant
	{
		return self.buildDict().to_variant();
	}
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
	
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("description", self.description.to_godot());
		dict.insert("globalPercentage", match self.globalPercentage { Some(f) => f, None => 0.0 });
		dict.insert("id", self.id.to_godot());
		dict.insert("name", self.name.to_godot());
		dict.insert("timestamp", match self.timestamp { Some(f) => f, None => 0 });
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let description = readVariant!(dict.get("description"), String);
		let globalPercentage = readVariantOption!(dict.get("globalPercentage"), f64);
		let id = readVariant!(dict.get("id"), String);
		let name = readVariant!(dict.get("name"), String);
		let timestamp = readVariantOption!(dict.get("timestamp"), i64);
		
		return Self
		{
			description,
			globalPercentage,
			id,
			name,
			timestamp,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
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
	
	fn setupAchievement(name: &str, unlockTime: Option<i64>) -> SteamAchievement
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
