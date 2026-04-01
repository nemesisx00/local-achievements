use serde::{Deserialize, Serialize};
//use crate::battlenet::BattleNetAuth;
use crate::retroachievements::RetroAchievementsAuth;
use crate::steam::SteamAuth;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuthState
{
	//pub battleNet: BattleNetAuth,
	pub retroAchievements: RetroAchievementsAuth,
	pub steam: SteamAuth,
}
