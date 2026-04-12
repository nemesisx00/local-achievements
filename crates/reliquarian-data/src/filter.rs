
pub trait Filterable<T>
{
	fn filter(&self, filter: impl Into<FilterCriteria>) -> Vec<T>;
}

#[derive(Clone, Debug, Default)]
pub struct FilterCriteria
{
	/// Should the text search be case sensitive
	pub caseSensitive: bool,
	
	/// Display only locked or unlocked
	pub locked: bool,
	
	/// Should the text be searched for in the name only
	pub nameOnly: bool,
	
	/// Search text to filter by name and description
	pub text: String,
}

impl From<String> for FilterCriteria
{
	fn from(value: String) -> Self
	{
		return Self
		{
			text: value,
			..Default::default()
		};
	}
}
