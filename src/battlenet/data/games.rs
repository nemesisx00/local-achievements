use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, IntoStaticStr};

#[derive(AsRefStr, Debug, Default, Copy, Clone, Deserialize, EnumString, IntoStaticStr, PartialEq, Serialize)]
pub enum Games
{
	#[strum(to_string = "Diablo III")]
	Diablo3,
	//Hearthstone,
	#[default]
	#[strum(to_string = "StarCraft II")]
	StarCraft2,
	#[strum(to_string = "World of Warcraft")]
	WorldOfWarcraft,
	#[strum(to_string = "World of Warcraft (Classic)")]
	WorldOfWarcraftClassic,
}
