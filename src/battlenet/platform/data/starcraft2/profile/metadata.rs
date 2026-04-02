use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PayloadStatic
{
	pub achievements: Vec<AchievementMetadata>,
	pub categories: Vec<CategoryMetadata>,
	pub criteria: Vec<CriteriaMetadata>,
	pub rewards: Vec<RewardMetadata>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AchievementMetadata
{
	pub categoryId: String,
	pub chainAchievementIds: Vec<String>,
	pub chainRewardSize: u64,
	pub criteriaIds: Option<Vec<String>>,
	pub description: String,
	pub flags: u64,
	pub id: String,
	pub imageUrl: String,
	pub isChained: bool,
	pub points: u64,
	pub title: String,
	pub uiOrderHint: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CategoryMetadata
{
	pub childrenCategoryIds: Vec<String>,
	pub featuredAchievementId: String,
	pub id: String,
	pub medalTiers: Option<Vec<u64>>,
	pub name: String,
	pub parentCategoryId: Option<String>,
	pub points: u64,
	pub uiOrderHint: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CriteriaMetadata
{
	pub achievementId: Option<String>,
	pub description: String,
	pub evaluationClass: String,
	pub flags: u64,
	pub id: String,
	pub necessaryQuantity: u64,
	pub uiOrderHint: u64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RewardMetadata
{
	pub achievementId: Option<String>,
	pub id: String,
	pub imageUrl: String,
	pub isSkin: bool,
	pub flags: u64,
	pub name: String,
	pub unlockableType: String,
	pub uiOrderHint: u64,
}
