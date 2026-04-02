use crate::data::AppData;
use crate::net::limiter::request::RequestData;

#[derive(Clone, Debug)]
pub struct DataOperationResult
{
	pub appData: AppData,
	pub requests: Vec<RequestData>,
}
