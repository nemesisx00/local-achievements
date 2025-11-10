use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Copy, Default, Debug, Display, EnumIter, EnumString, PartialEq, PartialOrd)]
pub enum ActiveContent
{
	#[default]
	RetroAchievements,
	Rpcs3,
	Settings,
	Steam,
}
