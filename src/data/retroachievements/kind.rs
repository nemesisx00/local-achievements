use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum AwardKind
{
	#[default]
	BeatenSoftcore,
	BeatenHardcore,
	Completed,
	Mastered,
}

impl AwardKind
{
	pub fn parse(value: &String) -> Option<Self>
	{
		return match value.as_str()
		{
			"beaten-softcore" => Some(Self::BeatenSoftcore),
			"beaten-hardcore" => Some(Self::BeatenHardcore),
			"completed" => Some(Self::Completed),
			"mastered" => Some(Self::Mastered),
			_=> None,
		};
	}
}
