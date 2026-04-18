use components::button::icon::IconButton;
use data::constants::{BorderColor, CornerRadius,
	RetroAchievementsProgressColorHardcore, TextColor};
use data::enums::GamePlatforms;
use data::format::truncateF32;
use freya::icons::lucide;
use freya::prelude::{Alignment, Border, BorderAlignment, BorderWidth,
	ChildrenExt, Color, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Content, Direction, Gaps, IntoElement, ProgressBar,
	ProgressBarThemePartialExt, Size, StyleExt, label, rect, spawn, svg};
use freya::radio::use_radio;
use crate::components::refresh::refreshUserData;
use crate::data::settings::Rpcs3Settings;
use crate::data::user::Rpcs3User;

#[derive(Clone, PartialEq)]
pub struct Rpcs3ProfileElement;

impl Component for Rpcs3ProfileElement
{
	fn render(&self) -> impl IntoElement
	{
		let settings = use_radio::<Rpcs3Settings, GamePlatforms>(GamePlatforms::Rpcs3);
		let mut user = use_radio::<Rpcs3User, GamePlatforms>(GamePlatforms::Rpcs3);
		
		let level = user.read().level();
		let platinumCount = user.read().platinumCount().to_string();
		let pointsToNextLevel = user.read().pointsToNextLevel() as f32;
		let pointTotalForLevel = user.read().pointTotalForLevel() as f32;
		
		let username = match user.read().name.is_empty()
		{
			false => user.read().formatAccountId(),
			true => user.read().name.clone(),
		};
		
		let percent = truncateF32(
			(pointsToNextLevel / pointTotalForLevel) * 100.0,
			2
		);
		
		return rect()
			.border(Some(
				Border::new()
					.alignment(BorderAlignment::Center)
					.fill(BorderColor)
					.width(BorderWidth::from(1.0))
			))
			.content(Content::Flex)
			.corner_radius(CornerRadius)
			.direction(Direction::Vertical)
			.main_align(Alignment::Center)
			.margin(Gaps::new_symmetric(0.0, 1.0))
			.height(Size::px(84.0))
			.padding(Gaps::new_all(10.0))
			.spacing(5.0)
			.width(Size::flex(1.0))
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.height(Size::flex(1.0))
					.main_align(Alignment::SpaceBetween)
					.width(Size::flex(1.0))
					
					.child(username)
					
					.child(
						IconButton::new(lucide::refresh_ccw())
							.alt("Refresh")
							.color(TextColor)
							.height(Size::px(32.0))
							.innerHeight(Size::px(24.0))
							.innerWidth(Size::px(24.0))
							.width(Size::px(32.0))
							.onPress(move |_| _ = spawn(async move {
								let refreshedData = refreshUserData(
									user.read().clone(),
									settings.read().clone()
								).await;
								**user.write() = refreshedData;
							}))
					)
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.height(Size::flex(1.0))
					.spacing(5.0)
					.width(Size::flex(1.0))
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.spacing(10.0)
							.width(Size::flex(1.5))
							
							.child(
								label()
									.text(format!("Level {}", level))
							)
							
							.child(
								ProgressBar::new(percent)
									//Match the background color until more properties are exposed to customize the text
									.color(RetroAchievementsProgressColorHardcore)
									.progress_background(RetroAchievementsProgressColorHardcore)
									.width(Size::flex(1.0))
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::End)
							.spacing(5.0)
							.width(Size::flex(0.5))
							
							.child(
								svg(lucide::trophy())
									.color(Color::from_rgb(229, 228, 226))
									.height(Size::px(16.0))
									.width(Size::px(16.0))
							)
							
							.child(format!("{}", platinumCount))
					)
			);
	}
}

impl Rpcs3ProfileElement
{
	pub fn new() -> Self
	{
		return Self {};
	}
}
