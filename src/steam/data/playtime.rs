use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::steam::platform::GameInfo;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Playtime
{
	#[serde(default)]
	pub linux: u64,
	
	#[serde(default)]
	pub mac: u64,
	
	#[serde(default)]
	pub offline: u64,
	
	#[serde(default)]
	pub total: u64,
	
	#[serde(default)]
	pub windows: u64,
}

impl Playtime
{
	pub fn parseJsonMap(map: &Map<String, Value>) -> Self
	{
		let mut playtime = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "linux")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					playtime.linux = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "mac")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					playtime.mac = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "offline")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					playtime.offline = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "total")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					playtime.total = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "windows")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					playtime.windows = number;
				}
			}
		}
		
		return playtime;
	}
	
	pub fn update(&mut self, info: &GameInfo)
	{
		self.linux = info.playtime_linux_forever;
		self.mac = info.playtime_mac_forever;
		self.offline = info.playtime_disconnected;
		self.total = info.playtime_forever;
		self.windows = info.playtime_windows_forever;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	fn buildMap() -> Map<String, Value>
	{
		let mut map = Map::new();
		
		map.insert("linux".into(), 20000.into());
		map.insert("mac".into(), 1.into());
		map.insert("offline".into(), 1000.into());
		map.insert("total".into(), 21101.into());
		map.insert("windows".into(), 100.into());
		
		return map;
	}
	
	#[test]
	fn parseJsonMap()
	{
		let map = buildMap();
		let playtime = Playtime::parseJsonMap(&map);
		assert_eq!(playtime.linux, 20000);
		assert_eq!(playtime.mac, 1);
		assert_eq!(playtime.offline, 1000);
		assert_eq!(playtime.total, 21101);
		assert_eq!(playtime.windows, 100);
	}
}
