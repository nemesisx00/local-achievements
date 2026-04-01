use serde::{Deserialize, Serialize};
//use crate::battlenet::{BattleNetAuth, BattleNetSession};
//use crate::egs::EgsSettings;
use crate::retroachievements::RetroAchievementsAuth;
use crate::rpcs3::Rpcs3Settings;
use crate::steam::SteamAuth;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GamePlatforms
{
	BattleNet,
	EpicGamesStore,
	Gog,
	RetroAchievements,
	Rpcs3,
	Steam,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformState
{
	//pub battleNetAuth: BattleNetAuth,
	//pub egs: EgsSettings,
	pub retroAchievements: RetroAchievementsAuth,
	pub rpcs3: Rpcs3Settings,
	pub steam: SteamAuth,
}
