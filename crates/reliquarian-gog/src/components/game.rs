use std::path::PathBuf;
use components::button::icon::IconButton;
use components::input::filter::AchievementsFilter;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{CornerRadius, FileName_GameIcon, Path_Games};
use data::enums::{DataChannel, GamePlatforms};
use data::filter::{FilterCriteria, Filterable};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, FontWeight, Gaps, ImageViewer, IntoElement,
	KeyboardEventData, ScrollConfig, ScrollPosition, Size, TextAlign,
	TextStyleExt, VirtualScrollView, WritableUtils, label, rect, spawn,
	use_scroll_controller, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, jpg};
use net::{RateLimiter, RequestEvent};
use crate::api::GogApi;
use crate::data::operation::GogOperation;
use crate::data::user::GogUser;
use super::achievement::AchievementElement;

#[derive(Clone, PartialEq)]
pub struct GameElement
{
	gameId: u64,
}

impl Component for GameElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::Gog);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		
		let mut cancelled = use_state(bool::default);
		let caseSensitive = use_state(bool::default);
		let mut confirmed = use_state(bool::default);
		let locked = use_state(bool::default);
		let nameOnly = use_state(bool::default);
		let search = use_state(String::default);
		let mut showConfirmationDialog = use_state(bool::default);
		
		let game = user.read().getGame(self.gameId)
			.unwrap_or_default();
		
		let achievements = game.filter(FilterCriteria
		{
			caseSensitive: caseSensitive(),
			locked: locked(),
			nameOnly: nameOnly(),
			text: search.read().clone(),
		});
		
		let achievementsListLength = achievements.len();
		
		let gameId = game.id;
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: GogApi::Platform.to_lowercase(),
		});
		
		use_side_effect(move || {
			if (cancelled() || confirmed()) && showConfirmationDialog()
			{
				if confirmed()
				{
					if gameId > 0
					{
						spawn(async move {
							rateLimiter.read().pushAll(vec![
								GogOperation::RefreshSession.into(),
								GogOperation::GetAchievements(gameId).into(),
								GogOperation::SaveToFile.into(),
							]).await;
							
							**requestEvent.write() = RequestEvent::Added;
						});
					}
				}
				
				cancelled.set(false);
				confirmed.set(false);
				showConfirmationDialog.set(false);
			}
		});
		
		return rect()
			.content(Content::Flex)
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
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.height(Size::px(64.0))
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
							.corner_radius(CornerRadius)
							.height(Size::px(64.0))
					))
					
					.child(
						label()
							.font_size(24.0)
							.font_weight(FontWeight::BOLD)
							.text_align(TextAlign::Center)
							.text(game.name)
							.width(Size::flex(1.0))
					)
					
					.child(
						IconButton::new(lucide::refresh_ccw())
							.alt("Refresh")
							.onPress(move |_| showConfirmationDialog.set(true))
					)
			)
			
			.child(
				AchievementsFilter::new(
					caseSensitive.into_writable(),
					locked.into_writable(),
					nameOnly.into_writable(),
					search.into_writable()
				)
					.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
					.width(Size::percent(50.0))
			)
			
			.maybe_child((game.hasAchievements.is_none_or(|b| !b)).then(||
				rect()
					.center()
					.direction(Direction::Vertical)
					.height(Size::percent(100.0))
					.width(Size::percent(100.0))
					.child("No achievements to display")
			))
			
			.maybe_child(game.hasAchievements.is_some_and(|b| b).then(||
				VirtualScrollView::new_controlled(
					move |i, _| {
						let chievo = &achievements[i];
						return AchievementElement::new(game.id, chievo.id.clone()).into();
					},
					scrollController
				)
					.direction(Direction::Vertical)
					.height(Size::flex(1.0))
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
	pub fn new(gameId: u64) -> Self
	{
		return Self
		{
			gameId,
		};
	}
}
