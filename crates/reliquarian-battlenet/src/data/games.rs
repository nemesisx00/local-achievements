use data::enums::GamePlatforms;
use freya::radio::RadioChannel;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, IntoStaticStr};

#[derive(AsRefStr, Debug, Default, Copy, Clone, Deserialize, EnumString, IntoStaticStr, PartialEq, Serialize)]
pub enum BattleNetGames
{
	#[default]
	None,
	#[strum(to_string = "Diablo III")]
	Diablo3,
	//Hearthstone,
	#[strum(to_string = "StarCraft II")]
	StarCraft2,
	#[strum(to_string = "World of Warcraft")]
	WorldOfWarcraft,
	#[strum(to_string = "World of Warcraft (Classic)")]
	WorldOfWarcraftClassic,
}

impl RadioChannel<BattleNetGames> for GamePlatforms {}
