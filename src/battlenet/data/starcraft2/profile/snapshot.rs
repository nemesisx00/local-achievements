use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LeagueSnapshot
{
	pub leagueName: Option<String>,
	pub rank: i64,
	pub totalGames: u64,
	pub totalWins: u64,
}

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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Snapshot
{
	leagues: SeasonLeagues,
	/// The total number of games played in the current ranked season
	totalGamesPlayed: u64,
}
