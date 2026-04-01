use crate::retroachievements::RetroAchievementsProgressState;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RetroAchievementsOperation
{
	GetGameInfo(u64),
	GetUserProfile,
	GetUserProgress(RetroAchievementsProgressState),
	SaveToFile,
}
