use ::godot::bind::property::{Property, PropertyHintInfo};
use ::godot::builtin::{Array, Dictionary, Variant};
use ::godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};
use ::serde::{Deserialize, Serialize};
use crate::readVariant;
use crate::data::game::{Game, SteamPlatform};
use crate::platforms::steam::{SteamAchievementData, SteamAchievementMetadata, SteamGame};
use super::{retro::RetroAchievementsProfile, steam::SteamProfile};

/**
A single user, containing platform-specific profile information and its combined
list of games which have achievements.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	/**
	The list of games associated with this user which also have achievements
	defined, across all platforms.
	*/
	pub games: Vec<Game>,
	
	/// This user's RetroAchievements profile information.
	pub retro: RetroAchievementsProfile,
	
	/// This user's Steam profile information.
	pub steam: SteamProfile,
}

unsafe impl Send for User {}

impl Property for User
{
	type Intermediate = User;
	
	fn get_property(&self) -> Self::Intermediate
	{
		return self.clone();
	}
	
	fn property_hint() -> PropertyHintInfo
	{
		return PropertyHintInfo::with_hint_none("User");
	}
	
	fn set_property(&mut self, value: Self::Intermediate)
	{
		self.games = value.games;
		self.retro = value.retro;
		self.steam = value.steam;
	}
}

impl FromGodot for User
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

impl GodotConvert for User
{
	type Via = Dictionary;
}

impl ToGodot for User
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

impl User
{
	pub const Filename: &'static str = "data.json";
	
	pub fn processSteamAchievements(&mut self, id: i64, achievements: Vec<SteamAchievementData>)
	{
		if let Some(game) = self.games.iter_mut()
			.find(|g| match &g.steam
			{
				Some(s) => s.info.id == id,
				None => false,
			})
			.as_mut()
		{
			if let Some(steam) = game.steam.as_mut()
			{
				steam.updateAchievements(achievements);
			}
		}
	}
	
	pub fn processSteamAchievementMetadata(&mut self, id: i64, achievements: Vec<SteamAchievementMetadata>)
	{
		if let Some(game) = self.games.iter_mut()
			.find(|g| match &g.steam
			{
				Some(s) => s.info.id == id,
				None => false,
			})
			.as_mut()
		{
			if let Some(steam) = game.steam.as_mut()
			{
				steam.updateAchievementMetadata(achievements);
			}
		}
	}
	
	pub fn processSteamGames(&mut self, games: Vec<SteamGame>)
	{
		for info in games
		{
			// Game already exists, just update metadata
			if let Some(game) = self.games.iter_mut().find(|g|
				g.steam.as_ref().is_some_and(|si|
					si.info.id == info.appid))
			{
				if let Some(steam) = game.steam.as_mut()
				{
					steam.info.update(info)
				}
			}
			// Game already exists as a duplicate, just update metadata
			else if let Some(game) = self.games.iter_mut()
				.find(|g|
					g.duplicates.as_ref().is_some_and(|list|
						list.iter().any(|dupe|
							dupe.steam.as_ref().is_some_and(|si|
								si.info.id == info.appid))))
			{
				if let Some(list) = game.duplicates.as_mut()
				{
					if let Some(dupe) = list.iter_mut().find(|d|
						d.steam.as_ref().is_some_and(|si|
							si.info.id == info.appid))
					{
						if let Some(steam) = dupe.steam.as_mut()
						{
							steam.info.update(info);
						}
					}
				}
			}
			// Game does not exist
			else
			{
				let mut steam = SteamPlatform::default();
				steam.info.update(info);
				
				let mut game = Game::default();
				game.steam = Some(steam);
				
				self.games.push(game);
			}
		}
	}
	
	fn buildDict(&self) -> Dictionary
	{
		let mut arr = Array::new();
		for game in &self.games
		{
			arr.push(game.to_godot());
		}
		
		let mut dict = Dictionary::new();
		dict.insert("games", arr);
		dict.insert("retro", self.retro.to_godot());
		dict.insert("steam", self.steam.to_godot());
		
		return dict;
	}
	
	fn fromDict(dict: Dictionary) -> Self
	{
		let mut games = vec![];
		let gameDicts = readVariant!(dict.get("games"), Array::<Dictionary>);
		for g in gameDicts.iter_shared()
		{
			games.push(Game::from_godot(g));
		}
		
		let retro = readVariant!(dict.get("retro"), RetroAchievementsProfile);
		let steam = readVariant!(dict.get("steam"), SteamProfile);
		
		return Self
		{
			games,
			retro,
			steam,
		};
	}
	
	fn fromVariant(variant: &Variant) -> Self
	{
		let dict = Dictionary::from_variant(variant);
		return Self::fromDict(dict);
	}
}
