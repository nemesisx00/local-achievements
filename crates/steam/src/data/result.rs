use net::DataRequest;
use super::user::SteamUser;

#[derive(Clone, Debug)]
pub struct SteamOperationResult
{
	pub user: SteamUser,
	pub requests: Vec<DataRequest>,
}

impl From<SteamUser> for SteamOperationResult
{
	fn from(value: SteamUser) -> Self
	{
		return Self
		{
			user: value,
			requests: Default::default(),
		};
	}
}
