use std::cmp::Ordering;
use data::io::FileLocation;
use super::operation::DataOperation;

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
pub struct DataRequest
{
	pub destination: Option<FileLocation>,
	pub operation: DataOperation,
	pub url: Option<String>,
}

impl Default for DataRequest
{
	fn default() -> Self
	{
		return Self
		{
			destination: Default::default(),
			operation: Default::default(),
			url: Default::default(),
		};
	}
}

impl PartialOrd for DataRequest
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.operation.partial_cmp(&other.operation);
	}
}
