use std::path::PathBuf;
use components::button::icon::IconButton;
use components::overlay::auth::OAuth2Overlay;
use data::constants::{BorderColor, CornerRadius, Path_Avatars};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, Gaps, ImageViewer, IntoElement,
	Size, StyleExt, WritableUtils, rect, spawn, use_side_effect, use_state};
use freya::radio::use_radio;
use macros::jpgAlt;
use net::{RateLimiter, RequestEvent};
use crate::api::{BattleNetApi, BattleNetSettings, Starcraft2};
use crate::components::refresh::openBrowserForAuthorization;
use crate::data::operation::BattleNetOperation;
use crate::data::user::BattleNetUser;
use crate::secure::getBattleNetSession;

#[derive(Clone, PartialEq)]
pub struct BattleNetUserProfile;

impl Component for BattleNetUserProfile
{
	fn render(&self) -> impl IntoElement
	{
		let settings = use_radio::<BattleNetSettings, GamePlatforms>(GamePlatforms::BattleNet);
		let user = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
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
				user.read().starcraft2
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
					let settings = settings.read().clone();
					let region = match user.read().starcraft2.clone()
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
			.padding(Gaps::new_all(10.0))
			.spacing(10.0)
			.width(Size::flex(1.0))
			
			.maybe_child(filePathExists(&avatarPath).then(||
				ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
					.width(Size::px(64.0))
			))
			
			.child(
				rect()
					.cross_align(Alignment::SpaceBetween)
					.direction(Direction::Vertical)
					.height(Size::px(64.0))
					.main_align(Alignment::Center)
					.spacing(5.0)
					.width(Size::flex(1.0))
					
					.child(user.read().battleTag.clone())
					
					.child(
						IconButton::new(lucide::refresh_ccw())
							.alt("Refresh")
							.height(Size::px(32.0))
							.innerHeight(Size::px(24.0))
							.innerWidth(Size::px(24.0))
							.width(Size::px(32.0))
							.onPress(move |_| {
								if sessionValid()
								{
									spawn(async move {
										rateLimiter.read().pushAll(vec![
											BattleNetOperation::GetUserInfo.into(),
											BattleNetOperation::SaveToFile.into(),
										]).await;
										
										**requestEvent.write() = RequestEvent::Added;
									});
								}
								else
								{
									browserOpened.set(false);
									showAuthorizationOverlay.set(true);
								}
							})
					)
			)
			
			.maybe_child(showAuthorizationOverlay().then(||
				OAuth2Overlay::new(cancelled, authDone)
					.platformName(GamePlatforms::BattleNet.as_ref())
			));
	}
}

impl BattleNetUserProfile
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
