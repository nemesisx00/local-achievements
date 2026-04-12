use net::DataRequest;
use super::user::BattleNetUser;

#[derive(Clone, Debug)]
pub struct BattleNetOperationResult
{
	pub user: BattleNetUser,
	pub requests: Vec<DataRequest>,
}

impl From<BattleNetUser> for BattleNetOperationResult
{
	fn from(value: BattleNetUser) -> Self
	{
		return Self
		{
			user: value,
			requests: Default::default(),
		};
	}
}
