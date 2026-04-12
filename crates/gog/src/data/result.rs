use net::DataRequest;
use super::user::GogUser;

#[derive(Clone, Debug)]
pub struct GogOperationResult
{
	pub user: GogUser,
	pub requests: Vec<DataRequest>,
}

impl From<GogUser> for GogOperationResult
{
	fn from(value: GogUser) -> Self
	{
		return Self
		{
			user: value,
			requests: Default::default(),
		};
	}
}
