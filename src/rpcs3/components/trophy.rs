use std::path::PathBuf;
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt,
	CircularLoader, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, CornerRadius, Direction, Gaps,
	ImageViewer, IntoElement, Size, StyleExt, TextAlign, TextStyleExt, label,
	rect};
use freya::radio::use_radio;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::join;
use crate::constants::BorderColor;
use crate::io::{Path_Games, getImagePath};
use crate::net::limiter::request::FileLocation;
use crate::rpcs3::platform::api::Rpcs3Api;

#[derive(Clone, PartialEq)]
pub struct TrophyElement
{
	npCommId: String,
	trophyId: u64,
}

impl Component for TrophyElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Rpcs3);
		
		let trophy = match appData.read().user.rpcs3.games.iter()
			.find(|g| g.npCommId == self.npCommId)
		{
			None => Default::default(),
			Some(g) => match g.trophies.iter()
				.find(|t| t.id == self.trophyId)
			{
				None => Default::default(),
				Some(t) => t.clone(),
			}
		};
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: format!(
				"{}{:03}.PNG",
				Rpcs3Api::TrophyIconPrefix,
				trophy.id
			),
			group: join!(Path_Games, self.npCommId),
			platform: Rpcs3Api::Platform.into(),
		});
		
		let timestamp = match trophy.formatUnlockedTimestamp()
		{
			Err(_) => Default::default(),
			Ok(ts) => ts,
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
							.width(Size::percent(100.0))
							
							.maybe_child(iconPath.is_none().then(||
								CircularLoader::new()
							))
							
							.maybe_child(iconPath.is_some().then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.width(Size::px(64.0))
							))
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.main_align(Alignment::SpaceBetween)
									.spacing(15.0)
									.width(Size::percent(89.0))
									
									.child(
										rect()
											.content(Content::Flex)
											.direction(Direction::horizontal())
											.main_align(Alignment::SpaceBetween)
											.width(Size::percent(98.0))
											
											.child(
												label()
													.text(trophy.name.clone())
													.width(Size::flex(0.7))
											)
											
											.child(
												label()
													.font_size(10.0)
													.text_align(TextAlign::End)
													.text(timestamp)
													.width(Size::flex(0.3))
											)
									)
									
									.child(
										label()
											.font_size(10.0)
											.text(trophy.detail.clone())
									)
							)
					)
			);
	}
}

impl TrophyElement
{
	pub fn new(npCommId: String, trophyId: u64) -> Self
	{
		return Self
		{
			npCommId,
			trophyId,
		};
	}
}
