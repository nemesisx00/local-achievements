use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::platform::data::starcraft2::profile::profile::SnapshotSeason;
use super::league::LeagueSnapshot;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SeasonLeagues
{
	pub archon: LeagueSnapshot,
	/// 1v1
	pub one: LeagueSnapshot,
	/// 2v2
	pub two: LeagueSnapshot,
	/// 3v3
	pub three: LeagueSnapshot,
	/// 4v4
	pub four: LeagueSnapshot,
}

impl From<SnapshotSeason> for SeasonLeagues
{
	fn from(value: SnapshotSeason) -> Self
	{
		return Self
		{
			archon: value.archon.into(),
			one: value.one.into(),
			two: value.two.into(),
			three: value.three.into(),
			four: value.four.into(),
		};
	}
}

impl SeasonLeagues
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut season = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "archon")
		{
			if let Value::Object(inner) = value
			{
				season.archon = LeagueSnapshot::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "one")
		{
			if let Value::Object(inner) = value
			{
				season.one = LeagueSnapshot::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "two")
		{
			if let Value::Object(inner) = value
			{
				season.two = LeagueSnapshot::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "three")
		{
			if let Value::Object(inner) = value
			{
				season.three = LeagueSnapshot::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "four")
		{
			if let Value::Object(inner) = value
			{
				season.four = LeagueSnapshot::parseJsonMapLossy(inner);
			}
		}
		
		return season;
	}
}
