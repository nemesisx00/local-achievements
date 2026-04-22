use std::path::PathBuf;
use components::button::icon::IconButton;
use components::overlay::refresh::ConfirmRefresh;
use data::constants::{BorderColor, CornerRadius, Path_Avatars,
	RetroAchievementsGameBeaten, RetroAchievementsGameBeatenCasual,
	RetroAchievementsGameComplete, RetroAchievementsGameMastered,
	RetroAchievementsGameUnfinished};
use data::enums::{DataChannel, GamePlatforms};
use data::io::{FileLocation, filePathExists, getImagePath};
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	BorderWidth, ChildrenExt, Color, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, Gaps, ImageViewer, IntoElement,
	Size, Span, StyleExt, TextStyleExt, WritableUtils, label, paragraph, rect,
	spawn, svg, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use macros::png;
use net::{RateLimiter, RequestEvent};
use crate::api::RetroAchievementsApi;
use crate::data::kind::AwardKind;
use crate::data::operation::RetroAchievementsOperation;
use crate::data::user::RetroAchievementsUser;

#[derive(Clone, PartialEq)]
pub struct RetroAchievementsUserProfile;

impl Component for RetroAchievementsUserProfile
{
	fn render(&self) -> impl IntoElement
	{
		let user = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		
		let mut cancelled = use_state(bool::default);
		let mut confirmed = use_state(bool::default);
		let mut showConfirmationDialog = use_state(bool::default);
		
		let ulid = user.read().ulid.clone();
		let username = user.read().username.clone();
		let points = user.read().points();
		let retroPoints = user.read().retroPoints;
		
		let beatenCasual = user.read().countByAward(AwardKind::BeatenCasual);
		let beaten = user.read().countByAward(AwardKind::BeatenHardcore);
		let completed = user.read().countByAward(AwardKind::Completed);
		let mastered = user.read().countByAward(AwardKind::Mastered);
		
		let avatarPath = match ulid
		{
			None => None,
			Some(ulid) => getImagePath(&FileLocation
			{
				fileName: png!(ulid),
				group: Path_Avatars.into(),
				platform: RetroAchievementsApi::Platform.into(),
			}),
		};
		
		let bottomHeight = Size::px(16.0);
		let topHeight = Size::px(46.0);
		
		use_side_effect(move || {
			if (cancelled() || confirmed()) && showConfirmationDialog()
			{
				if confirmed()
				{
					spawn(async move {
						rateLimiter.read().pushAll(vec![
							RetroAchievementsOperation::GetUserProfile.into(),
							RetroAchievementsOperation::GetUserProgress(Default::default()).into(),
							//GetUserProgress is recursive; it automatically pushes a SaveToFile operation when it is finished
						]).await;
						
						**requestEvent.write() = RequestEvent::Added;
					});
				}
				
				cancelled.set(false);
				confirmed.set(false);
				showConfirmationDialog.set(false);
			}
		});
		
		return rect()
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(BorderWidth::from(1.0))
			))
			.corner_radius(CornerRadius)
			.content(Content::Flex)
			.cross_align(Alignment::Center)
			.direction(Direction::Horizontal)
			.min_height(Size::px(64.0))
			.main_align(Alignment::Start)
			.margin(Gaps::new_symmetric(0.0, 1.0))
			.padding(Gaps::new_all(10.0))
			.spacing(10.0)
			.width(Size::Fill)
			
			.maybe_child(filePathExists(&avatarPath).then(||
				ImageViewer::new(PathBuf::from(avatarPath.unwrap()))
					.corner_radius(CornerRadius)
					.height(Size::px(64.0))
			))
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Vertical)
					.height(Size::px(64.0))
					.main_align(Alignment::Center)
					.spacing(2.0)
					.width(Size::flex(1.0))
					
					.child(
						rect()
							.content(Content::Flex)
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.height(topHeight.clone())
							.spacing(5.0)
							.width(Size::flex(1.0))
							
							.child(
								rect()
									.content(Content::Flex)
									.direction(Direction::Vertical)
									.height(topHeight.clone())
									.main_align(Alignment::Center)
									.spacing(5.0)
									.width(Size::flex(1.0))
									
									.child(username)
									
									.child(
										paragraph()
											.font_size(12)
											.span(points.to_string())
											.span(
												Span::new(format!(" ({})", retroPoints))
													.color(Color::GREY)
											)
									)
							)
							
							.child(
								IconButton::new(lucide::refresh_ccw())
									.alt("Refresh")
									.height(Size::px(32.0))
									.innerHeight(Size::px(24.0))
									.innerWidth(Size::px(24.0))
									.width(Size::px(32.0))
									.onPress(move |_| showConfirmationDialog.set(true))
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.height(bottomHeight.clone())
							.spacing(5.0)
							.width(Size::flex(1.0))
							
							.maybe_child((beatenCasual > 0).then(||
								awardElement(AwardKind::BeatenCasual, bottomHeight.clone(), beatenCasual)
							))
							
							.maybe_child((beaten > 0).then(||
								awardElement(AwardKind::BeatenHardcore, bottomHeight.clone(), beaten)
							))
							
							.maybe_child((completed > 0).then(||
								awardElement(AwardKind::Completed, bottomHeight.clone(), completed)
							))
							
							.maybe_child((mastered > 0).then(||
								awardElement(AwardKind::Mastered, bottomHeight.clone(), mastered)
							))
					)
			)
			
			.maybe_child(showConfirmationDialog().then(||
				ConfirmRefresh::new(
					cancelled.into_writable(),
					confirmed.into_writable()
				)
			));
	}
}

impl RetroAchievementsUserProfile
{
	pub fn new() -> Self
	{
		return Self {};
	}
}

fn awardElement(award: AwardKind, height: impl Into<Size>, value: usize) -> impl IntoElement
{
	let height = height.into();
	
	let mut circle = svg(lucide::circle())
		.height(Size::px(12.0))
		.width(Size::px(12.0));
	
	circle = match award
	{
		AwardKind::BeatenCasual => circle
			.a11y_alt("Mastered")
			.color(RetroAchievementsGameBeatenCasual)
			.fill(RetroAchievementsGameUnfinished),
		
		AwardKind::BeatenHardcore => circle
			.a11y_alt("Mastered")
			.color(RetroAchievementsGameBeaten)
			.fill(RetroAchievementsGameBeaten),
		
		AwardKind::Completed => circle
			.a11y_alt("Completed")
			.color(RetroAchievementsGameComplete)
			.fill(RetroAchievementsGameUnfinished),
		
		AwardKind::Mastered => circle
			.a11y_alt("Mastered")
			.color(RetroAchievementsGameMastered)
			.fill(RetroAchievementsGameMastered),
	};
	
	return rect()
		.cross_align(Alignment::Center)
		.direction(Direction::Horizontal)
		.height(height)
		.spacing(2.0)
		
		.child(circle)
		
		.child(
			label()
				.font_size(12)
				.text(value.to_string())
		);
}
