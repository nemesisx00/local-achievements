use std::cmp::Ordering;
use crate::net::limiter::request::{BattleNetOperation, GogOperation, RetroAchievementsOperation, SteamOperation};
use super::location::FileLocation;
use super::operation::DataOperation;

#[derive(Clone, Debug, Eq, PartialEq, Ord)]
pub struct DataRequest
{
	pub destination: Option<FileLocation>,
	pub operation: DataOperation,
	pub url: Option<String>,
}

impl Default for DataRequest
{
	fn default() -> Self
	{
		return Self
		{
			destination: Default::default(),
			operation: Default::default(),
			url: Default::default(),
		};
	}
}

impl From<BattleNetOperation> for DataRequest
{
	fn from(value: BattleNetOperation) -> Self
	{
		return Self
		{
			operation: DataOperation::BattleNet(value),
			..Default::default()
		};
	}
}

impl From<GogOperation> for DataRequest
{
	fn from(value: GogOperation) -> Self
	{
		return Self
		{
			operation: DataOperation::Gog(value),
			..Default::default()
		};
	}
}

impl From<RetroAchievementsOperation> for DataRequest
{
	fn from(value: RetroAchievementsOperation) -> Self
	{
		return Self
		{
			operation: DataOperation::RetroAchievements(value),
			..Default::default()
		};
	}
}

impl From<SteamOperation> for DataRequest
{
	fn from(value: SteamOperation) -> Self
	{
		return Self
		{
			operation: DataOperation::Steam(value),
			..Default::default()
		};
	}
}

impl PartialOrd for DataRequest
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		return self.operation.partial_cmp(&other.operation);
	}
}
