use std::path::PathBuf;
use data::constants::{BorderColor, Icon_Locked, Path_Games};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, CornerRadius,
	Direction, Gaps, ImageViewer, IntoElement, Size, StyleExt, TextAlign,
	TextStyleExt, label, rect};
use freya::radio::use_radio;
use macros::{join, jpg, jpgAlt};

use crate::api::GogApi;
use crate::data::user::GogUser;

#[derive(Clone, PartialEq)]
pub struct AchievementElement
{
	achievementId: String,
	gameId: u64,
}

impl Component for AchievementElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		
		let achievement = user.read()
			.getAchievement(self.gameId, &self.achievementId)
			.unwrap_or_default();
		
		let unlocked = achievement.dateUnlocked.is_some();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: match unlocked
			{
				false => jpgAlt!(achievement.id, Icon_Locked),
				true => jpg!(achievement.id),
			},
			group: join!(Path_Games, self.gameId.to_string()),
			platform: GogApi::Platform.to_lowercase(),
		});
		
		let timestamp = achievement.formatEarnedTimestamp()
			.unwrap_or_default();
		
		let showIcon = filePathExists(&iconPath);
		
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
									
									.child(label().text(achievement.name))
									
									.child(
										label()
											.font_size(10.0)
											.max_height(Size::px(48.0))
											.text(achievement.description)
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
	pub fn new(gameId: impl Into<u64>, achievementId: impl Into<String>) -> Self
	{
		return Self
		{
			achievementId: achievementId.into(),
			gameId: gameId.into(),
		};
	}
}
