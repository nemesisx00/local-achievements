#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetGlobalAchievementPercentagesForApp
endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetGlobalPercentagesPayload
{
	pub achievementpercentages: GlobalPercentages,
}

impl GetGlobalPercentagesPayload
{
	/**
	Convert this payload's data into a form that is ready to be consumed by a
	`crate::data::Game` instance.
	*/
	pub fn asMap(&self) -> HashMap<String, f64>
	{
		let mut map = HashMap::new();
		for pair in self.achievementpercentages.achievements.iter()
		{
			map.insert(pair.name.to_owned(), pair.percent);
		}
		return map;
	}
}

/**
The list of achievements and their global percentages.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalPercentages
{
	pub achievements: Vec<GlobalPercentage>,
}

/**
The global percentage for a single achievement.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalPercentage
{
	pub name: String,
	pub percent: f64,
}
