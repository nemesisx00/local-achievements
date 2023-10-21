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
	pub details: Vec<PlatformInfo>,
	
	/// The human-readable description of this achievement.
	pub description: String,
	
	/**
	Platform-specific human-readable descriptions of this achievement.
	
	Any strings in this map will take precedence over the description member if
	the map exists.
	*/
	pub descriptions: Option<HashMap<Platform, String>>,
	
	/// The platform-specific IDs of this achievement.
	pub id: HashMap<Platform, String>,
	
	/// The human-readable name of this achievement.
	pub name: String,
	
	/**
	Platform-specific human-readable names for this achievement.
	
	Any strings in this map will take precedence over the name member if the map
	exists.
	*/
	pub names: Option<HashMap<Platform, String>>,
	
	/**
	The platforms on which this achievement is available
	
	Some achievements are present on certain platforms while being absent on others.
	*/
	pub platforms: Vec<Platform>,
}

impl Achievement
{
	/**
	Is this achievement unlocked on at least one platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.details.iter().any(|pi| pi.isUnlocked());
	}
}

/**
Platform-specific information about an unlocked achievement.
*/
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlatformInfo
{
	/// The percentage of users on this platform who have unlocked this achievement.
	pub globalPercentage: Option<f64>,
	
	/// The path to the platform's associated icon for this achievement.
	pub icon: Option<String>,
	
	/// The mode under which this achievement was unlocked.
	pub mode: Option<Mode>,
	
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
	Is this achievement unlocked on this platform?
	*/
	pub fn isUnlocked(&self) -> bool
	{
		return self.timestamp.is_some();
	}
}
