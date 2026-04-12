use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Default, Debug, Deserialize, Display, EnumIter, EnumString, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum ActiveContent
{
	BattleNet,
	EpicGamesStore,
	Gog,
	RetroAchievements,
	Rpcs3,
	#[default]
	Settings,
	Steam,
}
