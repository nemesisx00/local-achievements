use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::battlenet::platform::data::starcraft2::profile::profile::SnapshotSeasonLeague;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LeagueSnapshot
{
	pub leagueName: Option<String>,
	pub rank: i64,
	pub totalGames: u64,
	pub totalWins: u64,
}

impl From<SnapshotSeasonLeague> for LeagueSnapshot
{
	fn from(value: SnapshotSeasonLeague) -> Self
	{
		return Self
		{
			leagueName: value.leagueName.clone(),
			rank: value.rank,
			totalGames: value.totalGames,
			totalWins: value.totalWins,
		};
	}
}

impl LeagueSnapshot
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut league = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "leagueName")
		{
			if let Value::String(inner) = value
			{
				league.leagueName = Some(inner.clone());
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "rank")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_i64()
				{
					league.rank = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalGames")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					league.totalGames = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalWins")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					league.totalWins = number;
				}
			}
		}
		
		return league;
	}
}
