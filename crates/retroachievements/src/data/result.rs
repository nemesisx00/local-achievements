use net::DataRequest;
use crate::data::user::RetroAchievementsUser;

#[derive(Clone, Debug)]
pub struct RetroAchievementsResult
{
	pub user: RetroAchievementsUser,
	pub requests: Vec<DataRequest>,
}

impl From<RetroAchievementsUser> for RetroAchievementsResult
{
	fn from(value: RetroAchievementsUser) -> Self
	{
		return Self
		{
			user: value,
			requests: Default::default(),
		};
	}
}
