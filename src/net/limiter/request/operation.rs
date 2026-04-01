use std::cmp::Ordering;
use crate::net::limiter::request::RetroAchievementsOperation;

use super::gog::GogOperation;
use super::steam::SteamOperation;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord)]
pub enum DataOperation
{
	#[default]
	CacheImage,
	Gog(GogOperation),
	RetroAchievements(RetroAchievementsOperation),
	Steam(SteamOperation),
}

impl PartialOrd for DataOperation
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return match self
		{
			DataOperation::CacheImage => match other
			{
				DataOperation::CacheImage => Some(Ordering::Equal),
				_ => Some(Ordering::Greater),
			},
			
			_ => match other
			{
				DataOperation::CacheImage => Some(Ordering::Less),
				_ => Some(Ordering::Equal),
			},
		};
	}
}
