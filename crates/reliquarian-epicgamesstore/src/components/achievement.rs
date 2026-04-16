use std::path::PathBuf;
use data::constants::{BorderColor, CornerRadius, Path_Games};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction,
	Gaps, ImageViewer, IntoElement, Size, StyleExt, TextStyleExt, label, rect};
use freya::radio::use_radio;
use macros::join;
use crate::api::EgsApi;
use crate::data::user::EgsUser;

#[derive(Clone, PartialEq)]
pub struct AchievementElement
{
	achievementId: String,
	sandboxId: String,
}

impl Component for AchievementElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
		
		let achievement = user.read()
			.getAchievement(&self.sandboxId, &self.achievementId)
			.unwrap_or_default();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: match achievement.isUnlocked
			{
				false => achievement.locked.iconId.clone(),
				true => achievement.unlocked.iconId.clone(),
			},
			group: join!(Path_Games, self.sandboxId.clone()),
			platform: EgsApi::Platform.to_lowercase(),
		});
		
		let timestamp = achievement.formatEarnedTimestamp()
			.unwrap_or_default();
		
		let showIcon = filePathExists(&iconPath);
		
		let name = match achievement.isUnlocked
		{
			false => match achievement.locked.name.is_empty()
			{
				false => achievement.locked.name.clone(),
				true => "Hidden Achievement".to_string(),
			},
			true => achievement.unlocked.name.clone(),
		};
		
		let description = match achievement.isUnlocked
		{
			false => achievement.locked.description.clone(),
			true => achievement.unlocked.description.clone(),
		};
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			.width(Size::Fill)
			
			.child(
				rect()
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(1.0)
					))
					.content(Content::Flex)
					.corner_radius(CornerRadius)
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new_symmetric(5.0, 0.0))
					.min_height(Size::px(64.0))
					.min_width(Size::px(540.0))
					.padding(Gaps::new_all(10.0))
					.spacing(15.0)
					.width(Size::percent(50.0))
					
					.maybe_child(showIcon.then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap()))
							.corner_radius(CornerRadius)
							.height(Size::px(64.0))
					))
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Vertical)
							.main_align(Alignment::SpaceBetween)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.text(name)
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(description)
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(timestamp)
									.width(Size::flex(100.0))
							)
					)
			);
	}
}

impl AchievementElement
{
	pub fn new(sandboxId: impl Into<String>, achievementId: impl Into<String>) -> Self
	{
		return Self
		{
			achievementId: achievementId.into(),
			sandboxId: sandboxId.into(),
		};
	}
}
