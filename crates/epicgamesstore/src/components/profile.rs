use std::path::PathBuf;
use components::button::icon::IconButton;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{BorderColor, CornerRadius, Path_Avatars};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Gaps, ImageViewer, IntoElement, Size, StyleExt, TextAlign,
	TextStyleExt, WritableUtils, label, rect, spawn, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::png;
use net::{RateLimiter, RequestEvent};
use crate::api::EgsApi;
use crate::data::operation::EgsOperation;
use crate::data::user::EgsUser;

pub fn EgsUserProfile() -> impl IntoElement
{
	let user = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut cancelled = use_state(bool::default);
	let mut confirmed = use_state(bool::default);
	let mut showConfirmationDialog = use_state(bool::default);
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: png!(user.read().accountId),
		group: Path_Avatars.into(),
		platform: EgsApi::Platform.to_lowercase(),
	});
	
	let username = user.read().name.clone();
	
	use_side_effect(move || {
		if (cancelled() || confirmed()) && showConfirmationDialog()
		{
			if confirmed()
			{
				spawn(async move {
					rateLimiter.read().pushAll(vec![
						EgsOperation::GetPlayerProfile.into(),
						EgsOperation::GetPlayerProfilePrivate.into(),
						EgsOperation::SaveToFile.into(),
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
		.border(Some(
			Border::new()
				.alignment(BorderAlignment::Center)
				.fill(BorderColor)
				.width(BorderWidth::from(1.0))
		))
		.corner_radius(CornerRadius)
		.direction(Direction::Horizontal)
		.main_align(Alignment::Start)
		.margin(Gaps::new_symmetric(0.0, 1.0))
		.padding(Gaps::new_symmetric(10.0, 10.0))
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
