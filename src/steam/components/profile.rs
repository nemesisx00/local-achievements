use std::path::PathBuf;
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Gaps, ImageViewer, IntoElement, Size,
	TextAlign, TextStyleExt, label, rect, spawn};
use freya::radio::use_radio;
use crate::components::IconButton;
use crate::data::AppData;
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::io::{Path_Avatars, getImagePath};
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{FileLocation, RequestEvent, SteamOperation};
use crate::steam::platform::api::SteamApi;

pub fn SteamProfile() -> impl IntoElement
{
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Steam);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let userId = appData.read().user.steam.id.clone();
	let username = appData.read().user.steam.name.clone();
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: format!("{}_full.jpg", userId),
		group: Path_Avatars.into(),
		platform: SteamApi::Platform.into(),
	});
	
	return rect()
		.direction(Direction::Horizontal)
		.main_align(Alignment::Start)
		.spacing(10.0)
		.width(Size::flex(1.0))
		
		.maybe_child(avatarPath.is_some().then(||
			ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
				.width(Size::px(64.0))
		))
		
		.child(
			rect()
				.direction(Direction::Vertical)
				.main_align(Alignment::SpaceAround)
				
				.child(
					label()
						.margin(Gaps::new(0.0, 0.0, 0.0, 7.0))
						.text_align(TextAlign::Center)
						.text(username)
				)
				
				.child(
					IconButton::new(lucide::refresh_ccw())
						.alt("Refresh")
						.height(Size::px(32.0))
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
