use anyhow::Result;
use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::{Payload_GetUserCompletionProgress, Payload_GetUserProfile};
use crate::data::kind::AwardKind;
use super::makeRelative;
use super::achievement::Achievement;
use super::mode::RetroAchievementsMode;
use super::rank::RankData;
use super::game::Game;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RetroAchievementsUser
{
	/// The relative path to the user's avatar on RetroAchievements.org.
	#[serde(default)]
	pub avatar: Option<String>,
	
	/// The user's rank data for Casual mode.
	#[serde(default)]
	pub casual: RankData,
	
	/// The list of games the player has played.
	#[serde(default)]
	pub games: Vec<Game>,
	
	/// The user's rank data for Hardcore mode.
	#[serde(default)]
	pub hardcore: RankData,
	
	#[serde(default)]
	pub retroPoints: u64,
	
	/// The user's ULID.
	#[serde(default)]
	pub ulid: Option<String>,
	
	/// The user's username.
	pub username: String,
}

impl RadioChannel<RetroAchievementsUser> for GamePlatforms {}

impl Default for RetroAchievementsUser
{
	fn default() -> Self
	{
		return Self
		{
			avatar: None,
			casual: RetroAchievementsMode::Casual.into(),
			games: vec![],
			hardcore: RankData::default(),
			retroPoints: u64::default(),
			ulid: None,
			username: String::default(),
		};
	}
}

impl RetroAchievementsUser
{
	pub const FileName: &str = "retroAchievements.json";
	
	pub fn countByAward(&self, award: AwardKind) -> usize
	{
		return self.games.iter()
			.filter(|g| g.highestAward.is_some_and(
				|a| a == award)
			)
			.count();
	}
	
	pub fn filterGames(&self, search: impl Into<String>) -> Vec<Game>
	{
		let search = search.into().to_lowercase();
		let mut games = self.games.iter()
			.filter(|g| g.name.to_lowercase().contains(&search)
					|| g.system.name.to_lowercase().contains(&search))
			.cloned()
			.collect::<Vec<_>>();
		games.sort();
		
		return games;
	}
	
	pub fn getAchievement(&self, gameId: impl Into<u64>, achievementId: impl Into<u64>) -> Option<Achievement>
	{
		let achievementId = achievementId.into();
		return match self.getGame(gameId)
		{
			None => None,
			Some(g) => g.achievements.iter()
				.find(|a| a.id == achievementId)
				.cloned(),
		};
	}
	
	pub fn getDistinctPlayersForGame(&self, id: impl Into<u64>) -> Option<u64>
	{
		let id = id.into();
		return match self.games.iter()
			.find(|g| g.id == id)
		{
			None => None,
			Some(g) => Some(g.distinctPlayers),
		};
	}
	
	pub fn getGame(&self, id: impl Into<u64>) -> Option<Game>
	{
		let id = id.into();
		return self.games.iter()
			.find(|g| g.id == id)
			.cloned();
	}
	
	/**
	Parse a JSON string which does not strictly conform to the expected `User`
	data structure.
	
	This function will retain as much data as possible but will omit objects if
	they are missing required properties.
	
	Any missing properties which are not strictly required will instead be
	filled with their default values.
	
	Only returns `Err` if `json` is not valid JSON.
	*/
	pub fn parseJsonLossy(json: String) -> Result<Self>
	{
		let root = serde_json::from_str::<Value>(json.as_str())?;
		
		let mut user = Self::default();
		
		match root
		{
			Value::Object(map) => {
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "avatar")
				{
					if let Value::String(value) = value
					{
						user.avatar = Some(value.clone());
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "casual")
				{
					if let Value::Object(rankMap) = value
					{
						user.casual = RankData::parseJsonMap(rankMap);
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "hardcore")
				{
					if let Value::Object(rankMap) = value
					{
						user.hardcore = RankData::parseJsonMap(rankMap);
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(key, _)| key.as_str() == "retroPoints")
				{
					if let Value::Number(number) = value
					{
						if let Some(inner) = number.as_u64()
						{
							user.retroPoints = inner;
						}
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "ulid")
				{
					if let Value::String(value) = value
					{
						user.ulid = Some(value.clone());
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "username")
				{
					if let Value::String(value) = value
					{
						user.username = value.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "games")
				{
					if let Value::Array(games) = value
					{
						let mut parsedGames = vec![];
						for gameValue in games
						{
							if let Value::Object(gameMap) = gameValue
							{
								if let Some(game) = Game::parseJsonMap(gameMap)
								{
									parsedGames.push(game);
								}
							}
						}
						user.games = parsedGames;
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn points(&self) -> u64
	{
		return match self.hardcore.points > 0
		{
			false => self.casual.points,
			true => self.hardcore.points,
		};
	}
	
	pub fn processUserCompletionProgress(&mut self, payload: &Payload_GetUserCompletionProgress)
	{
		for metadata in payload.Results.iter()
		{
			match self.games.iter_mut()
				.find(|g| g.id == metadata.GameID)
			{
				None => self.games.push(metadata.clone().into()),
				Some(game) => game.update(&metadata),
			}
		}
	}
	
	pub fn processUserProfile(&mut self, payload: &Payload_GetUserProfile)
	{
		self.casual.points = payload.TotalSoftcorePoints;
		self.hardcore.points = payload.TotalPoints;
		self.retroPoints = payload.TotalTruePoints;
		
		self.avatar = match payload.UserPic.is_empty()
		{
			false => Some(makeRelative(&payload.UserPic)),
			true => None,
		};
		
		self.ulid = match payload.ULID.is_empty()
		{
			false => Some(payload.ULID.clone()),
			true => None,
		};
		
		self.username = payload.User.clone();
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"username": "Test User",
	"games": [
		{
			"id": 7,
			"achievements": [
				{ "id": 4, "name": "Successful parse!" },
				{ "name": "This one should fail to parse" }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"achievements": [
				{ "id": 4, "name": "Successful parse!" }
			]
		}
	]
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = RetroAchievementsUser::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.games.len(), 1);
		
		let game = user.games.first().unwrap();
		assert_eq!(game.id, 7);
		assert_eq!(game.achievements.len(), 1);
		
		let achievement = game.achievements.first().unwrap();
		assert_eq!(achievement.id, 4);
		assert_eq!(achievement.name, "Successful parse!".to_string());
	}
}
