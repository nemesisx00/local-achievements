use std::path::PathBuf;
use data::constants::{BorderColor, CornerRadius, Format_ChronoDateTime, Path_Games};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction,
	Gaps, ImageViewer, IntoElement, Size, StyleExt, TextAlign, TextStyleExt,
	label, rect};
use freya::radio::use_radio;
use macros::{join, jpgAlt};
use crate::api::{BattleNetApi, Starcraft2};
use crate::data::user::BattleNetUser;

pub fn sc2Achievement(id: u64) -> Sc2Achievement
{
	return Sc2Achievement
	{
		id,
	};
}

#[derive(Clone, PartialEq)]
pub struct Sc2Achievement
{
	id: u64,
}

impl Component for Sc2Achievement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
		
		let achievement = user.read().starcraft2
			.clone()
			.unwrap_or_default()
			.getAchievement(self.id)
			.unwrap_or_default();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpgAlt!(BattleNetApi::AchievementPrefix, achievement.id),
			group: join!(Path_Games, Starcraft2::GamePrefix),
			platform: BattleNetApi::Platform.to_lowercase(),
		});
		
		let unlockedTimestamp = match achievement.unlockedTimestamp
		{
			None => None,
			Some(ts) => Some(ts.format(Format_ChronoDateTime).to_string()),
		};
		
		return rect()
			.border(Some(Border::new()
				.alignment(BorderAlignment::Center)
				.fill(BorderColor)
				.width(1.0)
			))
			.content(Content::Flex)
			.corner_radius(CornerRadius)
			.cross_align(Alignment::Center)
			.direction(Direction::Horizontal)
			.margin(Gaps::new(5.0, 15.0, 5.0, 0.0))
			.padding(Gaps::new_all(5.0))
			.spacing(2.5)
			.width(Size::percent(100.0))
			
			.maybe_child(filePathExists(&iconPath).then(||
				ImageViewer::new(PathBuf::from(iconPath.unwrap()))
					.height(Size::px(64.0))
					.width(Size::px(64.0))
			))
			
			.child(
				rect()
					.direction(Direction::Vertical)
					.main_align(Alignment::SpaceBetween)
					.spacing(5.0)
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Horizontal)
							
							.child(
								label()
									.width(Size::flex(0.8))
									.text(achievement.name)
							)
							
							.child(
								label()
									.text(achievement.points.to_string())
									.text_align(TextAlign::End)
									.width(Size::flex(0.2))
							)
					)
					
					.child(
						label()
							.font_size(10.0)
							.text(achievement.description)
					)
					
					.maybe_child(unlockedTimestamp.is_some().then(||
						label()
							.font_size(10.0)
							.text(unlockedTimestamp.unwrap())
					))
			);
	}
}
