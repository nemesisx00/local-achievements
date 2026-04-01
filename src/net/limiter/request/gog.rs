
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum GogOperation
{
	GetAchievements(u64),
	GetFilteredProducts(Option<u64>),
	GetUserInfo,
	RefreshSession,
	SaveToFile,
}
