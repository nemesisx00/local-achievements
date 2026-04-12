use std::path::PathBuf;
use components::extensions::PressableExt;
use data::constants::{BorderColor, ButtonBackgroundColor, FileName_GameIcon,
	Path_Games, RetroAchievementsProgressColorBackground, SteamContrast};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Code,
	Color, Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Event, EventHandlersExt, Gaps, ImageViewer, Input, IntoElement,
	KeyboardEventData, Layer, LayerExt, Position, ProgressBar,
	ProgressBarThemePartialExt, ScrollConfig, ScrollPosition, Size, StyleExt,
	TextAlign, TextStyleExt, VirtualScrollView, label, rect,
	use_scroll_controller, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, jpg};
use crate::api::SteamApi;
use crate::data::user::SteamUser;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<SteamUser, GamePlatforms>(GamePlatforms::Steam);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		let search = use_state(String::default);
		
		let games = user.read().filterGames(search.read().clone());
		let gamesLength = games.len();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
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
					.font_size(24.0)
					.text_align(TextAlign::Center)
					.width(Size::percent(100.0))
					.text("Steam")
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
		let user = use_radio::<SteamUser, GamePlatforms>(GamePlatforms::Steam);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::Steam);
		
		let game = user.read().getGame(self.gameId)
			.unwrap_or_default();
		
		let hovering = use_state(|| false);
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: SteamApi::Platform.to_lowercase(),
		});
		
		let background = match hovering()
		{
			false => Color::TRANSPARENT,
			true => ButtonBackgroundColor,
		};
		
		let percentUnlocked = game.percentUnlocked();
		let percentUnlockedString = format!("{:.2}", percentUnlocked);
		
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
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.pressableWithHover(
						hovering.into_writable(),
						move |_| **selectedGameId.write() = Some(game.id)
					)
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.spacing(15.0)
							
							.maybe_child(filePathExists(&iconPath).then(||
								rect()
									.cross_align(Alignment::Center)
									.direction(Direction::Vertical)
									.height(Size::px(64.0))
									.main_align(Alignment::Center)
									.child(
										ImageViewer::new(PathBuf::from(iconPath.unwrap()))
											.height(Size::px(48.0))
											.width(Size::px(48.0))
									)
							))
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.main_align(Alignment::SpaceAround)
									
									.child(
										label()
											.margin(Gaps::new(10.0, 0.0, 0.0, 0.0))
											.font_size(18.0)
											.text(game.name)
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.main_align(Alignment::SpaceAround)
							.min_width(Size::px(150.0))
							.min_height(Size::px(40.0))
							.width(Size::px(100.0))
							
							.maybe_child(game.hasAchievements.then(||
								rect()
									.layer(Layer::Relative(2))
									.position(Position::new_absolute()
										.right(0.0)
										.top(10.0)
									)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(percentUnlocked as f32)
											.background(RetroAchievementsProgressColorBackground)
											.color(SteamContrast)
											.height(8.0)
											.progress_background(SteamContrast)
									)
							))
							
							.maybe_child(game.hasAchievements.then(||
								label()
									.margin(Gaps::new(10.0, 0.0, 0.0, 0.0))
									.font_size(10.0)
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									.text(format!("{}%", percentUnlockedString))
							))
							
							.maybe_child((!game.hasAchievements && game.loaded).then(||
								label()
									.font_size(10.0)
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									.text("Achievements N/A")
							))
							
							.maybe_child((!game.hasAchievements && !game.loaded).then(||
								label()
									.font_size(10.0)
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									.text("Click to Load")
							))
					)
			);
	}
}

impl GameListNode
{
	pub fn new(id: u64) -> Self
	{
		return Self
		{
			gameId: id,
		};
	}
}
