use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::constants::TheString;
use crate::retroachievements::platform::{GameMetadata, Payload_GetGameInfo};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct System
{
	pub id: usize,
	pub name: String,
}

impl From<GameMetadata> for System
{
	fn from(value: GameMetadata) -> Self
	{
		return Self
		{
			id: value.ConsoleID,
			name: value.ConsoleName.to_owned(),
		};
	}
}

impl From<Payload_GetGameInfo> for System
{
	fn from(value: Payload_GetGameInfo) -> Self
	{
		return Self
		{
			id: value.ConsoleID,
			name: value.ConsoleName.to_owned(),
		};
	}
}

impl PartialOrd for System
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.sortName().to_lowercase().partial_cmp(&other.sortName().to_lowercase());
	}
}

impl System
{
	pub fn sortName(&self) -> String
	{
		return match self.name.starts_with(TheString)
		{
			true => {
				let mut the = self.name.clone();
				let name = the.split_off(TheString.len());
				format!("{}, {}", name, the.trim())
			},
			
			false => self.name.to_owned(),
		};
	}
}
