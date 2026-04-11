use net::DataRequest;
use super::user::EgsUser;

#[derive(Clone, Debug)]
pub struct EgsOperationResult
{
	pub user: EgsUser,
	pub requests: Vec<DataRequest>,
}

impl From<EgsUser> for EgsOperationResult
{
	fn from(value: EgsUser) -> Self
	{
		return Self
		{
			user: value,
			requests: Default::default(),
		};
	}
}
