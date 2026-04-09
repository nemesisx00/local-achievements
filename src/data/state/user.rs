use serde::{Deserialize, Serialize};
use crate::battlenet::BattleNetUser;
use crate::egs::EgsUser;
use crate::gog::GogUser;
use crate::retroachievements::RetroAchievementsUser;
use crate::rpcs3::Rpcs3User;
use crate::steam::SteamUser;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserState
{
	pub battleNet: BattleNetUser,
	pub egs: EgsUser,
	pub gog: GogUser,
	pub retroAchievements: RetroAchievementsUser,
	pub rpcs3: Rpcs3User,
	pub steam: SteamUser,
}
