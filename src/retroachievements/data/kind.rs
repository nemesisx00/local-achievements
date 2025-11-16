use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum AwardKind
{
	#[default]
	BeatenCasual,
	BeatenHardcore,
	Completed,
	Mastered,
}

impl From<Number> for AwardKind
{
	fn from(value: Number) -> Self
	{
		return match value.as_u64()
		{
			None => Self::default(),
			Some(num) => match num
			{
				0 => Self::BeatenCasual,
				1 => Self::BeatenHardcore,
				2 => Self::Completed,
				3 => Self::Mastered,
				_ => Self::default(),
			}
		};
	}
}

impl AwardKind
{
	pub fn parse(value: &String) -> Option<Self>
	{
		return match value.as_str()
		{
			"beaten-casual" => Some(Self::BeatenCasual),
			"beaten-hardcore" => Some(Self::BeatenHardcore),
			"completed" => Some(Self::Completed),
			"mastered" => Some(Self::Mastered),
			_=> None,
		};
	}
}
