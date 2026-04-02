use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct UserInfo
{
	pub battletag: String,
	pub id: u64,
	pub sub: String,
}
