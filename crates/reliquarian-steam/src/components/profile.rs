use std::path::PathBuf;
use components::button::icon::IconButton;
use data::constants::{BorderColor, CornerRadius, Path_Avatars};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Content, Direction, Gaps, ImageViewer, IntoElement, Size, StyleExt, rect,
	spawn};
use freya::radio::use_radio;
use net::{RateLimiter, RequestEvent};
use crate::api::SteamApi;
use crate::data::operation::SteamOperation;
use crate::data::user::SteamUser;

pub fn SteamProfile() -> impl IntoElement
{
	let user = use_radio::<SteamUser, GamePlatforms>(GamePlatforms::Steam);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let userId = user.read().id.clone();
	let username = user.read().name.clone();
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: format!("{}_full.jpg", userId),
		group: Path_Avatars.into(),
		platform: SteamApi::Platform.into(),
	});
	
	return rect()
		.border(Some(
			Border::new()
				.alignment(BorderAlignment::Center)
				.fill(BorderColor)
				.width(BorderWidth::from(1.0))
		))
		.corner_radius(CornerRadius)
		.content(Content::Flex)
		.direction(Direction::Horizontal)
		.main_align(Alignment::Start)
		.margin(Gaps::new_symmetric(0.0, 1.0))
		.padding(Gaps::new_symmetric(10.0, 10.0))
		.spacing(10.0)
		.width(Size::flex(1.0))
		
		.maybe_child(filePathExists(&avatarPath).then(||
			ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
				.corner_radius(CornerRadius)
				.height(Size::px(64.0))
		))
		
		.child(
			rect()
				.cross_align(Alignment::Center)
				.direction(Direction::Horizontal)
				.height(Size::px(64.0))
				.main_align(Alignment::SpaceBetween)
				.width(Size::flex(1.0))
				
				.child(username)
				
				.child(
					IconButton::new(lucide::refresh_ccw())
						.alt("Refresh")
						.height(Size::px(32.0))
						.innerHeight(Size::px(24.0))
						.innerWidth(Size::px(24.0))
						.width(Size::px(32.0))
						.onPress(move |_| {
							spawn(async move {
								rateLimiter.read().pushAll(vec![
									SteamOperation::GetPlayerSummary.into(),
									SteamOperation::GetGameList.into(),
									SteamOperation::SaveToFile.into(),
								]).await;
								
								**requestEvent.write() = RequestEvent::Added;
							});
						})
				)
		);
}
