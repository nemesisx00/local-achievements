use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::api::{Career, LeagueFinish};
use crate::data::starcraft2::enums::LeagueName;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CareerSummary
{
	pub best1v1: Option<CareerBestFinish>,
	pub bestTeam: Option<CareerBestFinish>,
	pub current1v1LeagueName: Option<String>,
	pub currentBestTeamLeagueName: Option<String>,
	pub totalCareerGames: u64,
	pub totalSeasonGames: u64,
	pub winsProtoss: u64,
	pub winsTerran: u64,
	pub winsZerg: u64,
}

impl From<Career> for CareerSummary
{
	fn from(value: Career) -> Self
	{
		return Self
		{
			best1v1: match value.best1v1Finish
			{
				None => None,
				Some(finish) => Some(finish.into()),
			},
			
			bestTeam: match value.bestTeamFinish
			{
				None => None,
				Some(finish) => Some(finish.into()),
			},
			
			current1v1LeagueName: value.current1v1LeagueName.clone(),
			currentBestTeamLeagueName: value.currentBestTeamLeagueName.clone(),
			totalCareerGames: value.totalCareerGames,
			totalSeasonGames: value.totalGamesThisSeason,
			winsProtoss: value.protossWins,
			winsTerran: value.terranWins,
			winsZerg: value.zergWins,
		};
	}
}

impl CareerSummary
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut summary = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "best1v1")
		{
			if let Value::Object(inner) = value
			{
				summary.best1v1 = Some(CareerBestFinish::parseJsonMapLossy(inner));
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "bestTeam")
		{
			if let Value::Object(inner) = value
			{
				summary.bestTeam = Some(CareerBestFinish::parseJsonMapLossy(inner));
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "current1v1LeagueName")
		{
			if let Value::String(inner) = value
			{
				summary.current1v1LeagueName = Some(inner.clone());
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "currentBestTeamLeagueName")
		{
			if let Value::String(inner) = value
			{
				summary.currentBestTeamLeagueName = Some(inner.clone());
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalCareerGames")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.totalCareerGames = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "totalSeasonGames")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.totalSeasonGames = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "winsProtoss")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.winsProtoss = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "winsTerran")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.winsTerran = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "winsZerg")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.winsZerg = number;
				}
			}
		}
		
		return summary;
	}
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CareerBestFinish
{
	pub leagueName: Option<LeagueName>,
	pub timesAchieved: u64,
}

impl From<LeagueFinish> for CareerBestFinish
{
	fn from(value: LeagueFinish) -> Self
	{
		return Self
		{
			leagueName: match LeagueName::from_str(&value.leagueName)
			{
				Err(_) => None,
				Ok(ln) => Some(ln),
			},
			timesAchieved: value.timesAchieved,
		};
	}
}

impl CareerBestFinish
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Self
	{
		let mut summary = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "leagueName")
		{
			if let Value::String(inner) = value
			{
				if let Ok(league) = LeagueName::from_str(inner)
				{
					summary.leagueName = Some(league);
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "timesAchieved")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					summary.timesAchieved = number;
				}
			}
		}
		
		return summary;
	}
}
