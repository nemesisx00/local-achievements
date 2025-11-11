use serde::{Deserialize, Serialize};
use super::game::Game;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct User
{
	pub games: Vec<Game>,
	pub accountId: u64,
	pub name: String,
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
	
	#[test]
	fn level()
	{
		let mut user = User::default();
		
		assert_eq!(user.level(), 0);
		user.points = 50;
		assert_eq!(user.level(), 1);
		user.points = 250;
		assert_eq!(user.level(), 2);
		user.points = 600;
		assert_eq!(user.level(), 3);
		user.points = 1501;
		assert_eq!(user.level(), 4);
		user.points = 2500;
		assert_eq!(user.level(), 5);
		user.points = 4321;
		assert_eq!(user.level(), 6);
		user.points = 6010;
		assert_eq!(user.level(), 7);
		user.points = 8888;
		assert_eq!(user.level(), 8);
		user.points = 11111;
		assert_eq!(user.level(), 9);
		user.points = 12345;
		assert_eq!(user.level(), 10);
		user.points = 14542;
		assert_eq!(user.level(), 11);
		user.points = 16789;
		assert_eq!(user.level(), 12);
		user.points = 25360;
		assert_eq!(user.level(), 13);
		user.points = 33333;
		assert_eq!(user.level(), 14);
		user.points = 44444;
		assert_eq!(user.level(), 15);
		user.points = 48901;
		assert_eq!(user.level(), 16);
		user.points = 56789;
		assert_eq!(user.level(), 17);
		user.points = 64208;
		assert_eq!(user.level(), 18);
		user.points = 76000;
		assert_eq!(user.level(), 19);
		user.points = 80000;
		assert_eq!(user.level(), 20);
		user.points = 87654;
		assert_eq!(user.level(), 21);
		user.points = 95045;
		assert_eq!(user.level(), 22);
	}
}
