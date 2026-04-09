
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum EpicGamesStoreOperation
{
	GetAchievementsList(String),
	GetAchievementProgress(String),
	GetPlayerProfile,
	GetPlayerProfilePrivate,
	SaveToFile,
}
