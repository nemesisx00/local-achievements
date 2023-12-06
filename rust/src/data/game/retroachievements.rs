use std::collections::HashMap;
use ::godot::builtin::{Array, Dictionary, Variant};
use ::godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};
use ::serde::{Deserialize, Serialize};
use crate::{readVariant, readVariantOption};

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetroPlatform
{
	/// Information specific to the Platform.
	pub info: RetroAchievementsInfo,
	/// The list of achievements associated with this game.
	pub achievements: Vec<RetroAchievement>,
}

impl FromGodot for RetroPlatform
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
		return Ok(Self::fromDict(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::fromVariant(variant));
	}
}

impl GodotConvert for RetroPlatform
{
	type Via = Dictionary;
}

impl ToGodot for RetroPlatform
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

impl RetroPlatform
{
	pub fn isGlobalPercentageMissing(&self) -> bool
	{
		return self.achievements.iter()
			.any(|a| a.globalPercentage.is_none());
	}
	
	/**
	Retrieve either the total accumulated points or the maximum possible points
	awarded for this game's achievements on RetroAchievements.org.
	
	Parameter | Type | Description
	---|---|---
	mode | Mode | The mode which determines the amount of points per achievement.
	maximumPossible | Boolean | Whether (TRUE) or not (FALSE) to take unlock status into consideration when summing the points.
	*/
	pub fn points(&self, mode: RetroMode, maximumPossible: bool) -> i64
	{
		let mut points = 0;
		for achievement in &self.achievements
		{
			if maximumPossible == true || achievement.mode == mode
			{
				if let Some(value) = achievement.points.get(&mode)
				{
					points += *value;
				}
			}
		}
		
		return points;
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
		let info = readVariant!(dict.get("info"), RetroAchievementsInfo);
		
		let mut achievements = vec![];
		let arr = readVariant!(dict.get("achievements"), Array::<Dictionary>);
		for d in arr.iter_shared()
		{
			achievements.push(RetroAchievement::from_godot(d));
		}
		
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
The mode representing the conditions under which an achievment was unlocked.

*Only used by: RetroAchievements*
*/
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum RetroMode
{
	Hardcore,
	Softcore,
}

impl Default for RetroMode
{
	fn default() -> Self
	{
		return Self::Softcore;
	}
}

impl FromGodot for RetroMode
{
	fn from_godot(via: Self::Via) -> Self
	{
		return Self::from(via);
	}
	
	fn from_variant(variant: &Variant) -> Self
	{
		let value = i64::from_variant(variant);
		return Self::from(value);
	}
	
	fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>
	{
		return Ok(Self::from(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		let value = i64::try_from_variant(variant)?;
		return Ok(Self::from(value));
	}
}

impl GodotConvert for RetroMode
{
	type Via = i64;
}

impl ToGodot for RetroMode
{
	fn into_godot(self) -> Self::Via
	{
		return self as i64;
	}
	
	fn to_godot(&self) -> Self::Via
	{
		return *self as i64;
	}
	
	fn to_variant(&self) -> Variant
	{
		return (*self as i64).to_variant();
	}
}

impl From<i64> for RetroMode
{
	fn from(value: i64) -> Self
	{
		return match value
		{
			0 => RetroMode::Hardcore,
			_ => RetroMode::Softcore,
		};
	}
}

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsInfo
{
	pub id: String,
}

impl FromGodot for RetroAchievementsInfo
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
		return Ok(Self::fromDict(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::fromVariant(variant));
	}
}

impl GodotConvert for RetroAchievementsInfo
{
	type Via = Dictionary;
}

impl ToGodot for RetroAchievementsInfo
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

impl RetroAchievementsInfo
{
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("id", self.id.to_godot());
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let id = readVariant!(dict.get("id"), String);
		
		return Self
		{
			id,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}

/**
Achievement data specific to the RetroAchievements.org platform.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievement
{
	/// The human-readable description of this achievement.
	pub description: String,
	
	/// The percentage of users on this platform who have unlocked this achievement.
	pub globalPercentage: Option<f64>,
	
	/// The platform-specific ID of this achievement.
	pub id: String,
	
	/// The mode under which this achievement was unlocked.
	pub mode: RetroMode,
	
	/// The human-readable name of this achievement.
	pub name: String,
	
	/// The points awarded when this achievement is unlocked.
	pub points: HashMap<RetroMode, i64>,
	
	/// The timestamp at which the achievement was unlocked.
	pub timestamp: Option<i64>,
}

impl FromGodot for RetroAchievement
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
		return Ok(Self::fromDict(via));
	}
	
	fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError>
	{
		return Ok(Self::fromVariant(variant));
	}
}

impl GodotConvert for RetroAchievement
{
	type Via = Dictionary;
}

impl ToGodot for RetroAchievement
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

impl RetroAchievement
{
	/**
	
	*/
	pub fn new(id: String, name: String, description: String) -> Self
	{
		return Self
		{
			description,
			id,
			name,
			..Default::default()
		}
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
		let mut points = Dictionary::new();
		points.insert("Softcore", self.points[&RetroMode::Softcore]);
		points.insert("Hardcore", self.points[&RetroMode::Hardcore]);
		
		let mut dict = Dictionary::new();
		dict.insert("description", self.description.to_godot());
		dict.insert("globalPercentage", match self.globalPercentage { Some(f) => f, None => 0.0 });
		dict.insert("id", self.id.to_godot());
		dict.insert("mode", self.mode);
		dict.insert("name", self.name.to_godot());
		dict.insert("points", points);
		dict.insert("timestamp", match self.timestamp { Some(f) => f, None => 0 });
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let mut points = HashMap::default();
		let p = readVariant!(dict.get("points"), Dictionary);
		for (k, v) in p.iter_shared()
		{
			let key = RetroMode::from_variant(&k);
			let value = i64::from_variant(&v);
			
			points.insert(key, value);
		}
		
		let description = readVariant!(dict.get("description"), String);
		let globalPercentage = readVariantOption!(dict.get("globalPercentage"), f64);
		let id = readVariant!(dict.get("id"), String);
		let mode = readVariant!(dict.get("mode"), RetroMode);
		let name = readVariant!(dict.get("name"), String);
		let timestamp = readVariantOption!(dict.get("timestamp"), i64);
		
		return Self
		{
			description,
			globalPercentage,
			id,
			mode,
			name,
			points,
			timestamp,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use std::collections::HashMap;
	
	fn setupAchievement(name: &str, hcPoints: i64, scPoints: i64, mode: RetroMode) -> RetroAchievement
	{
		let mut points = HashMap::new();
		points.insert(RetroMode::Hardcore, hcPoints);
		points.insert(RetroMode::Softcore, scPoints);
		
		let achievement = RetroAchievement
		{
			description: String::default(),
			globalPercentage: None,
			id: String::default(),
			mode,
			name: name.to_string(),
			points,
			timestamp: match mode
			{
				RetroMode::Hardcore => Some(1),
				RetroMode::Softcore => None,
			}
		};
		
		return achievement;
	}
}
