use std::path::PathBuf;
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, ImageViewer, Input, IntoElement, KeyboardEventData,
	ScrollConfig, ScrollPosition, Size, TextAlign, TextStyleExt,
	VirtualScrollView, label, rect, spawn, use_scroll_controller, use_state};
use freya::radio::use_radio;
use crate::components::IconButton;
use crate::data::AppData;
use crate::data::radio::{AppDataChannel, DataChannel, GameIdChannel};
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{FileLocation, RequestEvent, SteamOperation};
use crate::steam::components::achievement::AchievementElement;
use crate::util::filePathExists;
use crate::{join, jpg};
use crate::io::{FileName_GameIcon, Path_Games, getImagePath};
use crate::steam::platform::api::SteamApi;

#[derive(Clone, PartialEq)]
pub struct GameElement
{
	gameId: u64,
}

impl Component for GameElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Steam);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGameId = use_radio::<Option<u64>, GameIdChannel>(GameIdChannel::Steam);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		let search = use_state(String::default);
		
		let game = appData.read().user.steam.getGame(self.gameId)
			.unwrap_or_default();
		
		let achievements = game.filterAchievements(search.read().clone());
		let achievementsListLength = achievements.len();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: SteamApi::Platform.to_lowercase(),
		});
		
		let gameId = self.gameId;
		
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
								.text(game.name)
								.width(Size::flex(0.9))
						)
						
						.child(
							IconButton::new(lucide::refresh_ccw())
								.alt("Refresh")
								.onPress(move |_| if gameId > 0
								{
									spawn(async move {
										rateLimiter.read().pushAll(vec![
											SteamOperation::GetSchemaForGame(gameId).into(),
											SteamOperation::GetPlayerAchievements(gameId).into(),
											SteamOperation::GetGlobalPercentages(gameId).into(),
											SteamOperation::SetGameLoaded(gameId, true).into(),
											SteamOperation::SaveToFile.into(),
										]).await;
										
										**requestEvent.write() = RequestEvent::Added;
									});
								})
						)
				)
				
				.child(
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
				)
				
				.maybe_child((!game.hasAchievements).then(||
					label()
						.text_align(TextAlign::Center)
						.width(Size::Fill)
						.text("No Achievements to display")
				))
				
				.maybe_child(game.hasAchievements.then(||
					VirtualScrollView::new_controlled(move |i, _| {
							let chievo = &achievements[i];
							AchievementElement::new(gameId, chievo.id.clone()).into()
						},
						scrollController
					)
						.direction(Direction::Vertical)
						.item_size(105.0)
						.length(achievementsListLength)
						.scroll_with_arrows(true)
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
