use ::godot::builtin::{Dictionary, Variant};
use ::godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};
use ::serde::{Deserialize, Serialize};
use crate::data::game::RetroMode;
use crate::readVariant;

/**
Profile information for a RetroAchievements user.
*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsProfile
{
	/// The user's username
	pub username: String,
	
	pub hardcore: RetroAchievementsRank,
	
	pub softcore: RetroAchievementsRank,
}

impl Default for RetroAchievementsProfile
{
	fn default() -> Self
	{
		return Self
		{
			username: String::new(),
			hardcore: RetroAchievementsRank::default(),
			softcore: RetroAchievementsRank::new(RetroMode::Softcore),
		};
	}
}

impl FromGodot for RetroAchievementsProfile
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

impl GodotConvert for RetroAchievementsProfile
{
	type Via = Dictionary;
}

impl ToGodot for RetroAchievementsProfile
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

impl RetroAchievementsProfile
{
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("username", self.username.to_godot());
		dict.insert("hardcore", self.hardcore.to_godot());
		dict.insert("softcore", self.softcore.to_godot());
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let username = readVariant!(dict.get("username"), String);
		let hardcore = readVariant!(dict.get("hardcore"), RetroAchievementsRank);
		let softcore = readVariant!(dict.get("softcore"), RetroAchievementsRank);
		
		return Self
		{
			username,
			hardcore,
			softcore,
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsRank
{
	/// The mode corresponding to this rank and point amount.
	pub mode: RetroMode,
	
	/// The total number of points earned.
	pub points: i64,
	
	/// The current rank on RetroAchievements.org.
	pub rank: i64,
	
	/// The total users, used to create a relation for the rank.
	pub total: i64,
}

impl Default for RetroAchievementsRank
{
	fn default() -> Self
	{
		return Self
		{
			mode: RetroMode::Hardcore,
			points: 0,
			rank: 0,
			total: 0,
		}
	}
}

impl FromGodot for RetroAchievementsRank
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

impl GodotConvert for RetroAchievementsRank
{
	type Via = Dictionary;
}

impl ToGodot for RetroAchievementsRank
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

impl RetroAchievementsRank
{
	/**
	Create a new instance of RetroAchievementsRank with the given mode.
	*/
	pub fn new(mode: RetroMode) -> Self
	{
		return Self
		{
			mode,
			..Default::default()
		};
	}
	
	fn buildDict(&self) -> Dictionary
	{
		let mut dict = Dictionary::new();
		dict.insert("mode", self.mode);
		dict.insert("points", self.points);
		dict.insert("rank", self.rank);
		dict.insert("total", self.total);
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let mode = readVariant!(dict.get("mode"), RetroMode);
		let points = readVariant!(dict.get("points"), i64);
		let rank = readVariant!(dict.get("rank"), i64);
		let total = readVariant!(dict.get("total"), i64);
		
		return Self
		{
			mode,
			points,
			rank,
			total,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}