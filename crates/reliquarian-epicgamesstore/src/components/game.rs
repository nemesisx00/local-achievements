use std::path::PathBuf;
use components::button::icon::IconButton;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{FileName_GameIcon, Path_Games};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, ImageViewer, Input, IntoElement, KeyboardEventData,
	ScrollConfig, ScrollPosition, Size, TextAlign, TextStyleExt,
	VirtualScrollView, WritableUtils, label, rect, spawn, use_scroll_controller,
	use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, jpg};
use net::{RateLimiter, RequestEvent};
use crate::api::EgsApi;
use crate::data::operation::EgsOperation;
use crate::data::user::EgsUser;
use super::achievement::AchievementElement;

#[derive(Clone, PartialEq)]
pub struct GameElement
{
	sandboxId: String,
}

impl Component for GameElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGameId = use_radio::<Option<String>, GamePlatforms>(GamePlatforms::EpicGamesStore);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		
		let mut cancelled = use_state(bool::default);
		let mut confirmed = use_state(bool::default);
		let search = use_state(String::default);
		let mut showConfirmationDialog = use_state(bool::default);
		
		let game = user.read().getGame(&self.sandboxId)
			.unwrap_or_default();
		
		let achievements = game.filterAchievements(search.read().clone());
		let achievementsListLength = achievements.len();
		
		let sandboxId = game.sandboxId.clone();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, sandboxId),
			platform: EgsApi::Platform.to_lowercase(),
		});
		
		use_side_effect(move || {
			if (cancelled() || confirmed()) && showConfirmationDialog()
			{
				if confirmed()
				{
					if !sandboxId.is_empty()
					{
						spawn({
							let productId = game.productId.clone();
							let sandboxId = sandboxId.clone();
							async move {
								rateLimiter.read().pushAll(vec![
									EgsOperation::GetAchievementsList(sandboxId).into(),
									EgsOperation::GetAchievementProgress(productId).into(),
									EgsOperation::SaveToFile.into(),
								]).await;
								
								**requestEvent.write() = RequestEvent::Added;
							}
						});
					}
				}
				
				cancelled.set(false);
				confirmed.set(false);
				showConfirmationDialog.set(false);
			}
		});
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.expanded()
			.margin(Gaps::new(10.0, 0.0, 5.0, 0.0))
			.spacing(10.0)
			
			.on_global_key_up(move |e: Event<KeyboardEventData>| match e.code
			{
				Code::Home => scrollController.scroll_to(ScrollPosition::Start, Direction::Vertical),
				Code::End => scrollController.scroll_to(ScrollPosition::End, Direction::Vertical),
				_ => {},
			})
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new(5.0, 0.0, 5.0, 0.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.child(
						IconButton::new(lucide::arrow_big_left())
							.alt("Back")
							.onPress(move |_| **selectedGameId.write() = None)
					)
					
					.maybe_child(filePathExists(&iconPath).then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap_or_default()))
							.height(Size::px(64.0))
					))
					
					.child(
						label()
							.font_size(24.0)
							.text_align(TextAlign::Center)
							.text(game.name)
							.width(Size::flex(0.8))
					)
					
					.child(
						IconButton::new(lucide::refresh_ccw())
							.alt("Refresh")
							.onPress(move |_| showConfirmationDialog.set(true))
					)
			)
			
			.maybe_child((game.achievementsCount > 0).then(||
				rect()
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.margin(Gaps::new(5.0, 0.0, 5.0, 0.0))
					.width(Size::percent(50.0))
					
					.child(
						Input::new(search)
							.placeholder("Search by achievement name")
							.width(Size::Fill)
					)
			))
			
			.maybe_child((game.achievementsCount > 0).then(||
				VirtualScrollView::new_controlled(
					move |i, _| {
						let chievo = &achievements[i];
						return AchievementElement::new(
							game.sandboxId.clone(),
							chievo.id.clone()
						).into();
					},
					scrollController
				)
					.direction(Direction::Vertical)
					.item_size(105.0)
					.length(achievementsListLength)
					.scroll_with_arrows(true)
			))
			
			.maybe_child(showConfirmationDialog().then(||
				ConfirmRefresh::new(
					cancelled.into_writable(),
					confirmed.into_writable()
				)
			));
	}
}

impl GameElement
{
	pub fn new(sandboxId: impl Into<String>) -> Self
	{
		return Self
		{
			sandboxId: sandboxId.into(),
		};
	}
}
