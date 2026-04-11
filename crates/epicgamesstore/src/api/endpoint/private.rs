use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::AsRefStr;
use super::Variables;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementSummary
{
	pub totalUnlocked: u64,
	pub totalXP: u64,
	pub sandboxId: String,
	pub baseOfferForSandbox: BaseOfferForSandbox,
	pub product: Product,
	pub productAchievements: ProductAchievements,
	pub playerAwards: Vec<Value>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AchievementsSummaries
{
	pub __typename: String,
	pub status: u64,
	pub data: Vec<AchievementSummary>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BaseOfferForSandbox
{
	pub keyImages: Vec<KeyImage>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeyImage
{
	pub url: String,
	pub r#type: KeyImageType,
	pub alt: String,
}

#[derive(AsRefStr, Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum KeyImageType
{
	#[serde(alias = "featuredMedia")]
	FeaturedMedia,
	#[serde(alias = "heroCarouselVideo")]
	HeroCarouselVideo,
	#[default]
	OfferImageWide,
	OfferImageTall,
	Thumbnail,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload_PlayerProfilePrivate
{
	pub data: PlayerProfileData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfile
{
	pub privacy: Option<String>,
	pub relationship: String,
	pub achievementsSummaries: AchievementsSummaries,
	pub friendsSummaries: Value,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileContainer
{
	pub playerProfile: PlayerProfile,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileData
{
	pub Friends: Value,
	pub PlayerProfile: PlayerProfileContainer,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Product
{
	pub name: String,
	pub slug: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductAchievements
{
	pub totalAchievements: u64,
	pub totalProductXP: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerProfilePrivateVariables
{
	pub accountId: String,
	pub epicAccountId: String,
	pub locale: String,
	pub page: u64,
}

impl Default for PlayerProfilePrivateVariables
{
	fn default() -> Self
	{
		return Self
		{
			epicAccountId: Default::default(),
			//TODO: Set up localization for the app
			locale: "en-US".to_string(),
			page: 1,
			accountId: Default::default(),
		};
	}
}

impl Variables for PlayerProfilePrivateVariables {}
