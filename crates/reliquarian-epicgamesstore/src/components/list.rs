use std::path::PathBuf;
use components::extensions::PressableExt;
use data::constants::{BorderColor, ButtonBackgroundColor, FileName_GameIcon,
	Path_Games, RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorHardcore};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Code,
	Color, Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	CornerRadius, Direction, Event, EventHandlersExt, FontWeight, Gaps,
	ImageViewer, Input, IntoElement, KeyboardEventData, ProgressBar,
	ProgressBarThemePartialExt, ScrollConfig, ScrollPosition, Size, StyleExt,
	TextAlign, TextStyleExt, VirtualScrollView, label, rect,
	use_scroll_controller, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, jpg};
use crate::api::EgsApi;
use crate::data::user::EgsUser;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
		
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
					.text("Epic Games Store")
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
						GameListNode::new(game.sandboxId.clone()).into()
					},
					scrollController
				)
					.direction(Direction::Vertical)
					.height(Size::percent(100.0))
					.item_size(81.0)
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
	sandboxId: String,
}

impl Component for GameListNode
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
		let mut selectedGameId = use_radio::<Option<String>, GamePlatforms>(GamePlatforms::EpicGamesStore);
		
		let hovering = use_state(|| false);
		
		let game = user.read().getGame(&self.sandboxId)
			.unwrap_or_default();
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: jpg!(FileName_GameIcon),
			group: join!(Path_Games, game.sandboxId),
			platform: EgsApi::Platform.to_lowercase(),
		});
		
		let background = match hovering()
		{
			false => Color::TRANSPARENT,
			true => ButtonBackgroundColor,
		};
		
		let progress = game.percentUnlocked();
		let progressString = format!("{:.2}%", progress);
		let name = game.name.clone();
		
		let showIcon = filePathExists(&iconPath);
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			.margin(Gaps::new_symmetric(5.0, 0.0))
			.min_height(Size::px(86.0))
			.width(Size::percent(100.0))
			
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
					.padding(Gaps::new_symmetric(0.0, 15.0))
					//.spacing(10.0)
					.width(Size::percent(50.0))
					
					.pressableWithHover(
						hovering.into_writable(),
						move |_| **selectedGameId.write() = Some(game.sandboxId.clone())
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.spacing(15.0)
							
							.maybe_child((!showIcon).then(||
								rect()
									.height(Size::px(64.0))
									.width(Size::px(64.0))
							))
							
							.maybe_child(showIcon.then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap_or_default()))
									.height(Size::px(64.0))
							))
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.min_height(Size::px(64.0))
									.main_align(Alignment::Center)
									.width(Size::percent(50.0))
									
									.child(
										label()
											.font_size(18.0)
											.text(name)
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.main_align(Alignment::Center)
							.min_height(Size::px(64.0))
							.min_width(Size::px(100.0))
							.spacing(5.0)
							
							.maybe_child((game.achievementsCount > 0).then(||
								rect()
									.direction(Direction::Horizontal)
									.main_align(Alignment::End)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(progress)
											.background(RetroAchievementsProgressColorBackground)
											.height(8.0)
											.progress_background(RetroAchievementsProgressColorHardcore)
											.color(RetroAchievementsProgressColorHardcore)
									)
							))
							
							.maybe_child((game.achievementsCount > 0).then(||
								label()
									.font_size(10.0)
									.font_weight(FontWeight::BOLD)
									.min_width(Size::px(100.0))
									.text_align(TextAlign::Center)
									.text(progressString)
							))
							
							.maybe_child((game.achievementsCount <= 0).then(||
								label()
									.font_size(10.0)
									.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
									.text_align(TextAlign::End)
									.width(Size::percent(100.0))
									.text("Click to load")
							))
					)
			);
	}
}

impl GameListNode
{
	pub fn new(sandboxId: impl Into<String>) -> Self
	{
		return Self
		{
			sandboxId: sandboxId.into(),
		};
	}
}
