use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payload
{
	pub ContribCount: usize,
	pub ContribYield: usize,
	pub ID: usize,
	pub LastGameID: usize,
	pub MemberSince: String,
	pub Motto: String,
	pub Permissions: usize,
	pub RichPresenceMsg: String,
	pub TotalPoints: usize,
	pub TotalSoftcorePoints: usize,
	pub TotalTruePoints: usize,
	pub ULID: String,
	pub Untracked: usize,
	pub User: String,
	pub UserPic: String,
	pub UserWallActive: bool,
}
