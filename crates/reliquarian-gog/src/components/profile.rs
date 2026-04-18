use std::path::PathBuf;
use components::button::icon::IconButton;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{BorderColor, CornerRadius, OverlayBackgroundColor,
	OverlayGreyoutColor, Path_Avatars};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth, Button,
	ButtonLayoutThemePartialExt, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, CornerRadius, Direction,
	FontWeight, Gaps, ImageViewer, Input, IntoElement, Layer, LayerExt,
	Position, Size, StyleExt, TextAlign, TextStyleExt, WritableUtils, label,
	rect, spawn, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use freya::winit::dpi::PhysicalSize;
use macros::jpg;
use net::{RateLimiter, RequestEvent};
use crate::api::GogApi;
use crate::data::operation::GogOperation;
use crate::data::user::GogUser;
use crate::secure::getGogSession;
use super::refresh::{exchangeCode, openBrowserForAuthorization};

#[derive(Clone, PartialEq)]
pub struct GogUserProfile;

impl Component for GogUserProfile
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let mut authCodeUrl = use_state(String::default);
		let mut cancelled = use_state(bool::default);
		let mut confirmed = use_state(bool::default);
		let mut processAuthCode = use_state(bool::default);
		let mut showAuthOverlay = use_state(bool::default);
		let mut showConfirmationDialog = use_state(bool::default);
		
		let avatarPath = getImagePath(&FileLocation
		{
			fileName: jpg!(user.read().id),
			group: Path_Avatars.into(),
			platform: GogApi::Platform.into(),
		});
		
		let username = user.read().name.clone();
		
		let overlayWidth = windowSize.read().width / 2;
		let overlayHeight = windowSize.read().height / 2;
		
		let left = (windowSize.read().width / 2) - (overlayWidth / 2);
		let top = (windowSize.read().height / 2) - (overlayHeight / 2);
		
		use_side_effect(move || {
			if (cancelled() || confirmed()) && showConfirmationDialog()
			{
				if confirmed()
				{
					spawn(async move {
						rateLimiter.read().pushAll(vec![
							GogOperation::RefreshSession.into(),
							GogOperation::GetUserInfo.into(),
							GogOperation::GetFilteredProducts(None).into(),
						]).await;
						
						**requestEvent.write() = RequestEvent::Added;
					});
				}
				
				cancelled.set(false);
				confirmed.set(false);
				showConfirmationDialog.set(false);
			}
			
			if processAuthCode() && showAuthOverlay()
			{
				exchangeCode(authCodeUrl.read().clone());
				showAuthOverlay.set(false);
			}
		});
		
		let validSession = getGogSession()
			.is_ok_and(|s| !s.hasExpired());
		
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
			.width(Size::Fill)
			
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
								if validSession
								{
									showConfirmationDialog.set(true);
								}
								else
								{
									openBrowserForAuthorization();
									showAuthOverlay.set(true);
								}
							})
					)
			)
			
			.maybe_child(showConfirmationDialog().then(||
				ConfirmRefresh::new(
					cancelled.into_writable(),
					confirmed.into_writable()
				)
			))
			
			.maybe_child(showAuthOverlay().then(||
				rect()
					.position(Position::new_global()
						.left(0.0)
						.top(0.0)
					)
					.background(OverlayGreyoutColor)  
					.cross_align(Alignment::Center)
					.direction(Direction::Vertical)
					.height(Size::px(windowSize.read().height as f32))
					.layer(Layer::Overlay)
					.main_align(Alignment::SpaceEvenly)
					.width(Size::px(windowSize.read().width as f32))
					
					.child(
						rect()
							.position(Position::new_global()
								.left(left as f32)
								.top(top as f32)
							)
							.background(OverlayBackgroundColor)
							.border(Border::new()
								.alignment(BorderAlignment::Center)
								.fill(BorderColor)
								.width(1.0)
							)
							.corner_radius(CornerRadius::new_all(10.0))
							.center()
							.direction(Direction::Vertical)
							.height(Size::px(overlayHeight as f32))
							.padding(Gaps::new_symmetric(5.0, 20.0))
							.min_height(Size::px(400.0))
							.min_width(Size::px(400.0))
							.spacing(15.0)
							.width(Size::px(overlayWidth as f32))
							
							.child(
								label()
									.font_size(24.0)
									.font_weight(FontWeight::BOLD)
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									.text("GOG Authorization Flow")
							)
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.main_align(Alignment::SpaceAround)
									.width(Size::percent(100.0))
									.spacing(5.0)
							
									.child(
										label()
											.width(Size::percent(100.0))
											.text("1) Login to GOG in the browser tab that has been opened.")
									)
									
									.child(
										label()
											.width(Size::percent(100.0))
											.text("2) Once you have logged in, the browser will be redirected to a blank page.")
									)
									
									.child(
										label()
											.width(Size::percent(100.0))
											.text("3) Copy the entire URL from your browser and paste it into the input box below.")
									)
							)
							
							.child(
								label()
									.font_size(12.0)
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									.text("Your authorization will be persisted so next time you shouldn't need to log in manually like this.")
							)
							
							.child(
								Input::new(authCodeUrl)
									.placeholder("Paste URL Here")
									.width(Size::percent(100.0))
							)
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Horizontal)
									.main_align(Alignment::SpaceEvenly)
									.spacing(15.0)
									.width(Size::percent(100.0))
									
									.child(
										Button::new()
											.on_press(move |_| processAuthCode.set(true))
											.width(Size::flex(0.5))
											.child("Ok")
									)
									
									.child(
										Button::new()
											.on_press(move |_| {
												showAuthOverlay.set(false);
												authCodeUrl.set(Default::default());
											})
											.width(Size::flex(0.5))
											.child("Cancel")
									)
							)
				)));
	}
}

impl GogUserProfile
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
