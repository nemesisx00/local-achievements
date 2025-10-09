use serde::{Deserialize, Serialize};

use crate::data::SteamAchievement;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameAchievement
{
	pub name: String,
	pub defaultvalue: usize,
	pub displayName: String,
	pub hidden: usize,
	pub description: Option<String>,
	pub icon: String,
	pub icongray: String,
}

impl From<SteamAchievement> for GameAchievement
{
	fn from(value: SteamAchievement) -> Self
	{
		return Self
		{
			name: value.id,
			displayName: value.name,
			description: match value.description.is_empty()
			{
				true => None,
				false => Some(value.description),
			},
			icon: value.iconUrl,
			icongray: value.iconLockedUrl,
			..Default::default()
		};
	}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameAvailableStats
{
	pub achievements: Option<Vec<GameAchievement>>,
	pub stats: Option<Vec<GameStat>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameSchema
{
	pub availableGameStats: GameAvailableStats,
	pub gameName: String,
	pub gameVersion: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameStat
{
	pub name: String,
	pub defaultvalue: isize,
	pub displayName: String,
}

/**
The expected response data returned by the GetSchemaForGame endpoint.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload
{
	pub game: GameSchema,
}
