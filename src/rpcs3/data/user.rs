use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::game::Game;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	#[serde(default)]
	pub games: Vec<Game>,
	
	pub accountId: u64,
	
	#[serde(default)]
	pub name: String,
	
	#[serde(default)]
	pub points: u64,
}

impl User
{
	pub const FileName: &str = "rpcs3.json";
	
	pub fn calculatePoints(&mut self)
	{
		self.points = self.games.iter()
			.fold(0, |acc, g| acc + g.points());
	}
	
	pub fn formatAccountId(&self) -> String
	{
		return format!("{:08}", self.accountId);
	}
	
	pub fn level(&self) -> u64
	{
		let mut level = 0;
		
		if self.points >= 70000
		{
			level = 19 + ((self.points - 70000) / 8000);
		}
		else if self.points >= 16000
		{
			level = 12 + ((self.points - 16000) / 8000);
		}
		else if self.points >= 4000
		{
			level = 6 + ((self.points - 4000) / 2000);
		}
		else if self.points >= 2400
		{
			level = 5;
		}
		else if self.points >= 1200
		{
			level = 4;
		}
		else if self.points >= 600
		{
			level = 3;
		}
		else if self.points >= 200
		{
			level = 2;
		}
		else if self.points > 0
		{
			level = 1;
		}
		
		return level;
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
					.find(|(k, _)| k.as_str() == "games")
				{
					if let Value::Array(inner) = value
					{
						let mut parsedGames = vec![];
						for gameValue in inner
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
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "accountId")
				{
					if let Value::Number(inner) = value
					{
						if let Some(number) = inner.as_u64()
						{
							user.accountId = number;
						}
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "name")
				{
					if let Value::String(value) = value
					{
						user.name = value.to_owned();
					}
				}
				
				if let Some((_, value)) = map.iter()
					.find(|(k, _)| k.as_str() == "points")
				{
					if let Value::Number(inner) = value
					{
						if let Some(number) = inner.as_u64()
						{
							user.points = number;
						}
					}
				}
			},
			
			_ => {},
		}
		
		return Ok(user);
	}
	
	pub fn platinumCount(&self) -> u64
	{
		return self.games.iter()
			.filter(|g| g.trophies.iter()
				.filter(|t| t.platinumRelevance && t.unlocked)
				.count() == g.trophies.iter()
					.filter(|t| t.platinumRelevance)
					.count()
			)
			.count() as u64;
	}
	
	/**
	Calculate the number of points required to reach the next level.
	*/
	pub fn pointsToNextLevel(&self) -> u64
	{
		return match self.level()
		{
			0 => 1,
			1 => 200 - self.points,
			2 => 600 - self.points,
			3 => 1200 - self.points,
			4 => 2400 - self.points,
			5 => 4000 - self.points,
			6..12 => (self.points - 4000) % 2000,
			12..19 => (self.points - 16000) % 8000,
			_ => (self.points - 70000) % 8000,
		};
	}
	
	/**
	The total points required to level up for the current level.
	*/
	pub fn pointTotalForLevel(&self) -> u64
	{
		return match self.level()
		{
			0 => 1,
			1 => 200,
			2 => 600,
			3 => 1200,
			4 => 2400,
			5 => 4000,
			6..12 => 2000,
			_ => 8000,
		};
	}
	
