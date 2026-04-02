use crate::battlenet::BattleNetGames;

#[derive(Clone, Debug, Default)]
pub struct SelectedGames
{
	pub battleNet: Option<BattleNetGames>,
	pub egs: Option<String>,
	pub gog: Option<u64>,
	pub retroAchievements: Option<u64>,
	pub rpcs3: Option<String>,
	pub steam: Option<u64>,
}
