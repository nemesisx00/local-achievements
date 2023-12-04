use ::serde::{Deserialize, Serialize};
use crate::platforms::steam::{SteamAchievementData, SteamAchievementMetadata, SteamGame};
use super::game::{Game, SteamAchievement, SteamInfo, GamePlatform, RetroMode};

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
	pub retroAchievements: RetroAchievementsProfile,
	
	/// This user's Steam profile information.
	pub steam: SteamProfile,
}

unsafe impl Send for User {}

impl User
{
	pub const Filename: &'static str = "data.json";
	
	pub fn processSteamAchievements(&mut self, id: usize, achievements: Vec<SteamAchievementData>)
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
	
	pub fn processSteamAchievementMetadata(&mut self, id: usize, achievements: Vec<SteamAchievementMetadata>)
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
				let mut steam = GamePlatform::<SteamInfo, SteamAchievement>::default();
				steam.info.update(info);
				
				let mut game = Game::default();
				game.steam = Some(steam);
				
				self.games.push(game);
			}
		}
	}
}

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

/**

*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsRank
{
	/// The mode corresponding to this rank and point amount.
	pub mode: RetroMode,
	
	/// The total number of points earned.
	pub points: usize,
	
	/// The current rank on RetroAchievements.org.
	pub rank: usize,
	
	/// The total users, used to create a relation for the rank.
	pub total: usize,
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
}

/**
Profile information for a Steam user.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SteamProfile
{
	/// The path to the user's avatar
	pub avatar: Option<String>,
	
	/// The user's 64-bit Steam ID
	pub id: String,
	
	/// The user's current publicly visible display name.
	pub name: String,
}

impl SteamProfile
{
	pub fn update(&mut self, id: String, name: String, avatar: Option<String>)
	{
		self.id = id;
		self.name = name;
		self.avatar = avatar;
	}
}
