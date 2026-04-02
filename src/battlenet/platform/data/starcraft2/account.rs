use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PayloadPlayer
{
	pub avatarUrl: String,
	pub name: String,
	pub profileId: String,
	pub profileUrl: String,
	pub realmId: u64,
	pub regionId: u64,
}
