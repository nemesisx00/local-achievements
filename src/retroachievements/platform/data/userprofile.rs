use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub ContribCount: u64,
	pub ContribYield: u64,
	pub ID: u64,
	pub LastGameID: u64,
	pub MemberSince: String,
	pub Motto: String,
	pub Permissions: u64,
	pub RichPresenceMsg: String,
	pub TotalPoints: u64,
	pub TotalSoftcorePoints: u64,
	pub TotalTruePoints: u64,
	pub ULID: String,
	pub Untracked: u64,
	pub User: String,
	pub UserPic: String,
	pub UserWallActive: bool,
}
