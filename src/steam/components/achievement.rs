use std::path::PathBuf;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, CircularLoader, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, CornerRadius, Direction, Gaps,
	ImageViewer, IntoElement, Size, StyleExt, TextAlign, TextStyleExt, label,
	rect};
use freya::radio::use_radio;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::net::limiter::request::FileLocation;
use crate::util::filePathExists;
use crate::{join, jpg, jpgAlt};
use crate::constants::{BorderColor, Icon_Locked};
use crate::io::{Path_Games, getImagePath};
use crate::steam::platform::api::SteamApi;

#[derive(Clone, Default, PartialEq)]
pub struct AchievementElement
{
	gameId: u64,
	id: String,
}

impl Component for AchievementElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Steam);
		
		let achievement = appData.read().user.steam
			.getAchievement(self.gameId, self.id.clone())
			.unwrap_or_default();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: match achievement.unlocked()
			{
				false => jpgAlt!(achievement.id, Icon_Locked),
				true => jpg!(achievement.id),
			},
			group: join!(Path_Games, self.gameId),
			platform: SteamApi::Platform.to_lowercase(),
		});
		
		let timestamp = achievement.formatTimestamp()
			.unwrap_or_default();
		
		let globalPercentage = match achievement.globalPercentage
		{
			None => Default::default(),
			Some(gp) => format!("{}% of players have this achievement", gp),
		};
		
		let showIcon = filePathExists(&iconPath);
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			//.margin(Gaps::new_symmetric(5.0, 0.0))
			.width(Size::Fill)
			
			.child(
				rect()
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(BorderWidth::from(1.0))
					))
					.corner_radius(CornerRadius::new_all(10.0))
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new_symmetric(5.0, 0.0))
					.min_height(Size::px(64.0))
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 10.0))
					.width(Size::percent(50.0))
					
					.maybe_child((!showIcon).then(||
						CircularLoader::new()
					))
					
					.maybe_child(showIcon.then(||
						rect()
							.cross_align(Alignment::Center)
							.main_align(Alignment::Center)
							
							.child(
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.height(Size::px(64.0))
									.width(Size::px(64.0))
							)
					))
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Vertical)
							//.min_height(Size::px(40.0))
							//.padding(Gaps::new(0.0, 20.0, 0.0, 0.0))
							.spacing(10.0)
							.width(Size::percent(80.0))
							
							.child(
								rect()
									.cross_align(Alignment::Center)
									.direction(Direction::Horizontal)
									.main_align(Alignment::SpaceBetween)
									.width(Size::percent(100.0))
									
									.child(
										label()
											.text(achievement.name)
											.width(Size:: percent(100.0))
									)
							)
							
							.child(
								rect()
									.direction(Direction::Horizontal)
									.main_align(Alignment::Start)
									.width(Size::percent(100.0))
									
									.child(
										label()
											.font_size(10.0)
											.text(achievement.description)
									)
							)
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::SpaceBetween)
									.width(Size::percent(100.0))
									
									.child(
										label()
											.font_size(10.0)
											.text_align(TextAlign::Start)
											.text(timestamp)
											.width(Size::flex(0.5))
									)
									
									.child(
										label()
											.font_size(10.0)
											.text_align(TextAlign::End)
											.text(globalPercentage)
											.width(Size::flex(0.5))
									)
							)
					)
			);
	}
}

impl AchievementElement
{
	pub fn new(gameId: impl Into<u64>, id: impl Into<String>) -> Self
	{
		return Self
		{
			gameId: gameId.into(),
			id: id.into(),
		};
	}
}
