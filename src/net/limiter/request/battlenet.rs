
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum BattleNetOperation
{
	GetSc2PlayerAccount,
	GetSc2PlayerProfile,
	GetSc2StaticProfile,
	GetUserInfo,
	SaveToFile,
}
