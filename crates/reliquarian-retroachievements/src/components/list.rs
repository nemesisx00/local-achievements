use std::path::PathBuf;
use components::extensions::PressableExt;
use data::constants::{BorderColor, ButtonBackgroundColor, CornerRadius,
	FileName_GameIcon, Path_Games, RetroAchievementsGameBeaten,
	RetroAchievementsGameBeatenCasual, RetroAchievementsGameComplete,
	RetroAchievementsGameMastered, RetroAchievementsGameUnfinished,
	RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorCasual,
	RetroAchievementsProgressColorHardcore};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	ChildrenExt, Code, Color, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, Event, EventHandlersExt,
	FontWeight, Gaps, ImageViewer, Input, IntoElement, KeyboardEventData, Layer,
	LayerExt, Position, ProgressBar, ProgressBarThemePartialExt, ScrollConfig,
	ScrollPosition, Size, Span, StyleExt, TextAlign, TextStyleExt,
	VirtualScrollView, label, paragraph, rect, svg, use_scroll_controller,
	use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::{join, png};
use crate::api::RetroAchievementsApi;
use crate::data::kind::AwardKind;
use crate::data::mode::RetroAchievementsMode;
use crate::data::settings::RetroAchievementsSettings;
use crate::data::user::RetroAchievementsUser;

#[derive(Clone, PartialEq)]
pub struct GameList;

impl Component for GameList
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		
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
		let settings = use_radio::<RetroAchievementsSettings, GamePlatforms>(GamePlatforms::RetroAchievements);
		let user = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		let mut selectedGameId = use_radio::<Option<u64>, GamePlatforms>(GamePlatforms::RetroAchievements);
		
		let hovering = use_state(|| false);
		
		let game = user.read().getGame(self.gameId)
			.unwrap_or_default();
		
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
		
		let awardImage = match settings.read().showGameAwardBadges
		{
			false => None,
			true => awardImageElement(game.highestAward)
		};
		
		let progressCasual = game.percentUnlocked(RetroAchievementsMode::Casual);
		let progressCasualString = format!("{:.2}", progressCasual);
		let progressHardcore = game.percentUnlocked(RetroAchievementsMode::Hardcore);
		let progressHardcoreString = format!("{:.2}", progressHardcore);
		
		let achievementsCount= game.achievements.len();
		let casualUnlockedCount = game.achievements.iter()
			.filter(|a| a.unlocked(RetroAchievementsMode::Casual))
			.count();
		let hardcoreUnlockedCount = game.achievements.iter()
			.filter(|a| a.unlocked(RetroAchievementsMode::Hardcore))
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
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.pressableWithHover(
						hovering.into_writable(),
						move |_| **selectedGameId.write() = Some(game.id)
					)
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Horizontal)
							.spacing(15.0)
							.width(Size::flex(1.0))
							
							.maybe_child(filePathExists(&iconPath).then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
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
											.text(game.name)
									)
									
									.child(
										label()
											.font_size(12.0)
											.text(game.system.name)
									)
							)
					)
					
					.maybe_child(awardImage)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.height(Size::px(64.0))
							.main_align(Alignment::Center)
							.min_height(Size::px(40.0))
							.spacing(5.0)
							.width(Size::px(100.0))
							
							.child(
								ProgressBar::new(progressCasual)
									.background(RetroAchievementsProgressColorBackground)
									.height(8.0)
									.progress_background(RetroAchievementsProgressColorCasual)
									.color(RetroAchievementsProgressColorCasual)
							)
							
							.child(
								rect()
									.layer(Layer::Relative(2))
									.position(Position::new_absolute()
										.right(0.0)
										.top(11.0)
									)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(progressHardcore)
											.background(Color::TRANSPARENT)
											.height(8.0)
											.progress_background(RetroAchievementsProgressColorHardcore)
											.color(RetroAchievementsProgressColorHardcore)
									)
							)
							
							.child(
								paragraph()
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									
									.span(
										Span::new(format!("{} / {} ", casualUnlockedCount, achievementsCount))
											.font_size(10.0)
									)
									
									.span(
										Span::new(format!("({}%)", progressCasualString))
											.color(Color::GRAY)
											.font_size(10.0)
									)
							)
							
							.child(
								paragraph()
									.text_align(TextAlign::Center)
									.width(Size::px(100.0))
									
									.span(
										Span::new(format!("{} / {} ", hardcoreUnlockedCount, achievementsCount))
											.font_size(10.0)
											.font_weight(FontWeight::BOLD)
									)
									
									.span(
										Span::new(format!("({}%)", progressHardcoreString))
											.color(Color::GRAY)
											.font_size(10.0)
											.font_weight(FontWeight::BOLD)
									)
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

fn awardImageElement(award: Option<AwardKind>) -> Option<impl IntoElement>
{
	let size = Size::px(24.0);
	
	return match award
	{
		None => None,
		Some(award) => match award
		{
			AwardKind::BeatenCasual => Some(
				svg(lucide::circle())
					.a11y_alt("Beaten (Casual)")
					.color(RetroAchievementsGameBeatenCasual)
					.fill(RetroAchievementsGameUnfinished)
					.height(size.clone())
					.width(size.clone())
			),
			
			AwardKind::BeatenHardcore => Some(
				svg(lucide::circle())
					.a11y_alt("Beaten")
					.color(RetroAchievementsGameBeaten)
					.fill(RetroAchievementsGameBeaten)
					.height(size.clone())
					.width(size.clone())
			),
			
			AwardKind::Completed => Some(
				svg(lucide::circle())
					.a11y_alt("Completed")
					.color(RetroAchievementsGameComplete)
					.fill(RetroAchievementsGameUnfinished)
					.height(size.clone())
					.width(size.clone())
			),
			
			AwardKind::Mastered => Some(
				svg(lucide::circle())
					.a11y_alt("Mastered")
					.color(RetroAchievementsGameMastered)
					.fill(RetroAchievementsGameMastered)
					.height(size.clone())
					.width(size.clone())
			),
		}
	};
}
