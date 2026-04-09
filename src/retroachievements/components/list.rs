use std::path::PathBuf;
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Code,
	Color, Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	CornerRadius, CursorIcon, Direction, Event, EventHandlersExt, FontWeight,
	Gaps, ImageViewer, Input, IntoElement, KeyboardEventData, Layer, LayerExt,
	Platform, Position, ProgressBar, ProgressBarThemePartialExt, ScrollConfig,
	ScrollPosition, Size, Span, StyleExt, TextAlign, TextStyleExt,
	VirtualScrollView, WinitPlatformExt, WritableUtils, label, paragraph, rect,
	use_scroll_controller, use_state};
use freya::radio::use_radio;
use crate::constants::{BorderColor, ButtonBackgroundColor,
	RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorCasual,
	RetroAchievementsProgressColorHardcore};
use crate::data::AppData;
use crate::data::radio::{AppDataChannel, GameIdChannel};
use crate::net::limiter::request::FileLocation;
use crate::util::filePathExists;
use crate::{join, png};
use crate::io::{FileName_GameIcon, Path_Games, getImagePath};
use crate::retroachievements::RetroAchievementsMode;
use crate::retroachievements::platform::api::RetroAchievementsApi;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::RetroAchievements);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		let search = use_state(String::default);
		
		let games = appData.read().user.retroAchievements.filterGames(search.read().clone());
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
					.text("Retro Achievements")
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
						return GameListNode::new(game.id).into();
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
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::RetroAchievements);
		let mut selectedGameId = use_radio::<Option<u64>, GameIdChannel>(GameIdChannel::RetroAchievements);
		
		let game = appData.read().user.retroAchievements.getGame(self.gameId)
			.unwrap_or_default();
		
		let mut hovering = use_state(|| false);
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: png!(FileName_GameIcon),
			group: join!(Path_Games, game.id),
			platform: RetroAchievementsApi::Platform.to_lowercase(),
		});
		
		let background = match hovering()
		{
			false => Color::TRANSPARENT,
			true => ButtonBackgroundColor,
		};
		
		let progressCasual = game.percentUnlocked(RetroAchievementsMode::Casual);
		let progressCasualString = format!("{:.2}", progressCasual);
		let progressHardcore = game.percentUnlocked(RetroAchievementsMode::Hardcore);
		let progressHardcoreString = format!("{:.2}", progressHardcore);
		
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
					.corner_radius(CornerRadius::new_all(5.0))
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.on_press(move |_| {
						Platform::get().with_window(
							None,
							move |window| window.set_cursor(CursorIcon::default())
						);
						**selectedGameId.write() = Some(game.id);
					})
					
					.on_pointer_enter(move |_| {
						Platform::get().with_window(
							None,
							move |window| window.set_cursor(CursorIcon::Pointer)
						);
						hovering.set(true);
					})
					
					.on_pointer_leave(move |_| {
						Platform::get().with_window(
							None,
							move |window| window.set_cursor(CursorIcon::default())
						);
						hovering.set(false);
					})
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.spacing(15.0)
							
							.maybe_child(filePathExists(&iconPath).then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.width(Size::px(64.0))
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
									
									.child(
										label()
											.font_size(12.0)
											.text(game.system.name)
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.main_align(Alignment::SpaceAround)
							.min_height(Size::px(40.0))
							.min_width(Size::px(150.0))
							.width(Size::px(100.0))
							
							.child(
								rect()
									.layer(Layer::Relative(1))
									.position(Position::new_absolute()
										.right(0.0)
										.top(10.0)
									)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(progressCasual as f32)
											.background(RetroAchievementsProgressColorBackground)
											.height(8.0)
											.progress_background(RetroAchievementsProgressColorCasual)
											.color(RetroAchievementsProgressColorCasual)
									)
							)
							
							.child(
								rect()
									.layer(Layer::Relative(2))
									.position(Position::new_absolute()
										.right(0.0)
										.top(10.0)
									)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(progressHardcore as f32)
											.background(Color::TRANSPARENT)
											.height(8.0)
											.progress_background(RetroAchievementsProgressColorHardcore)
											.color(RetroAchievementsProgressColorHardcore)
									)
							)
							
							.child(
								paragraph()
									.margin(Gaps::new(10.0, 0.0, 0.0, 0.0))
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									
									.span(Span::new(format!("{}% ", progressCasualString)).font_size(10.0))
									.span(Span::new(format!("({}%)", progressHardcoreString)).font_size(10.0).font_weight(FontWeight::BOLD))
							)
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
