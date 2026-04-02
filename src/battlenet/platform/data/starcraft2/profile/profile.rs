use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PayloadProfile
{
	pub summary: Summary,
	pub snapshot: Snapshot,
	pub career: Career,
	pub swarmLevels: SwarmLevels,
	pub campaign: Campaign,
	pub categoryPointProgress: Vec<CategoryPointProgress>,
	pub achievementShowcase: Vec<String>,
	pub earnedRewards: Vec<EarnedReward>,
	pub earnedAchievements: Vec<EarnedAchievement>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Campaign
{
	pub difficultyCompleted: CampaignDifficultyComplete,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CampaignDifficultyComplete
{
	#[serde(alias = "wings-of-liberty")]
	pub wingsOfLiberty: String,
	
	#[serde(alias = "heart-of-the-swarm")]
	pub heartOfTheSwarm: String,
	
	#[serde(alias = "legacy-of-the-void")]
	pub legacyOfTheVoid: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Career
{
	pub best1v1Finish: Option<LeagueFinish>,
	pub bestTeamFinish: Option<LeagueFinish>,
	pub current1v1LeagueName: Option<String>,
	pub currentBestTeamLeagueName: Option<String>,
	pub terranWins: u64,
	pub totalCareerGames: u64,
	pub totalGamesThisSeason: u64,
	pub protossWins: u64,
	pub zergWins: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct LeagueFinish
{
	pub leagueName: String,
	pub timesAchieved: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CategoryPointProgress
{
	pub categoryId: String,
	pub pointsEarned: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Criteria
{
	pub criterionId: String,
	pub earned: Option<CriteriaEarned>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CriteriaEarned
{
	pub quantity: u64,
	pub startTime: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EarnedAchievement
{
	pub achievementId: String,
	pub completionDate: u64,
	pub numCompletedAchievementsInSeries: u64,
	pub totalAchievementsInSeries: u64,
	pub isComplete: bool,
	pub inProgress: bool,
	pub criteria: Vec<Criteria>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EarnedReward
{
	pub achievementId: Option<String>,
	pub category: Option<String>,
	pub rewardId: String,
	pub selected: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Snapshot
{
	pub seasonSnapshot: SnapshotSeason,
	pub totalRankedSeasonGamesPlayed: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SnapshotSeason
{
	#[serde(alias = "Archon")]
	pub archon: SnapshotSeasonLeague,
	
	#[serde(alias = "1v1")]
	pub one: SnapshotSeasonLeague,
	
	#[serde(alias = "2v2")]
	pub two: SnapshotSeasonLeague,
	
	#[serde(alias = "3v3")]
	pub three: SnapshotSeasonLeague,
	
	#[serde(alias = "4v4")]
	pub four: SnapshotSeasonLeague,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SnapshotSeasonLeague
{
	pub leagueName: Option<String>,
	pub rank: i64,
	pub totalGames: u64,
	pub totalWins: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Summary
{
	pub decalProtoss: String,
	pub decalTerran: String,
	pub decalZerg: String,
	pub displayName: String,
	pub id: String,
	pub realm: u64,
	pub totalAchievementPoints: u64,
	pub totalSwarmLevel: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SwarmLevels
{
	pub level: u64,
	pub terran: SwarmLevel,
	pub protoss: SwarmLevel,
	pub zerg: SwarmLevel,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SwarmLevel
{
	pub currentLevelPoints: u64,
	pub level: u64,
	pub maxLevelPoints: u64,
}
