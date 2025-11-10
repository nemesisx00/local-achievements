use serde::{Deserialize, Serialize};

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
	pub percent: String,
}

/**
The expected response data returned by the GetGlobalAchievementPercentagesForApp
endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload
{
	pub achievementpercentages: GlobalPercentages,
}
