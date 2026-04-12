use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Clone, Copy, Debug, Default, Deserialize, EnumString, PartialEq, Serialize)]
pub enum DifficultyLevel
{
	#[default]
	#[strum(serialize = "CASUAL", serialize = "Casual")]
	Casual,
	#[strum(serialize = "NORMAL", serialize = "Normal")]
	Normal,
	#[strum(serialize = "HARD", serialize = "Hard")]
	Hard,
	#[strum(serialize = "BRUTAL", serialize = "Brutal")]
	Brutal,
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, PartialEq)]
pub enum SeasonLeagueType
{
	Archon,
	#[default]
	#[strum(to_string = "1v1")]
	One,
	#[strum(to_string = "2v2")]
	Two,
	#[strum(to_string = "3v3")]
	Three,
	#[strum(to_string = "4v4")]
	Four,
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, Deserialize, EnumString, PartialEq, Serialize)]
pub enum LeagueName
{
	#[default]
	#[strum(serialize = "PRACTICE", serialize = "Practice")]
	Practice,
	#[strum(serialize = "BRONZE", serialize = "Bronze")]
	Bronze,
	#[strum(serialize = "SILVER", serialize = "Silver")]
	Silver,
	#[strum(serialize = "GOLD", serialize = "Gold")]
	Gold,
	#[strum(serialize = "PLATINUM", serialize = "Platinum")]
	Platinum,
	#[strum(serialize = "DIAMOND", serialize = "Diamond")]
	Diamond,
	#[strum(serialize = "MASTER", serialize = "Master")]
	Master,
	#[strum(serialize = "GRANDMASTER", serialize = "Grandmaster")]
	Grandmaster,
}