	/**
	Update the user's game data based on the given list of `games`.
	
	## Effects
	
	- Updates games which do exist.
	- Adds games which do not exist.
	
	## Note
	
	- Does not delete games which exist but are not present in the new list.
	*/
	pub fn updateGamesList(&mut self, games: Vec<Game>)
	{
		for game in self.games.iter_mut()
		{
			if let Some(other) = games.iter()
				.find(|g| g.npCommId == game.npCommId)
			{
				game.update(other);
			}
		}
		
		let gameIds = self.games.iter()
			.cloned()
			.map(|internal| internal.npCommId)
			.collect::<Vec<String>>();
		
		for game in games.iter()
			.filter(|g| !gameIds.contains(&g.npCommId))
		{
			self.games.push(game.to_owned());
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const PartialJson: &str = r#"{
	"username": "Test User",
	"points": 12345,
	"accountId": 3,
	"games": [
		{
			"npCommId": "The game 1",
			"name": "First game",
			"trophies": [
				{ "id": 4, "name": "Successful parse!" },
				{ "name": "This one should fail to parse" }
			]
		},
		
		{
			"name": "Test game that shouldn't parse",
			"trophies": [
				{ "id": 4, "name": "Successful parse!" }
			]
		}
	]
}"#;
	
	#[test]
	fn parseJsonLossy()
	{
		let result = User::parseJsonLossy(PartialJson.into());
		assert!(result.is_ok());
		
		let user = result.unwrap();
		assert_eq!(user.games.len(), 1);
		
		let game = user.games.first().unwrap();
		assert_eq!(game.npCommId, "The game 1".to_string());
		assert_eq!(game.name, "First game".to_string());
		assert_eq!(game.trophies.len(), 1);
		
		let trophy = game.trophies.first().unwrap();
		assert_eq!(trophy.id, 4);
		assert_eq!(trophy.name, "Successful parse!".to_string());
	}
	
	#[test]
	fn level()
	{
		let mut user = User::default();
		
		assert_eq!(user.level(), 0);
		assert_eq!(user.pointsToNextLevel(), 1);
		user.points = 50;
		assert_eq!(user.level(), 1);
		assert_eq!(user.pointsToNextLevel(), 150);
		user.points = 250;
		assert_eq!(user.level(), 2);
		assert_eq!(user.pointsToNextLevel(), 350);
		user.points = 600;
		assert_eq!(user.level(), 3);
		assert_eq!(user.pointsToNextLevel(), 600);
		user.points = 1501;
		assert_eq!(user.level(), 4);
		assert_eq!(user.pointsToNextLevel(), 899);
		user.points = 2500;
		assert_eq!(user.level(), 5);
		assert_eq!(user.pointsToNextLevel(), 1500);
		user.points = 4321;
		assert_eq!(user.level(), 6);
		assert_eq!(user.pointsToNextLevel(), 321);
		user.points = 6010;
		assert_eq!(user.level(), 7);
		assert_eq!(user.pointsToNextLevel(), 10);
		user.points = 8888;
		assert_eq!(user.level(), 8);
		assert_eq!(user.pointsToNextLevel(), 888);
		user.points = 11111;
		assert_eq!(user.level(), 9);
		assert_eq!(user.pointsToNextLevel(), 1111);
		user.points = 12345;
		assert_eq!(user.level(), 10);
		assert_eq!(user.pointsToNextLevel(), 345);
		user.points = 14542;
		assert_eq!(user.level(), 11);
		assert_eq!(user.pointsToNextLevel(), 542);
		user.points = 16789;
		assert_eq!(user.level(), 12);
		assert_eq!(user.pointsToNextLevel(), 789);
		user.points = 25360;
		assert_eq!(user.level(), 13);
		assert_eq!(user.pointsToNextLevel(), 1360);
		user.points = 33333;
		assert_eq!(user.level(), 14);
		assert_eq!(user.pointsToNextLevel(), 1333);
		user.points = 44444;
		assert_eq!(user.level(), 15);
		assert_eq!(user.pointsToNextLevel(), 4444);
		user.points = 48901;
		assert_eq!(user.level(), 16);
		assert_eq!(user.pointsToNextLevel(), 901);
		user.points = 56789;
		assert_eq!(user.level(), 17);
		assert_eq!(user.pointsToNextLevel(), 789);
		user.points = 64208;
		assert_eq!(user.level(), 18);
		assert_eq!(user.pointsToNextLevel(), 208);
		user.points = 76000;
		assert_eq!(user.level(), 19);
		assert_eq!(user.pointsToNextLevel(), 6000);
		user.points = 80000;
		assert_eq!(user.level(), 20);
		assert_eq!(user.pointsToNextLevel(), 2000);
		user.points = 87654;
		assert_eq!(user.level(), 21);
		assert_eq!(user.pointsToNextLevel(), 1654);
		user.points = 95045;
		assert_eq!(user.level(), 22);
		assert_eq!(user.pointsToNextLevel(), 1045);
	}
}
