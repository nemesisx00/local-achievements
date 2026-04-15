use serde::Deserialize;

/**
The list of achievements and their global percentages.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct GlobalPercentages
{
	pub achievements: Vec<GlobalPercentage>,
}

/**
The global percentage for a single achievement.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct GlobalPercentage
{
	pub name: String,
	pub percent: String,
}

/**
The expected response data returned by the GetGlobalAchievementPercentagesForApp
endpoint.
*/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Payload_GetGlobalPercentages
{
	pub achievementpercentages: GlobalPercentages,
}
