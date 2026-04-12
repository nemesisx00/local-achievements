use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::{Product, UserInfo};
use super::achievement::GogAchievement;
use super::game::Game;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GogUser
{
	#[serde(with = "ts_seconds")]
	pub createdTimestamp: DateTime<Utc>,
	pub employee: bool,
	pub games: Vec<Game>,
	pub id: String,
	pub name: String,
}

impl RadioChannel<GogUser> for GamePlatforms {}

impl GogUser
{
	pub const FileName: &str = "gog.json";
	
	const UserCreatedFormat: &str = "%Y-%m-%dT%H:%M:%S%:z";
	
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
					.find(|(k, _)| k.as_str() == "createdTimestamp")
				{
					if let Value::Number(inner) = value
					{
						if let Some(number) = inner.as_i64()
						{
							if let Some(timestamp) = DateTime::from_timestamp(number, 0)
							{
								user.createdTimestamp = timestamp.clone();
							}
						}
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "employee")
				{
					if let Value::Bool(inner) = value
					{
						user.employee = inner.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "id")
				{
					if let Value::String(inner) = value
					{
						user.id = inner.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "name")
				{
					if let Value::String(inner) = value
					{
						user.name = inner.clone();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(key, _)| key.as_str() == "games")
				{
					if let Value::Array(inner) = value
					{
						let mut parsedGames = vec![];
						for gameValue in inner
						{
							if let Value::Object(map) = gameValue
							{
								if let Some(game) = Game::parseJsonMap(map)
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
	
	pub fn filterGames(&self, search: impl Into<String>) -> Vec<Game>
	{
		let text = search.into().to_lowercase();
		let mut games = self.games.iter()
			.filter(|g| g.name.to_lowercase().contains(&text))
			.cloned()
			.collect::<Vec<_>>();
		games.sort();
		
		return games;
	}
	
	pub fn getAchievement(&self, gameId: impl Into<u64>, achievementId: impl Into<String>) -> Option<GogAchievement>
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
	
	pub fn getGame(&self, id: impl Into<u64>) -> Option<Game>
	{
		let id = id.into();
		return self.games.iter()
			.find(|g| g.id == id)
			.cloned();
	}
	
	pub fn updateGames(&mut self, games: impl Into<Vec<Product>>)
	{
		for product in games.into()
		{
			if let Some(game) = self.games.iter_mut()
				.find(|g| g.id == product.id)
			{
				game.update(&product);
			}
			else
			{
				self.games.push(Game::from(product.clone()));
			}
		}
	}
	
	pub fn updateGameAchievements(&mut self, gameId: impl Into<u64>, achievements: impl Into<Vec<GogAchievement>>)
	{
		let gameId = gameId.into();
		let achievements = achievements.into();
		
		if let Some(game) = self.games.iter_mut()
			.find(|g| g.id == gameId)
		{
			game.updateAchievements(achievements);
		}
	}
	
	pub fn updateUserInfo(&mut self, userInfo: impl Into<UserInfo>)
	{
		let userInfo = userInfo.into();
		
		let timestamp = DateTime::parse_from_str(
			&userInfo.created_date,
			Self::UserCreatedFormat
		);
		
		self.createdTimestamp = match timestamp
		{
			Err(_) => Utc::now(),
			Ok(ts) => ts.into(),
		};
		
		self.employee = userInfo.is_employee;
		self.id = userInfo.id.clone();
		self.name = userInfo.username.clone();
	}
}

#[cfg(test)]
mod tests
{
	use chrono::DateTime;
	use super::*;
	
	const PartialJson: &str = r#"{
	"createdTimestamp": 1763543348,
	"employee": true,
	"games": [
		{
			"id": 7,
			"achievements": [
				{ "id": "4", "name": "Successful parse!" },
				{ "name": "This one should fail to parse" }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"achievements": [
				{ "id": "4", "name": "Successful parse!" }
			]
		}
	],
	"id": "The id",
	"name": "Test User"
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let ct = DateTime::from_timestamp_secs(1763543348).unwrap();
		
		let result = GogUser::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		
		assert_eq!(user.createdTimestamp, ct);
		assert_eq!(user.employee, true);
		assert_eq!(user.games.len(), 1);
		assert_eq!(&user.id, "The id");
		assert_eq!(&user.name, "Test User");
		
		let game = user.games.first().unwrap();
		assert_eq!(game.id, 7);
		assert_eq!(game.achievements.len(), 1);
		
		let achievement = game.achievements.first().unwrap();
		assert_eq!(&achievement.id, "4");
		assert_eq!(&achievement.name, "Successful parse!");
	}
}
