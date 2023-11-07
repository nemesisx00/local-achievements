#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::serde::{Deserialize, Serialize};

/**
The expected response data returned by the GetGlobalAchievementPercentagesForApp
endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GetGlobalPercentagesPayload
{
	achievementpercentages: GlobalPercentages,
}

/**
The list of achievements and their global percentages.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalPercentages
{
	achievements: Vec<GlobalPercentage>,
}

/**
The global percentage for a single achievement.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalPercentage
{
	name: String,
	percent: f64,
}
