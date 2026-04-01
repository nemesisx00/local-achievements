
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum SteamOperation
{
	GetGameList,
	GetGlobalPercentages(u64),
	GetPlayerAchievements(u64),
	GetPlayerSummary,
	GetSchemaForGame(u64),
	SaveToFile,
	SetGameLoaded(u64, bool),
}
