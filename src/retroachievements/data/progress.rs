use crate::retroachievements::RetroAchievementsApi;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct RetroAchievementsProgressState
{
	pub offset: u64,
	pub received: u64,
	pub total: u64,
}

impl RetroAchievementsProgressState
{
	pub fn reachedEnd(&self) -> bool
	{
		return self.received <= 0
			|| self.received >= self.total
			|| self.received % RetroAchievementsApi::GetUserGameCompletion_Count != 0;
	}
}
