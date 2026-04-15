use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IntOrStringInt
{
	Int(u64),
	StringInt(String),
}

impl Default for IntOrStringInt
{
	fn default() -> Self
	{
		return Self::Int(u64::default());
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SystemRequirementsOrNot
{
	Not(Vec<u64>),
	Some(SystemRequirements),
}

impl Default for SystemRequirementsOrNot
{
	fn default() -> Self
	{
		return Self::Not(vec![]);
	}
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Payload_GetAppInfo
{
	pub data: Option<AppInfoData>,
	pub success: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AppInfoData
{
	pub about_the_game: Option<String>,
	pub achievements: Option<AchievementsData>,
	pub background: Option<String>,
	pub background_raw: Option<String>,
	pub capsule_image: Option<String>,
	pub capsule_imagev5: Option<String>,
	pub categories:  Option<Vec<Category>>,
	pub content_descriptors: Option<ContentDescriptors>,
	pub controller_support: Option<String>,
	pub detailed_description: Option<String>,
	pub developers:  Option<Vec<String>>,
	pub dlc: Option<Vec<u64>>,
	pub genres:  Option<Vec<Genre>>,
	pub header_image: Option<String>,
	pub is_free: Option<bool>,
	pub legal_notice: Option<String>,
	pub linux_requirements:  Option<SystemRequirementsOrNot>,
	pub mac_requirements:  Option<SystemRequirementsOrNot>,
	pub movies:  Option<Vec<Movie>>,
	pub name: Option<String>,
	pub package_groups:  Option<Vec<PackageGroup>>,
	pub packages:  Option<Vec<u64>>,
	pub pc_requirements: Option<SystemRequirementsOrNot>,
	pub platforms: Option<PlatformSupport>,
	pub price_overview: Option<PriceOverview>,
	pub publishers:  Option<Vec<String>>,
	pub ratings: Option<Ratings>,
	pub recommendations: Option<Recommendations>,
	pub release_date: Option<ReleaseDate>,
	pub required_age: Option<IntOrStringInt>,
	pub screenshots:  Option<Vec<Screenshot>>,
	pub short_description: Option<String>,
	pub steam_appid: Option<IntOrStringInt>,
	pub support_info: Option<SupportInfo>,
	pub supported_languages: Option<String>,
	pub r#type: Option<String>,
	pub website: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SystemRequirements
{
	pub minimum: Option<String>,
	pub recommended: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PriceOverview
{
	pub currency: Option<String>,
	pub discount_percent: Option<IntOrStringInt>,
	pub r#final: Option<IntOrStringInt>,
	pub final_formatted: Option<String>,
	pub initial: Option<IntOrStringInt>,
	pub initial_formatted: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PackageGroup
{
	pub description: Option<String>,
	pub display_type: Option<IntOrStringInt>,
	pub is_recurring_subscription: Option<String>,
	pub name: Option<String>,
	pub save_text: Option<String>,
	pub selection_text: Option<String>,
	pub subs: Vec<PackageSubgroup>,
	pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PackageSubgroup
{
	pub can_get_free_license: Option<String>,
	pub is_free_license: Option<bool>,
	pub option_description: Option<String>,
	pub option_text: Option<String>,
	pub packageid: Option<IntOrStringInt>,
	pub percent_savings: Option<IntOrStringInt>,
	pub percent_savings_text: Option<String>,
	pub price_in_cents_with_discount: Option<IntOrStringInt>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PlatformSupport
{
	pub linux: Option<bool>,
	pub mac: Option<bool>,
	pub windows: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Category
{
	pub description: Option<String>,
	pub id: Option<IntOrStringInt>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Genre
{
	pub description: Option<String>,
	pub id: Option<IntOrStringInt>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Screenshot
{
	pub id: Option<IntOrStringInt>,
	pub path_full: Option<String>,
	pub path_thumbnail: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Movie
{
	pub dash_av1: Option<String>,
	pub dash_h264: Option<String>,
	pub highlight: Option<bool>,
	pub hls_h264: Option<String>,
	pub id: Option<IntOrStringInt>,
	pub name: Option<String>,
	pub thumbnail: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Recommendations
{
	pub total: Option<IntOrStringInt>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AchievementsData
{
	pub highlighted: Option<Vec<HighlightedAchievement>>,
	pub total: Option<IntOrStringInt>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct HighlightedAchievement
{
	pub icon: Option<String>,
	pub localized_name: Option<String>,
	pub name: Option<String>,
	pub path: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ReleaseDate
{
	pub coming_soon: Option<bool>,
	pub date: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SupportInfo
{
	pub email: Option<String>,
	pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ContentDescriptors
{
	pub ids: Option<Vec<IntOrStringInt>>,
	pub notes: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Ratings
{
	pub cero: Option<RatingsCero>,
	pub crl: Option<RatingsCrl>,
	pub csrr: Option<RatingsCsrr>,
	pub dejus: Option<RatingsDejus>,
	pub esrb: Option<RatingsEsrb>,
	pub igrs: Option<RatingsIgrs>,
	pub kgrb: Option<RatingsKgrb>,
	pub oflc: Option<RatingsOflc>,
	pub pegi: Option<RatingsPegi>,
	pub steam_germany: Option<RatingsSteamGermany>,
	pub usk: Option<RatingsUsk>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsEsrb
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsPegi
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsUsk
{
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsOflc
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsCero
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsKgrb
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsDejus
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsCrl
{
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsCsrr
{
	pub descriptors: Option<String>,
	pub rating: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsSteamGermany
{
	pub banned: Option<String>,
	pub descriptors: Option<String>,
	pub rating: Option<String>,
	pub rating_generated: Option<String>,
	pub required_age: Option<String>,
	pub use_age_gate: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RatingsIgrs
{
	pub banned: Option<String>,
	pub descriptors: Option<String>,
	pub rating: Option<String>,
	pub rating_generated: Option<String>,
	pub required_age: Option<String>,
	pub use_age_gate: Option<String>,
}
