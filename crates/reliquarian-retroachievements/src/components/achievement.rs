use std::path::PathBuf;
use data::constants::{BorderColor, CornerRadius, Icon_Locked, Path_Games,
	RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorCasual,
	RetroAchievementsProgressColorHardcore};
use data::enums::GamePlatforms;
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt, Color,
	Component, ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content,
	Direction, FontWeight, Gaps, ImageViewer, IntoElement, Layer, LayerExt,
	Position, ProgressBar, ProgressBarThemePartialExt, Size, Span, StyleExt,
	TextAlign, TextStyleExt, label, paragraph, rect};
use freya::radio::use_radio;
use macros::{join, png, pngAlt};
use crate::api::RetroAchievementsApi;
use crate::data::mode::RetroAchievementsMode;
use crate::data::user::RetroAchievementsUser;

#[derive(Clone, PartialEq)]
pub struct AchievementElement
{
	achievementId: u64,
	gameId: u64,
}

impl Component for AchievementElement
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		
		let distinctPlayers = user.read()
			.getDistinctPlayersForGame(self.gameId)
			.unwrap_or(0);
		
		let achievement = user.read()
			.getAchievement(self.gameId, self.achievementId)
			.unwrap_or_default();
		
		let unlockedCasual = achievement.unlocked(RetroAchievementsMode::Casual);
		let unlockedHardcore = achievement.unlocked(RetroAchievementsMode::Hardcore);
		
		let iconPath = getImagePath(&FileLocation
		{
			fileName: match unlockedCasual || unlockedHardcore
			{
				false => pngAlt!(RetroAchievementsApi::sanitizeIconTitle(&achievement.name), Icon_Locked),
				true => png!(RetroAchievementsApi::sanitizeIconTitle(&achievement.name)),
			},
			group: join!(Path_Games, self.gameId),
			platform: RetroAchievementsApi::Platform.to_lowercase(),
		});
		
		let timestamp = match achievement.formatEarnedTimestamp(match unlockedHardcore
			{
				false => RetroAchievementsMode::Casual,
				true => RetroAchievementsMode::Hardcore
			})
		{
			Err(_) => Default::default(),
			Ok(ts) => ts
		};
		
		let percentCasual = achievement.unlockedPercent(
			RetroAchievementsMode::Casual,
			distinctPlayers
		);
		
		let percentHardcore = achievement.unlockedPercent(
			RetroAchievementsMode::Hardcore,
			distinctPlayers
		);
		
		let percentCasualString = format!("{:.2}", percentCasual);
		let percentHardcoreString = format!("{:.2}", percentHardcore);
		
		let showIcon = filePathExists(&iconPath);
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			.width(Size::Fill)
			
			.child(
				rect()
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(1.0)
					))
					.content(Content::Flex)
					.corner_radius(CornerRadius)
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new_symmetric(5.0, 0.0))
					.min_height(Size::px(64.0))
					.min_width(Size::px(540.0))
					.padding(Gaps::new_all(10.0))
					.spacing(15.0)
					.width(Size::percent(50.0))
					
					.maybe_child(showIcon.then(||
						ImageViewer::new(PathBuf::from(iconPath.unwrap()))
							.corner_radius(CornerRadius)
							.height(Size::px(64.0))
					))
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child(
								label()
									.text(achievement.name)
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(achievement.description)
									.width(Size::flex(1.0))
							)
							
							.child(
								label()
									.font_size(10.0)
									.text(timestamp)
									.width(Size::flex(1.0))
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.main_align(Alignment::Center)
							.height(Size::px(64.0))
							.spacing(5.0)
							.width(Size::px(150.0))
							
							.child(
								ProgressBar::new(percentCasual as f32)
									.background(RetroAchievementsProgressColorBackground)
									.height(8.0)
									.color(RetroAchievementsProgressColorCasual)
									.progress_background(RetroAchievementsProgressColorCasual)
									.width(Size::px(100.0))
							)
							
							.child(
								rect()
									.layer(Layer::Relative(2))
									.position(Position::new_absolute()
										.right(0.0)
										.top(10.5)
									)
									.width(Size::px(100.0))
									
									.child(
										ProgressBar::new(percentHardcore as f32)
											.background(Color::TRANSPARENT)
											.height(8.0)
											.color(RetroAchievementsProgressColorHardcore)
											.progress_background(RetroAchievementsProgressColorHardcore)
									)
							)
							
							.child(
								paragraph()
									.margin(Gaps::new(1.0, 0.0, 0.0, 0.0))
									.text_align(TextAlign::End)
									.width(Size::percent(100.0))
									
									.span(
										Span::new(format!("{} of {} ", achievement.awardedCasual, distinctPlayers))
											.font_size(10.0)
									)
									
									.span(
										Span::new(format!("({}%)", percentCasualString))
											.font_size(10.0)
									)
							)
							
							.child(
								paragraph()
									.text_align(TextAlign::End)
									.width(Size::percent(100.0))
									
									.span(
										Span::new(format!("{} of {} ", achievement.awardedHardcore, distinctPlayers))
											.font_size(10.0)
											.font_weight(FontWeight::BOLD)
									)
									
									.span(
										Span::new(format!("({}%)", percentHardcoreString))
											.font_size(10.0)
											.font_weight(FontWeight::BOLD)
									)
							)
					)
			);
	}
}

impl AchievementElement
{
	pub fn new(gameId: u64, achievementId: u64) -> Self
	{
		return Self
		{
			achievementId,
			gameId,
		};
	}
}
