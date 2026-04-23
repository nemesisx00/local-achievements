use std::path::PathBuf;
use components::extensions::PressableExt;
use components::input::filter::GamesFilter;
use data::constants::{BorderColor, ButtonBackgroundColor, CornerRadius,
	Path_Games, RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorHardcore};
use data::enums::GamePlatforms;
use data::filter::{FilterCriteria, Filterable};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Code,
	Color, Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Content, Direction, Event, EventHandlersExt, FontWeight, Gaps, ImageViewer,
use freya::radio::{IntoWritable, use_radio};
	IntoElement, KeyboardEventData, ProgressBar, ProgressBarThemePartialExt,
	ScrollConfig, ScrollPosition, Size, Span, StyleExt, TextAlign, TextStyleExt,
	VirtualScrollView, label, paragraph, rect, use_scroll_controller, use_state};
use macros::join;
use crate::data::user::Rpcs3User;
use crate::api::api::Rpcs3Api;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<Rpcs3User, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let mut scrollController = use_scroll_controller(ScrollConfig::default);
		let caseSensitive = use_state(bool::default);
		let search = use_state(String::default);
		
		let games = user.read().filter(FilterCriteria
		{
			caseSensitive: caseSensitive(),
			text: search.read().clone(),
			..Default::default()
		});
		
		let gamesLength = games.len();
		
		return rect()
			.content(Content::Flex)
			.direction(Direction::Vertical)
			.cross_align(Alignment::Center)
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
					.text("RPCS3")
			)
			
			.child(
				GamesFilter::new(caseSensitive, search)
					.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
					.width(Size::percent(50.0))
			)
			
			.child(
				VirtualScrollView::new_controlled(
					move |i, _| {
						let game = &games[i];
						return GameListNode::new(game.npCommId.clone()).into();
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
	npCommId: String,
}

impl Component for GameListNode
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<Rpcs3User, GamePlatforms>(GamePlatforms::Rpcs3);
		let mut selectedGameId = use_radio::<Option<String>, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let game = user.read().getGame(self.npCommId.clone())
			.unwrap_or_default();
		
		let hovering = use_state(|| false);
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: Rpcs3Api::GameIconFileName.into(),
			group: join!(Path_Games, game.npCommId),
			platform: Rpcs3Api::Platform.to_lowercase(),
		});
		
		let background = match hovering()
		{
			false => Color::TRANSPARENT,
			true => ButtonBackgroundColor,
		};
		
		let progress = game.percentUnlocked();
		let progressString = format!("{:.2}", progress);
		let trophyCount = game.trophies.len();
		let unlockedCount = game.trophies.iter()
			.filter(|t| t.unlocked)
			.count();
		
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
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(15.0)
					.width(Size::percent(50.0))
					
					.pressableWithHover(
						hovering.into_writable(),
						move |_| **selectedGameId.write() = Some(game.npCommId.clone())
					)
							
					.maybe_child(filePathExists(&iconPath).then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap()))
							.corner_radius(CornerRadius)
							.height(Size::px(64.0))
					))
					
					.child(
						label()
							.font_size(18.0)
							.text(game.name)
							.width(Size::flex(1.0))
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.height(Size::px(64.0))
							.main_align(Alignment::Center)
							.spacing(5.0)
							.width(Size::px(100.0))
							
							.child(
								ProgressBar::new(progress)
									.background(RetroAchievementsProgressColorBackground)
									.height(8.0)
									.progress_background(RetroAchievementsProgressColorHardcore)
									.color(RetroAchievementsProgressColorHardcore)
									.width(Size::percent(100.0))
							)
							
							.child(
								paragraph()
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									
									.span(
										Span::new(format!("{} / {} ", unlockedCount, trophyCount))
											.font_size(10.0)
									)
									.span(
										Span::new(format!("({}%) ", progressString))
											.font_size(10.0)
											.color(Color::GREY)
									)
							)
					)
			);
	}
}

impl GameListNode
{
	pub fn new(npCommId: impl Into<String>) -> Self
	{
		return Self
		{
			npCommId: npCommId.into(),
		};
	}
}
