use std::path::PathBuf;
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, CornerRadius,
	Direction, Gaps, ImageViewer, IntoElement, Size, StyleExt, TextAlign,
	TextStyleExt, label, rect};
use freya::radio::use_radio;
use crate::constants::BorderColor;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::egs::EgsApi;
use crate::io::{Path_Games, getImagePath};
use crate::net::limiter::request::FileLocation;
use crate::util::filePathExists;
use crate::join;

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
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::EpicGamesStore);
		
		let achievement = appData.read().user.egs
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
			.margin(Gaps::new_symmetric(5.0, 0.0))
			.width(Size::Fill)
			
			.child(
				rect()
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(1.0)
					))
					.corner_radius(CornerRadius::new_all(10.0))
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new_all(5.0))
					.min_height(Size::px(64.0))
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.min_height(Size::px(64.0))
							.spacing(10.0)
							.width(Size::Fill)
							
							.maybe_child((!showIcon).then(||
								rect()
									.height(Size::px(64.0))
									.width(Size::px(64.0))
							))
							
							.maybe_child(showIcon.then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.width(Size::px(64.0))
							))
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.main_align(Alignment::SpaceBetween)
									.spacing(15.0)
									.width(Size::percent(60.0))
									
									.child(name)
									
									.child(
										label()
											.font_size(10.0)
											.max_height(Size::px(48.0))
											.text(description)
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.min_height(Size::px(50.0))
							.main_align(Alignment::Center)
							.min_width(Size::px(150.0))
							
							.child(
								label()
									.font_size(10.0)
									.text_align(TextAlign::End)
									.width(Size::percent(100.0))
									.text(timestamp)
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
