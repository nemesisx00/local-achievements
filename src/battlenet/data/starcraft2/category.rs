use serde::{Deserialize, Serialize};
use crate::battlenet::platform::data::starcraft2::profile::metadata::CategoryMetadata;
use super::achievement::Sc2Achievement;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sc2Category
{
	pub achievements: Vec<Sc2Achievement>,
	pub children: Vec<Sc2Category>,
	pub displayOrder: u64,
	pub id: u64,
	pub name: String,
	pub points: u64,
}

impl From<CategoryMetadata> for Sc2Category
{
	fn from(value: CategoryMetadata) -> Self
	{
		return Self
		{
			id: value.id.parse::<u64>()
				.unwrap_or_default(),
			
			displayOrder: value.uiOrderHint,
			name: value.name.clone(),
			points: value.points,
			..Default::default()
		};
	}
}

impl Sc2Category
{
	pub fn addAchievement(&mut self, categoryId: &String, achievement: Sc2Achievement)
	{
		if &self.id.to_string() == categoryId
		{
			self.achievements.push(achievement);
		}
		else if let Some(parent) = self.children.iter_mut()
			.find(|c| c.containsId(&categoryId.to_string()))
		{
			parent.addAchievement(categoryId, achievement);
		}
	}
	
	/**
	Add a child category, either to this instance or one of its children.
	
	The child is only added if the specified `parentId` exists as an `id`
	under this `Category`.
	*/
	pub fn addChild(&mut self, parentId: &String, child: Sc2Category)
	{
		if &self.id.to_string() == parentId
		{
			self.children.push(child);
		}
		else if let Some(parent) = self.children.iter_mut()
			.find(|c| c.containsId(parentId))
		{
			parent.addChild(parentId, child);
		}
	}
	
	/**
	Calculate the total number of unlocked achievements within this category
	and its children.
	*/
	pub fn completedAchievements(&self) -> u64
	{
		let completed = self.achievements.iter()
			.fold(
				0,
				|acc, ach| acc + match ach.unlocked
				{
					false => 0,
					true => 1
				}
			);
		
		return self.children.iter()
			.fold(
				completed,
				|acc, cat| acc + cat.completedAchievements()
			);
	}
	
	/**
	Check if the given `id` exists, within this instance or one of its
	children.
	*/
	pub fn containsId(&self, id: &String) -> bool
	{
		return &self.id.to_string() == id
			|| self.children.iter().any(|c| c.containsId(id));
	}
	
	/**
	Process the categories metadata returned by the Profile/Static endpoint.
	
	Since categories can nest within one another and the metadata is returned
	by the API in a flattened state, the most reliable means of processing the
	metadata is iterating until all categories with parent IDs have been
	processed.
	
	If a state is reached wherein all of the leftover categories reference
	parent IDs which do not exist in the processed `Sc2Category` list, then
	those leftover categories are assumed to be invalid and the method returns.
	*/
	pub fn processAllMetadata(metadata: Vec<CategoryMetadata>) -> Vec<Self>
	{
		let (mut categories, mut leftovers) = Self::processMetadata(metadata, vec![]);
		while leftovers.len() > 0
		{
			(categories, leftovers) = Self::processMetadata(leftovers, categories);
			
			// For now, prune any leftovers that don't have an existing parent category
			if leftovers.iter().all(|m| categories.iter()
				.find(|c|c.containsId(&m.parentCategoryId.clone().unwrap_or_default()))
				.is_none()
			)
			{
				break;
			}
		}
		
		return categories;
	}
	
