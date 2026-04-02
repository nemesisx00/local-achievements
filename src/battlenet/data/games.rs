use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Default, Copy, Clone, Deserialize, EnumString, PartialEq, Serialize)]
pub enum Games
{
	Diablo3,
	//Hearthstone,
	#[default]
	StarCraft2,
	//WorldOfWarcraft,
	//WorldOfWarcraftClassic,
}
