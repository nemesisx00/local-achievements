use std::cmp::Ordering;
use super::{BattleNetOperation, EpicGamesStoreOperation, GogOperation,
	RetroAchievementsOperation, SteamOperation};

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
pub enum DataOperation
{
	BattleNet(BattleNetOperation),
	CacheImage(bool),
	EpicGamesStore(EpicGamesStoreOperation),
	Gog(GogOperation),
	RetroAchievements(RetroAchievementsOperation),
	Steam(SteamOperation),
}

impl Default for DataOperation
{
	fn default() -> Self
	{
		return Self::CacheImage(false);
	}
}

impl PartialOrd for DataOperation
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self
		{
			DataOperation::CacheImage(_) => match other
			{
				DataOperation::CacheImage(_) => Some(Ordering::Equal),
				_ => Some(Ordering::Greater),
			},
			
			_ => match other
			{
				DataOperation::CacheImage(_) => Some(Ordering::Less),
				_ => Some(Ordering::Equal),
			},
		};
	}
}
