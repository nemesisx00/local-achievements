use serde::{Deserialize, Serialize};
use crate::enums::ActiveContent;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EnabledPlatforms
{
	#[serde(default)]
	pub battleNet: bool,
	
	#[serde(default)]
	pub epicGamesStores: bool,
	
	#[serde(default)]
	pub gog: bool,
	
	#[serde(default)]
	pub retroAchievements: bool,
	
	#[serde(default)]
	pub rpcs3: bool,
	
	#[serde(default)]
	pub steam: bool,
}

impl EnabledPlatforms
{
	pub fn isEnabled(&self, active: ActiveContent) -> bool
	{
		return match active
		{
			ActiveContent::Settings => true,
			
			ActiveContent::BattleNet => self.battleNet,
			ActiveContent::EpicGamesStore => self.epicGamesStores,
			ActiveContent::Gog => self.gog,
			ActiveContent::RetroAchievements => self.retroAchievements,
			ActiveContent::Rpcs3 => self.rpcs3,
			ActiveContent::Steam => self.steam,
		};
	}
}
