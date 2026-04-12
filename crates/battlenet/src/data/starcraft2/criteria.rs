use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Ord, Serialize)]
pub struct Sc2Criteria
{
	pub description: String,
	pub displayOrder: u64,
	pub id: u64,
	pub necessaryQuantity: u64,
}

impl PartialOrd for Sc2Criteria
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		return self.displayOrder.partial_cmp(&other.displayOrder);
	}
}

impl Sc2Criteria
{
	pub fn parseJsonMapLossy(map: &Map<String, Value>) -> Option<Self>
	{
		let mut criteria = Self::default();
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "description")
		{
			if let Value::String(inner) = value
			{
				criteria.description = inner.clone();
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "displayOrder")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					criteria.displayOrder = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "id")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					criteria.id = number;
				}
			}
		}
		
		if let Some((_, value)) = map.iter()
			.find(|(key, _)| key.as_str() == "necessaryQuantity")
		{
			if let Value::Number(inner) = value
			{
				if let Some(number) = inner.as_u64()
				{
					criteria.necessaryQuantity = number;
				}
			}
		}
		
		return match criteria.id > 0
		{
			false => None,
			true => Some(criteria),
		};
	}
}
