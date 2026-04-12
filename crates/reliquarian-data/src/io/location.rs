use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FileLocation
{
	pub fileName: String,
	pub group: String,
	pub platform: String,
}

impl Display for FileLocation
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		return write!(
			f,
			"{}:{}:{}",
			self.platform.to_lowercase(),
			self.group,
			self.fileName
		);
	}
}
