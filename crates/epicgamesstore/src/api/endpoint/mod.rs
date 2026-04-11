pub mod achievement;
pub mod player;
pub mod private;
pub mod progress;
pub mod unified;

use serde::Serialize;

pub trait Variables : Serialize {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Extensions
{
	pub persistedQuery: PersistedQuery,
}

impl From<String> for Extensions
{
	fn from(value: String) -> Self
	{
		return Self
		{
			persistedQuery: PersistedQuery
			{
				sha256Hash: value.clone(),
				..Default::default()
			}
		}
	}
}

#[derive(Clone, Debug, Serialize)]
pub struct PersistedQuery
{
	pub sha256Hash: String,
	pub version: u64,
}

impl Default for PersistedQuery
{
	fn default() -> Self
	{
		return Self
		{
			sha256Hash: Default::default(),
			version: 1,
		};
	}
}
