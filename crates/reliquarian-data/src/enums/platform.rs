use freya::radio::RadioChannel;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
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

impl RadioChannel<Option<String>> for GamePlatforms {}
impl RadioChannel<Option<u64>> for GamePlatforms {}
