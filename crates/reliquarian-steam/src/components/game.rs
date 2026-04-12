use std::path::PathBuf;
use components::button::icon::IconButton;
use components::input::filter::AchievementsFilter;
use data::constants::{FileName_GameIcon, Path_Games};
use data::enums::{DataChannel, GamePlatforms};
use data::filter::{FilterCriteria, Filterable};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{Alignment, ChildrenExt, Code, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event,
	EventHandlersExt, Gaps, ImageViewer, IntoElement, KeyboardEventData,
	ScrollConfig, ScrollPosition, Size, TextAlign, TextStyleExt,
	VirtualScrollView, label, rect, spawn, use_memo, use_scroll_controller,
	use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, jpg};
use net::{RateLimiter, RequestEvent};
use crate::api::SteamApi;
use crate::data::operation::SteamOperation;
use crate::data::user::SteamUser;
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
		let user = use_radio::<SteamUser, GamePlatforms>(GamePlatforms::Steam);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::Steam);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		
		let caseSensitive = use_state(bool::default);
		let locked = use_state(bool::default);
		let nameOnly = use_state(bool::default);
		let search = use_state(String::default);
		
		let game = user.read().getGame(self.gameId)
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
					AchievementsFilter::new(
						caseSensitive.into_writable(),
						locked.into_writable(),
						nameOnly.into_writable(),
						search.into_writable()
					)
						.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
						.width(Size::percent(50.0))
				)
				
				.maybe_child((!game.hasAchievements).then(||
					label()
						.text_align(TextAlign::Center)
						.width(Size::Fill)
						.text("No Achievements to display")
				))
				
				.maybe_child(game.hasAchievements.then(||
					VirtualScrollView::new_controlled(move |i, _| {
							let chievo = &achievements.read()[i];
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
