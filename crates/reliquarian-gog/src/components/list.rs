use std::path::PathBuf;
use components::extensions::PressableExt;
use data::constants::{BorderColor, ButtonBackgroundColor, CornerRadius, FileName_GameIcon, GogProgressColor, Path_Games, RetroAchievementsProgressColorBackground};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Code, Color, Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction, Event, EventHandlersExt, FontWeight, Gaps, ImageViewer, Input, IntoElement, KeyboardEventData, ProgressBar, ProgressBarThemePartialExt, ScrollConfig, ScrollPosition, Size, Span, StyleExt, TextAlign, TextStyleExt, VirtualScrollView, label, paragraph, rect, use_scroll_controller, use_state};
use freya::radio::use_radio;
use macros::{join, jpg};
use crate::api::GogApi;
use crate::data::user::GogUser;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		let search = use_state(String::default);
		
		let games = user.read().filterGames(search.read().clone());
		let gamesLength = games.len();
		
		return rect()
			.content(Content::Flex)
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new(10.0, 0.0, 5.0, 0.0))
			.spacing(10.0)
			.width(Size::Fill)
			
			.on_global_key_up(move |e: Event<KeyboardEventData>| match e.code
			{
				Code::Home => scrollController.scroll_to(ScrollPosition::Start, Direction::Vertical),
				Code::End => scrollController.scroll_to(ScrollPosition::End, Direction::Vertical),
				_ => {},
			})
			
			.child(
				label()
					.font_size(32.0)
					.font_weight(FontWeight::BOLD)
					.text_align(TextAlign::Center)
					.width(Size::percent(100.0))
					.text("GOG")
			)
			
			.child(
				Input::new(search)
					.placeholder("Search by game title")
					.width(Size::percent(50.0))
			)
			
			.child(
				VirtualScrollView::new_controlled(
					move |i, _| {
						let game = &games[i];
						GameListNode::new(game.id).into()
					},
					scrollController
				)
					.direction(Direction::Vertical)
					.height(Size::flex(1.0))
					.item_size(105.0)
					.length(gamesLength)
					.scroll_with_arrows(true)
			);
	}
}

impl GameList
{
	pub fn new() -> Self
	{
		return Self {};
	}
}

#[derive(Clone, PartialEq)]
pub struct GameListNode
{
	gameId: u64,
}

impl Component for GameListNode
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::Gog);
		
		let hovering = use_state(|| false);
		
		let game = user.read().getGame(self.gameId)
			.unwrap_or_default();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: GogApi::Platform.to_lowercase(),
		});
		
		let background = match hovering()
		{
			false => Color::TRANSPARENT,
			true => ButtonBackgroundColor,
		};
		
		let progress = game.percentUnlocked();
		let progressString = format!("{:.2}", progress);
		let achievementsCount = game.achievements.len();
		let unlockedCount = game.achievements.iter()
			.filter(|a| a.dateUnlocked.is_some())
			.count();
		
		let name = game.name.clone();
		
		let showIcon = filePathExists(&iconPath);
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			.margin(Gaps::new_symmetric(5.0, 0.0))
			.min_height(Size::px(54.0))
			.width(Size::Fill)
			
			.child(
				rect()
					.background(background)
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(1.0)
					))
					.content(Content::Flex)
					.corner_radius(CornerRadius)
					.direction(Direction::Horizontal)
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.pressableWithHover(
						hovering,
						move |_| **selectedGameId.write() = Some(game.id)
					)
					
					.maybe_child(showIcon.then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap_or_default()))
							.corner_radius(CornerRadius)
							.height(Size::px(64.0))
					))
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.height(Size::px(64.0))
							.main_align(Alignment::Center)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.font_size(18.0)
									.text(name)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.height(Size::px(64.0))
							.main_align(Alignment::Center)
							.spacing(5.0)
							.width(Size::px(100.0))
							
							.maybe_child(game.hasAchievements.is_some_and(|b| b).then(||
								ProgressBar::new(progress)
									.background(RetroAchievementsProgressColorBackground)
									.height(8.0)
									.progress_background(GogProgressColor)
									.color(GogProgressColor)
							))
							
							.maybe_child(game.hasAchievements.is_some_and(|b| b).then(||
								paragraph()
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									
									.span(
										Span::new(format!("{} / {} ", unlockedCount, achievementsCount))
											.font_size(10.0)
									)
									
									.span(
										Span::new(format!("({}%) ", progressString))
											.font_size(10.0)
											.color(Color::GREY)
									)
							))
							
							.maybe_child(game.hasAchievements.is_none_or(|b| !b).then(||
								label()
									.font_size(10.0)
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									.text(match game.hasAchievements.is_none()
									{
										false => "No achievements",
										true => "Click to load"
									})
							))
					)
			);
	}
}

impl GameListNode
{
	pub fn new(gameId: impl Into<u64>) -> Self
	{
		return Self
		{
			gameId: gameId.into(),
		};
	}
}
