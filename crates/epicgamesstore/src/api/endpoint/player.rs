use serde::{Deserialize, Serialize};
use super::unified::Avatars;
use super::Variables;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Payload_PlayerProfile
{
	pub data: PlayerProfileData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfile
{
	pub epicAccountId: String,
	pub displayName: String,
	pub avatar: Avatars,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileContainer
{
	pub playerProfile: PlayerProfile,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerProfileData
{
	pub PlayerProfile: PlayerProfileContainer,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlayerProfileVariables
{
	pub epicAccountId: String,
}

impl Variables for PlayerProfileVariables {}
