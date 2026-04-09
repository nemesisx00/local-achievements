use std::path::PathBuf;
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Gaps, ImageViewer, IntoElement, Size,
	TextAlign, TextStyleExt, label, rect, spawn, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use crate::egs::EgsApi;
use crate::png;
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::components::IconButton;
use crate::components::refresh::confirm::ConfirmRefresh;
use crate::data::AppData;
use crate::io::{Path_Avatars, getImagePath};
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{EpicGamesStoreOperation, FileLocation,
	RequestEvent};
use crate::util::filePathExists;

pub fn EgsUserProfile() -> impl IntoElement
{
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::EpicGamesStore);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut cancelled = use_state(bool::default);
	let mut confirmed = use_state(bool::default);
	let mut showConfirmationDialog = use_state(bool::default);
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: png!(appData.read().user.egs.accountId),
		group: Path_Avatars.into(),
		platform: EgsApi::Platform.to_lowercase(),
	});
	
	let username = appData.read().user.egs.name.clone();
	
	use_side_effect(move || {
		if (cancelled() || confirmed()) && showConfirmationDialog()
		{
			if confirmed()
			{
				spawn(async move {
					rateLimiter.read().pushAll(vec![
						EpicGamesStoreOperation::GetPlayerProfile.into(),
						EpicGamesStoreOperation::GetPlayerProfilePrivate.into(),
						EpicGamesStoreOperation::SaveToFile.into(),
					]).await;
					
					**requestEvent.write() = RequestEvent::Added;
				});
			}
			
			cancelled.set(false);
			confirmed.set(false);
			showConfirmationDialog.set(false);
		}
	});
	
	return rect()
		.direction(Direction::Horizontal)
		.main_align(Alignment::Start)
		.spacing(10.0)
		.width(Size::Fill)
		
		.maybe_child(filePathExists(&avatarPath).then(||
			ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
				.width(Size::px(64.0))
		))
		
		.child(
			rect()
				.direction(Direction::Vertical)
				.main_align(Alignment::SpaceAround)
				
				.child(
					label()
						.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
						.text_align(TextAlign::Start)
						.text(username)
				)
				
				.child(
					IconButton::new(lucide::refresh_ccw())
						.alt("Refresh")
						.height(Size::px(32.0))
						.width(Size::px(32.0))
						.onPress(move |_| showConfirmationDialog.set(true))
				)
		)
		
		.maybe_child(showConfirmationDialog().then(||
			ConfirmRefresh::new(
				cancelled.into_writable(),
				confirmed.into_writable()
			)
		))
}
