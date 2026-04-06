use std::path::PathBuf;
use freya::icons::lucide;
use freya::prelude::{Alignment, Button, ChildrenExt, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Direction, Gaps, ImageViewer,
	IntoElement, Size, TextAlign, TextStyleExt, label, rect, spawn,
	use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use crate::battlenet::components::refresh::openBrowserForAuthorization;
use crate::battlenet::platform::api::BattleNetApi;
use crate::battlenet::platform::starcraft2::Starcraft2;
use crate::components::IconButton;
use crate::components::refresh::auth::OAuth2Overlay;
use crate::data::{AppData, GamePlatforms};
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::data::secure::getBattleNetSession;
use crate::io::{Path_Avatars, getImagePath};
use crate::jpgAlt;
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{BattleNetOperation, FileLocation,
	RequestEvent};
use crate::util::filePathExists;

pub fn BattleNetUserProfile() -> impl IntoElement
{
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut cancelled = use_state(bool::default);
	let mut authDone = use_state(bool::default);
	let mut showAuthorizationOverlay = use_state(bool::default);
	let mut browserOpened = use_state(bool::default);
	let mut sessionValid = use_state(bool::default);
	
	let avatarPath = getImagePath(&FileLocation
	{
		fileName: jpgAlt!(
			Starcraft2::GamePrefix,
			appData.read().user.battleNet.starcraft2
				.clone()
				.unwrap_or_default()
				.id
		),
		group: Path_Avatars.into(),
		platform: BattleNetApi::Platform.to_lowercase(),
	});
	
	use_side_effect(move || {
		if showAuthorizationOverlay()
		{
			if cancelled()
			{
				authDone.set(false);
				browserOpened.set(true);
				cancelled.set(false);
				showAuthorizationOverlay.set(false);
			}
			
			if sessionValid()
			{
				authDone.set(true);
				cancelled.set(true);
			}
			else if !browserOpened()
			{
				let settings = appData.read().platform.battleNet.clone();
				let region = match appData.read().user.battleNet.starcraft2.clone()
				{
					None => settings.defaultRegion,
					Some(profile) => profile.region,
				};
				
				openBrowserForAuthorization(settings, region);
				browserOpened.set(true);
			}
		}
	});
	
	sessionValid.set(
		getBattleNetSession()
			.is_ok_and(|s| !s.hasExpired())
	);
	
	return rect()
		.direction(Direction::Horizontal)
		.main_align(Alignment::Start)
		.spacing(10.0)
		.width(Size::flex(1.0))
		
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
						.text(appData.read().user.battleNet.battleTag.clone())
				)
				
				.maybe_child(sessionValid().then(||
					IconButton::new(lucide::refresh_ccw())
						.alt("Refresh")
						.height(Size::px(32.0))
						.width(Size::px(32.0))
						.onPress(move |_| {
							spawn(async move {
								rateLimiter.read().pushAll(vec![
									BattleNetOperation::GetUserInfo.into(),
									BattleNetOperation::SaveToFile.into(),
								]).await;
								
								**requestEvent.write() = RequestEvent::Added;
							});
						})
				))
				
				.maybe_child((!sessionValid()).then(||
					Button::new()
						.on_press(move |_| {
							browserOpened.set(false);
							showAuthorizationOverlay.set(true);
						})
						.child("Log In")
				))
		)
		
		.maybe_child(showAuthorizationOverlay().then(||
			OAuth2Overlay::new(
				cancelled.into_writable(),
				authDone.into_writable()
			)
				.platformName(GamePlatforms::BattleNet.as_ref())
		));
}
