use crate::data::AppData;
use crate::net::limiter::request::DataRequest;

#[derive(Clone, Debug)]
pub struct DataOperationResult
{
	pub appData: AppData,
	pub requests: Vec<DataRequest>,
}

impl From<AppData> for DataOperationResult
{
	fn from(value: AppData) -> Self
	{
		return Self
		{
			appData: value,
			requests: Default::default(),
		};
	}
}
