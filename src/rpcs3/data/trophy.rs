use anyhow::{anyhow, Result};
use chrono::{TimeZone, Utc, offset::LocalResult};
use serde::{Deserialize, Serialize};
use crate::constants::Format_ChronoDateTime;
use crate::rpcs3::platform::data::conf::TrophyMetadata;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Trophy
{
	pub detail: String,
	pub hidden: bool,
	pub id: u32,
	pub grade: TrophyGrade,
	pub name: String,
	pub platinumRelevance: bool,
	pub unlocked: bool,
	
	/// The timestamp, in microseconds, when the trophy was unlocked.
	pub unlockedTimestamp: Option<u64>,
}

impl From<TrophyMetadata> for Trophy
{
	fn from(value: TrophyMetadata) -> Self
	{
		return Self
		{
			detail: value.detail.to_owned(),
			grade: value.ttype.into(),
			hidden: value.hidden == TrophyMetadata::HiddenTrue,
			id: value.id.to_owned(),
			name: value.name.to_owned(),
			platinumRelevance: value.pid >= 0,
			..Default::default()
		};
	}
}

impl PartialOrd for Trophy
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.id.partial_cmp(&other.id);
	}
}

impl Trophy
{
	pub fn formatUnlockedTimestamp(&self) -> Result<String>
	{
		if let Some(timestamp) = self.unlockedTimestamp
		{
			return match Utc.timestamp_micros(timestamp as i64)
			{
				LocalResult::Ambiguous(earliest, _) => Ok(earliest.format(Format_ChronoDateTime).to_string()),
				LocalResult::Single(dt) => Ok(dt.format(Format_ChronoDateTime).to_string()),
				LocalResult::None => Err(anyhow!("Error parsing timestamp value: {}", timestamp)),
			}
		}
		
		return Err(anyhow!("Trophy not unlocked"));
	}
	
	pub fn update(&mut self, other: &Self)
	{
		self.detail = other.detail.to_owned();
		self.hidden = other.hidden;
		self.id = other.id;
		self.grade = other.grade;
		self.name = other.name.to_owned();
		self.platinumRelevance = other.platinumRelevance;
		self.unlocked = other.unlocked;
		self.unlockedTimestamp = other.unlockedTimestamp;
	}
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum TrophyGrade
{
	#[default]
	Unknown,
	Platinum,
	Gold,
	Silver,
	Bronze,
}

impl From<u32> for TrophyGrade
{
	fn from(value: u32) -> Self
	{
		return match value
		{
			1 => Self::Platinum,
			2 => Self::Gold,
			3 => Self::Silver,
			4 => Self::Bronze,
			_ => Self::Unknown,
		};
	}
}

impl From<String> for TrophyGrade
{
	fn from(value: String) -> Self
	{
		return value.as_str().into();
	}
}

impl From<&str> for TrophyGrade
{
	fn from(value: &str) -> Self
	{
		return match value
		{
			"B" => Self::Bronze,
			"G" => Self::Gold,
			"P" => Self::Platinum,
			"S" => Self::Silver,
			_ => Self::Unknown,
		};
	}
}

impl TrophyGrade
{
	pub fn points(&self) -> u64
	{
		return match self
		{
			Self::Bronze => 15,
			Self::Silver => 30,
			Self::Gold => 90,
			Self::Platinum => 180,
			_ => 0,
		};
	}
}
