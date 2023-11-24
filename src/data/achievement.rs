#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use ::serde::{Deserialize, Serialize};
use crate::platforms::Platform;

/**
The mode representing the conditions under which an achievment was unlocked.

*Only used by: RetroAchievements*
*/
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Mode
{
	Hardcore,
	Softcore,
}

/**
A single achievement, along with platform-specific details.
*/
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Achievement
{
	/// The platform-specific details about when this achievement was unlocked.
	pub platforms: Vec<PlatformInfo>,
}

impl Achievement
{
	pub fn new(platform: PlatformInfo) -> Self
	{
		let mut instance = Self::default();
		instance.platforms.push(platform);
		return instance;
	}
	
	pub fn getIds(&self) -> Vec<String>
	{
		let mut ids = vec![];
		for pi in &self.platforms
		{
			ids.push(pi.id.to_owned());
		}
		return ids;
	}
	
	pub fn update(&mut self, other: Achievement)
	{
		for pi in other.platforms
		{
			match self.platforms.iter_mut()
				.find(|p| p.platform == pi.platform)
				.as_mut()
			{
				Some(info) => info.update(pi),
				None => self.platforms.push(pi),
			}
		}
	}
	
	/**
	Does this achievement have a global percentage value for a specific `Platform`?
	*/
	pub fn hasGlobalPercentage(&self, platform: Platform) -> bool
	{
		return self.platforms
			.iter()
			.find(|pi| pi.platform == platform)
			.is_some_and(|pi| pi.globalPercentage != None);
	}
	
	/**
	Is this achievement unlocked on at least one platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.platforms
			.iter()
			.any(|pi| pi.isUnlocked());
	}
	
	/**
	Is this achievement unlocked on every platform?
	*/
	pub fn isUnlockedAll(&self) -> bool
	{
		return self.platforms
			.iter()
			.all(|pi| pi.isUnlocked());
	}
	
	/**
	
	*/
	pub fn updateGlobalPercentage(&mut self, platform: Platform, percentage: f64)
	{
		match self.platforms.iter_mut().find(|pi| pi.platform == platform)
		{
			Some(info) => info.globalPercentage = Some(percentage),
			None => {
				/*
				let mut info = PlatformInfo::new(platform);
				info.globalPercentage = Some(percentage);
				self.details.push(info);
				*/
			},
		}
	}
}

/**
Platform-specific information about an unlocked achievement.
*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlatformInfo
{
	/// The human-readable description of this achievement.
	pub description: String,
	
	/// The percentage of users on this platform who have unlocked this achievement.
	pub globalPercentage: Option<f64>,
	
	/// The platform-specific ID of this achievement.
	pub id: String,
	
	/// The path to the platform's associated icon for this achievement.
	pub icon: Option<String>,
	
	/// The path to the platform's associated locked icon for this achievement.
	pub iconLocked: Option<String>,
	
	/// The mode under which this achievement was unlocked.
	pub mode: Option<Mode>,
	
	/// The human-readable name of this achievement.
	pub name: String,
	
	/// The platform on which this achievement was unlocked.
	pub platform: Platform,
	
	/// The points awarded when this achievement is unlocked.
	pub points: Option<HashMap<Mode, usize>>,
	
	/// The timestamp at which the achievement was unlocked.
	pub timestamp: Option<usize>,
}

impl PlatformInfo
{
	/**
	
	*/
	pub fn new(id: String, name: String, description: String, platform: Platform) -> Self
	{
		return Self
		{
			description,
			globalPercentage: None,
			id,
			icon: None,
			iconLocked: None,
			mode: None,
			name,
			platform,
			points: None,
			timestamp: None,
		}
	}
	
	pub fn update(&mut self, info: PlatformInfo)
	{
		self.description = info.description.to_owned();
		self.globalPercentage = info.globalPercentage.to_owned();
		self.icon = info.icon.to_owned();
		self.iconLocked = info.iconLocked.to_owned();
		self.id = info.id.to_owned();
		self.mode = info.mode.to_owned();
		self.name = info.name.to_owned();
		self.platform = info.platform.to_owned();
		self.points = info.points.to_owned();
		self.timestamp = info.timestamp.to_owned();
	}
	
	/**
	Is this achievement unlocked on this platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.timestamp.is_some();
	}
}

#[cfg(test)]
mod tests
{
    use super::*;
	use std::collections::HashMap;
	use crate::data::PlatformInfo;
	use crate::data::achievement::Mode;
	
	fn setupAchievement(name: &str, platform: Platform, hcPoints: usize, scPoints: usize, mode: Option<Mode>) -> Achievement
	{
		let mut points = HashMap::new();
		points.insert(Mode::Hardcore, hcPoints);
		points.insert(Mode::Softcore, scPoints);
		
		let achievement = Achievement::new(PlatformInfo
			{
				description: String::default(),
				globalPercentage: None,
				id: String::default(),
				icon: None,
				iconLocked: None,
				mode: mode,
				name: name.to_string(),
				platform: platform,
				points: Some(points),
				timestamp: match mode
				{
					Some(_) => Some(1),
					None => None,
				}
			});
		
		return achievement;
	}
	
	#[test]
	fn GlobalPercentage()
	{
		let mut a1 = setupAchievement("A1", Platform::Steam, 0, 0, None);
		
		assert!(!a1.hasGlobalPercentage(Platform::Steam));
		a1.platforms.iter_mut().for_each(|d| d.globalPercentage = Some(25.0));
		assert!(a1.hasGlobalPercentage(Platform::Steam));
	}
}
