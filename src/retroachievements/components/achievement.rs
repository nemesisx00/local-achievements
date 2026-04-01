use std::path::PathBuf;
use freya::prelude::{Alignment, Border, BorderAlignment, ChildrenExt,
	CircularLoader, Color, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, CornerRadius, Direction, FontWeight, Gaps,
	ImageViewer, IntoElement, Layer, LayerExt, Position, ProgressBar,
	ProgressBarThemePartialExt, Size, Span, StyleExt, TextAlign,
	TextStyleExt, label, paragraph, rect};
use freya::radio::use_radio;
use crate::constants::{BorderColor, Icon_Locked,
	RetroAchievementsProgressColorBackground,
	RetroAchievementsProgressColorCasual,
	RetroAchievementsProgressColorHardcore};
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::io::{Path_Games, getImagePath};
use crate::net::limiter::request::FileLocation;
use crate::retroachievements::RetroAchievementsMode;
use crate::retroachievements::platform::api::RetroAchievementsApi;
use crate::{join, png, pngAlt};

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
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::RetroAchievements);
		
		let distinctPlayers = appData.read().user.retroAchievements
			.getDistinctPlayersForGame(self.gameId)
			.unwrap_or(0);
		
		let achievement = appData.read().user.retroAchievements
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
		
		return rect()
			.direction(Direction::Horizontal)
			.main_align(Alignment::SpaceAround)
			.margin(Gaps::new_symmetric(5.0, 0.0))
			.width(Size::Fill)
			
			.child(
				rect()
					.border(Some(
						Border::new()
							.alignment(BorderAlignment::Center)
							.fill(BorderColor)
							.width(1.0)
					))
					.corner_radius(CornerRadius::new_all(10.0))
					.direction(Direction::Horizontal)
					.main_align(Alignment::SpaceBetween)
					.margin(Gaps::new_all(5.0))
					.min_height(Size::px(64.0))
					.min_width(Size::px(540.0))
					.padding(Gaps::new_symmetric(10.0, 15.0))
					.spacing(10.0)
					.width(Size::percent(50.0))
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.min_height(Size::px(64.0))
							.spacing(10.0)
							.width(Size::Fill)
							
							.maybe_child(iconPath.is_none().then(||
								CircularLoader::new()
							))
							
							.maybe_child(iconPath.is_some().then(||
								ImageViewer::new(PathBuf::from(iconPath.unwrap()))
									.width(Size::px(64.0))
							))
							
							.child(
								rect()
									.direction(Direction::Vertical)
									.main_align(Alignment::SpaceBetween)
									.spacing(15.0)
									.width(Size::percent(60.0))
									
									.child(label().text(achievement.name))
									
									.child(
										label()
											.font_size(10.0)
											.max_height(Size::px(48.0))
											.text(achievement.description)
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::End)
							.direction(Direction::Vertical)
							.main_align(Alignment::SpaceBetween)
							.min_width(Size::px(150.0))
							.width(Size::px(150.0))
							
							.child(
								rect()
									.cross_align(Alignment::End)
									.direction(Direction::Vertical)
									.main_align(Alignment::Start)
									.width(Size::Fill)
									
									.child(
										rect()
											.layer(Layer::Relative(1))
											.position(Position::new_absolute()
												.right(0.0)
												.top(0.0)
											)
											.width(Size::px(150.0))
											
											.child(
												ProgressBar::new(percentCasual as f32)
													.background(RetroAchievementsProgressColorBackground)
													.height(8.0)
													.color(RetroAchievementsProgressColorCasual)
													.progress_background(RetroAchievementsProgressColorCasual)
											)
									)
									
									.child(
										rect()
											.layer(Layer::Relative(2))
											.position(Position::new_absolute()
												.right(0.0)
												.top(0.0)
											)
											.width(Size::px(150.0))
											
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
											.margin(Gaps::new(15.0, 0.0, 0.0, 0.0))
											.text_align(TextAlign::Center)
											.width(Size::Fill)
											
											.span(Span::new(format!("{} ", achievement.awardedCasual)).font_size(10.0))
											.span(Span::new(format!("({})", achievement.awardedCasual)).font_size(10.0).font_weight(FontWeight::BOLD))
											.span(Span::new(format!(" of {}", distinctPlayers)).font_size(10.0))
									)
									
									.child(
										paragraph()
											.text_align(TextAlign::Center)
											.width(Size::Fill)
											
											.span(Span::new(format!("{}% ", percentCasualString)).font_size(10.0))
											.span(Span::new(format!("({}%)", percentHardcoreString)).font_size(10.0).font_weight(FontWeight::BOLD))
											.span(Span::new(" unlock rate").font_size(10.0))
									)
							)
							
							.child(
								label()
									.font_size(10.0)
									.text_align(TextAlign::Center)
									.width(Size::Fill)
									.text(timestamp)
							)
					)
			)
		;
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
