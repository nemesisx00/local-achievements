/*!
Data structures used to unify the disparate pieces of raw PS3 trophy data into
a more manageable form.
*/

use crate::rpcs3::TrophyGrade;
use crate::rpcs3::platform::data::conf::TrophyMetadata;

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct TrophyData
{
	pub detail: String,
	pub hidden: bool,
	pub id: u32,
	pub grade: TrophyGrade,
	pub name: String,
	/// Whether or not the trophy is relevant to Platinum the game. -1 == Not relevant
	pub pid: i32,
	pub unlockTimestamp: Option<u64>,
}

impl From<TrophyMetadata> for TrophyData
{
	fn from(value: TrophyMetadata) -> Self
	{
		return Self
		{
			detail: value.detail,
			grade: value.ttype.into(),
			hidden: value.hidden == TrophyMetadata::HiddenTrue,
			id: value.id,
			name: value.name,
			pid: value.pid,
			..Default::default()
		};
	}
}