	/**
	A single metadata processing iteration.
	
	Any categories which reference parent IDs but cannot be added to an
	existing category are returned as leftovers.
	*/
	fn processMetadata(metadata: Vec<CategoryMetadata>, mut categories: Vec<Sc2Category>) -> (Vec<Self>, Vec<CategoryMetadata>)
	{
		let mut leftovers = vec![];
		
		for metaCategory in metadata
		{
			if metaCategory.parentCategoryId.is_none()
			{
				categories.push(Self::from(metaCategory));
			}
			else if let Some(parent) = categories.iter_mut()
				.find(|c| c.containsId(&metaCategory.parentCategoryId.clone().unwrap_or_default()))
			{
				let category = Self::from(metaCategory.clone());
				parent.addChild(&metaCategory.parentCategoryId.clone().unwrap_or_default(), category);
			}
			else
			{
				leftovers.push(metaCategory);
			}
		}
		
		return (categories, leftovers);
	}
	
	pub fn totalAchievements(&self) -> usize
	{
		return self.children.iter()
			.fold(
				self.achievements.len(),
				|acc, cat| acc + cat.totalAchievements()
			);
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	const JsonPayload: &str = r#"
[
	{
		"childrenCategoryIds":[
			"5",
			"9"
		],
		"featuredAchievementId":"769",
		"id":"12",
		"name":"First Top Level Parent",
		"parentCategoryId":null,
		"points":1530,
		"uiOrderHint":1,
		"medalTiers":[0,250,500,1000]
	},
	{
		"childrenCategoryIds":[],
		"featuredAchievementId":"0",
		"id":"9",
		"name":"Category with id 9",
		"parentCategoryId":"12",
		"points":100,
		"uiOrderHint":3
	},
	{
		"childrenCategoryIds":[
			"47"
		],
		"featuredAchievementId":"513",
		"id":"2",
		"name":"Category with id 2",
		"parentCategoryId":null,
		"points":1590,
		"uiOrderHint":0,
		"medalTiers":[0,250,500,1000]
	},
	{
		"childrenCategoryIds":[],
		"featuredAchievementId":"0",
		"id":"47",
		"name":"Category with id 47",
		"parentCategoryId":"2",
		"points":105,
		"uiOrderHint":0
	},
	{
		"childrenCategoryIds":[],
		"featuredAchievementId":"0",
		"id":"5",
		"name":"Category with id 5",
		"parentCategoryId":"12",
		"points":340,
		"uiOrderHint":0
	},	
	{
		"childrenCategoryIds":[],
		"featuredAchievementId":"0",
		"id":"23",
		"name":"Category with id 23",
		"parentCategoryId":"5",
		"points":630,
		"uiOrderHint":0
	},
	{
		"childrenCategoryIds":[],
		"featuredAchievementId":"0",
		"id":"92",
		"name":"This category should be pruned",
		"parentCategoryId":"54",
		"points":360,
		"uiOrderHint":0
	}
]
"#;
	
	#[test]
	fn processMetadata()
	{
		let metadata = serde_json::from_str(JsonPayload);
		
		assert!(metadata.is_ok());
		let metadata = metadata.unwrap();
		
		let categories = Sc2Category::processAllMetadata(metadata);
		assert!(!categories.is_empty());
		assert_eq!(categories.len(), 2);
		
		// Categories with a specified parent id and whose parent does not exist are pruned
		assert!(categories.iter().all(|c| !c.containsId(&"92".into())));
		
		let parent = categories.iter().find(|c| c.id == 12).unwrap();
		assert_eq!(&parent.name, "First Top Level Parent");
		assert_eq!(parent.displayOrder, 1);
		assert_eq!(parent.points, 1530);
		assert_eq!(parent.children.len(), 2);
		
		let child = parent.children.iter().find(|c| !c.children.is_empty()).unwrap();
		assert_eq!(child.id, 5);
		assert_eq!(&child.name, "Category with id 5");
		assert_eq!(child.displayOrder, 0);
		assert_eq!(child.points, 340);
		assert_eq!(child.children.len(), 1);
		
		let secondChild = child.children.first().unwrap();
		assert_eq!(secondChild.id, 23);
		assert_eq!(&secondChild.name, "Category with id 23");
		assert_eq!(secondChild.displayOrder, 0);
		assert_eq!(secondChild.points, 630);
	}
}
