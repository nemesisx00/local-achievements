use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};

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
	pub fn points(&self, mode: RetroMode, maximumPossible: bool) -> usize
	{
		let mut points = 0;
		for achievement in &self.achievements
		{
			if maximumPossible == true || achievement.mode.is_some_and(|m| m == mode)
			{
				if let Some(map) = &achievement.points
				{
					if let Some(value) = map.get(&mode)
					{
						points += *value;
					}
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

/**

*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetroAchievementsInfo
{
	pub id: String,
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
	pub mode: Option<RetroMode>,
	
	/// The human-readable name of this achievement.
	pub name: String,
	
	/// The points awarded when this achievement is unlocked.
	pub points: Option<HashMap<RetroMode, usize>>,
	
	/// The timestamp at which the achievement was unlocked.
	pub timestamp: Option<usize>,
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
			globalPercentage: None,
			id,
			mode: None,
			name,
			points: None,
			timestamp: None,
		}
	}
	
	/**
	Is this achievement unlocked on this platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.timestamp.is_some();
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use std::collections::HashMap;
	
	fn setupAchievement(name: &str, hcPoints: usize, scPoints: usize, mode: Option<RetroMode>) -> RetroAchievement
	{
		let mut points = HashMap::new();
		points.insert(RetroMode::Hardcore, hcPoints);
		points.insert(RetroMode::Softcore, scPoints);
		
		let achievement = RetroAchievement
		{
			description: String::default(),
			globalPercentage: None,
			id: String::default(),
			mode: mode,
			name: name.to_string(),
			points: Some(points),
			timestamp: match mode
			{
				Some(_) => Some(1),
				None => None,
			}
		};
		
		return achievement;
	}
}
