use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AchievementMetadata
{
	pub achievement_id: String,
	pub achievement_key: String,
	pub visible: bool,
	pub name: String,
	pub description: String,
	pub image_url_unlocked: String,
	pub image_url_locked: String,
	pub date_unlocked: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Payload_Achievements
{
	pub total_count: u64,
	pub limit: u64,
	pub page_token: String,
	pub items: Vec<AchievementMetadata>,
}
