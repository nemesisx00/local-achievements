use std::path::PathBuf;
use components::button::icon::IconButton;
use components::input::filter::AchievementsFilter;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{FileName_GameIcon, Path_Games};
use data::enums::{DataChannel, GamePlatforms};
use data::filter::{FilterCriteria, Filterable};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, ImageViewer, IntoElement, KeyboardEventData,
	ScrollConfig, ScrollPosition, Size, TextAlign, TextStyleExt,
	VirtualScrollView, WritableUtils, label, rect, spawn, use_memo,
	use_scroll_controller, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, png};
use net::{RateLimiter, RequestEvent};
use crate::api::RetroAchievementsApi;
use crate::data::operation::RetroAchievementsOperation;
use crate::data::user::RetroAchievementsUser;
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
		let user = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::RetroAchievements);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		
		let caseSensitive = use_state(bool::default);
		let mut cancelled = use_state(bool::default);
		let mut confirmed = use_state(bool::default);
		let locked = use_state(bool::default);
		let nameOnly = use_state(bool::default);
		let search = use_state(String::default);
		let mut showConfirmationDialog = use_state(bool::default);
		
		let game = user.read()
			.getGame(self.gameId)
			.unwrap_or_default();
		
		let achievements = use_memo({
			let game = game.clone();
			move || {
				game.filter(FilterCriteria
				{
					caseSensitive: caseSensitive(),
					locked: locked(),
					nameOnly: nameOnly(),
					text: search.read().clone(),
				})
			}
		});
		let achievementsListLength = achievements.read().len();
		
		let gameId = game.id;
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: png!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: RetroAchievementsApi::Platform.to_lowercase(),
		});
		
		use_side_effect(move || {
			if (cancelled() || confirmed()) && showConfirmationDialog()
			{
				if confirmed() && gameId > 0
				{
					spawn(async move {
						rateLimiter.read().pushAll(vec![
							RetroAchievementsOperation::GetGameInfo(gameId).into(),
							RetroAchievementsOperation::SaveToFile.into(),
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
						ImageViewer::new(PathBuf::from(iconPath.unwrap()))
							.width(Size::px(64.0))
					))
					
					.child(
						label()
							.font_size(24.0)
							.text_align(TextAlign::Center)
							.text(format!("{} ({})", game.name, game.system.name))
							.width(Size::flex(0.8))
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
			
			.child(
				VirtualScrollView::new_controlled(
					move |i, _| {
						let chievo = &achievements.read()[i];
						return AchievementElement::new(game.id, chievo.id).into();
					},
					scrollController
				)
					.direction(Direction::Vertical)
					.item_size(105.0)
					.length(achievementsListLength)
					.scroll_with_arrows(true)
			)
			
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
