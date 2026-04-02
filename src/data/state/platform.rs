use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use crate::battlenet::BattleNetSettings;
//use crate::egs::EgsSettings;
use crate::rpcs3::Rpcs3Settings;

#[derive(AsRefStr, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GamePlatforms
{
	#[strum(to_string="Battle.Net")]
	BattleNet,
	#[strum(to_string="Epic Games Store")]
	EpicGamesStore,
	#[strum(to_string="GOG")]
	Gog,
	#[strum(to_string="Retro Achievements")]
	RetroAchievements,
	#[strum(to_string="RPCS3")]
	Rpcs3,
	Steam,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformState
{
	pub battleNet: BattleNetSettings,
	//pub egs: EgsSettings,
	pub rpcs3: Rpcs3Settings,
}
