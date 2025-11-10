use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::rpcs3::platform::data::conf::TrophyConf;

use super::trophy::Trophy;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Game
{
	pub detail: String,
	pub name: String,
	pub npCommId: String,
	pub parentalLevel: i32,
	pub trophies: Vec<Trophy>,
	pub trophySetVersion: String,
}

impl From<TrophyConf> for Game
{
	fn from(value: TrophyConf) -> Self
	{
		return Self
		{
			detail: value.titleDetail.to_owned(),
			name: value.titleName.to_owned(),
			npCommId: value.npcommid.to_owned(),
			parentalLevel: value.parentalLevel.value,
			trophies: value.trophies.iter()
				.cloned()
				.map(|t| t.into())
				.collect(),
			trophySetVersion: value.trophysetVersion.to_owned(),
		};
	}
}

impl PartialOrd for Game
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.name.partial_cmp(&other.name);
	}
}

impl Game
{
	const MaxPoints_Disc: u64 = 1230;
	#[allow(unused)]
	const MaxPoints_Psn: u64 = 315;
	#[allow(unused)]
	const MaxPoints_Dlc: u64 = 200;
	
	pub fn percentUnlocked(&self) -> f32
	{
		return (self.trophies.iter()
					.filter(|t| t.unlocked == true)
					.count() as f32
				/ self.trophies.len() as f32)
			* 100f32;
	}
	
	pub fn points(&self) -> u64
	{
		let points = self.trophies.iter()
			.fold(0, |acc, t| acc + t.grade.points());
		
		return match points > Self::MaxPoints_Disc
		{
			false => points,
			true => Self::MaxPoints_Disc,
		};
	}
}
