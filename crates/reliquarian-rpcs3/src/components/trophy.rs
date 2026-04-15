use std::path::PathBuf;
use data::constants::{BorderColor, CornerRadius, Path_Games};
use data::enums::GamePlatforms;
use data::io::{FileLocation, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Component,
	ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction,
	Gaps, ImageViewer, IntoElement, Size, StyleExt, TextStyleExt, label, rect};
use freya::radio::use_radio;
use macros::join;
use crate::api::api::Rpcs3Api;
use crate::data::user::Rpcs3User;

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
		let user = use_radio::<Rpcs3User, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let trophy = match user.read().games.iter()
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
					
					.maybe_child(iconPath.is_some().then(||
						rect()
							.cross_align(Alignment::Center)
							.main_align(Alignment::Center)
							
							.child(
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.corner_radius(CornerRadius)
									.height(Size::px(64.0))
							)
					))
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Vertical)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.text(trophy.name.clone())
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(trophy.detail.clone())
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(timestamp)
									.width(Size::flex(1.0))
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
