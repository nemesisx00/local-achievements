use freya::icons::lucide;
use freya::prelude::{Alignment, Button, ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, Direction, Gaps, IntoElement, Size, TextAlign, TextStyleExt, label, rect, spawn, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use crate::battlenet::components::refresh::openBrowserForAuthorization;
use crate::components::IconButton;
use crate::components::refresh::auth::OAuth2Overlay;
use crate::data::{AppData, GamePlatforms};
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::data::secure::getBattleNetSession;
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{BattleNetOperation, RequestEvent};

pub fn BattleNetUserProfile() -> impl IntoElement
{
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
	let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
	let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut cancelled = use_state(bool::default);
	let mut showAuthorizationOverlay = use_state(bool::default);
	let mut browserOpened = use_state(bool::default);
	let mut sessionValid = use_state(bool::default);
	
	/*
	let avatar = match BattleNetUserData().ulid
	{
		None => vec![],
		Some(ulid) => loadIcon(
			&Api::Platform.into(),
			&Path_Avatars.into(),
			&png!(ulid)
		),
	};
	*/
	
	use_side_effect(move || {
		if showAuthorizationOverlay()
		{
			if cancelled()
			{
				browserOpened.set(true);
				cancelled.set(false);
				showAuthorizationOverlay.set(false);
			}
			
			if sessionValid()
			{
				cancelled.set(true);
			}
			else if !browserOpened()
			{
				openBrowserForAuthorization(appData.read().platform.battleNet.clone());
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
		
		//.maybe_child()
		
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
									BattleNetOperation::GetSc2PlayerAccount.into(),
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
			OAuth2Overlay::new(cancelled.into_writable())
				.platformName(GamePlatforms::BattleNet.as_ref())
		));
}
