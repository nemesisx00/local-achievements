use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::platform::data::starcraft2::profile::profile::ProfileSnapshot;
use super::season::SeasonLeagues;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Snapshot
{
	pub leagues: SeasonLeagues,
	/// The total number of games played in the current ranked season
	pub	totalGamesPlayed: u64,
}

impl From<ProfileSnapshot> for Snapshot
{
	fn from(value: ProfileSnapshot) -> Self
	{
		return Self
		{
			leagues: value.seasonSnapshot.into(),
			totalGamesPlayed: value.totalRankedSeasonGamesPlayed,
		};
	}
}

impl Snapshot
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut snapshot = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "achievements")
		{
			if let Value::Object(inner) = value
			{
				snapshot.leagues = SeasonLeagues::parseJsonMapLossy(inner);
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalGamesPlayed")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					snapshot.totalGamesPlayed = number;
				}
			}
		}
		
		return snapshot;
	}
}
