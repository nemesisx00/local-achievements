use data::constants::{BorderColor, RetroAchievementsProgressColorHardcore, TextColor};
use data::enums::GamePlatforms;
use data::format::truncateF32;
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment, BorderWidth, Button, ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, Content, CornerRadius, Direction, Gaps, IntoElement, ProgressBar, ProgressBarThemePartialExt, Size, StyleExt, TextAlign, TextStyleExt, label, rect, spawn, svg};
use freya::radio::use_radio;
use crate::components::refresh::refreshUserData;
use crate::data::settings::Rpcs3Settings;
use crate::data::user::Rpcs3User;

pub fn Rpcs3ProfileElement() -> impl IntoElement
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
		.corner_radius(CornerRadius::new_all(10.0))
		.direction(Direction::Horizontal)
		.main_align(Alignment::SpaceBetween)
		.margin(Gaps::new_symmetric(0.0, 1.0))
		.padding(Gaps::new_symmetric(10.0, 10.0))
		.spacing(25.0)
		.width(Size::Fill)
		
		.child(
			rect()
				.direction(Direction::Vertical)
				.main_align(Alignment::SpaceAround)
				.spacing(10.0)
				.width(Size::percent(40.0))
				
				.child(
					label()
						.text_align(TextAlign::Center)
						.width(Size::Fill)
						.text(username)
				)
				
				.child(
					Button::new()
						.on_press(move |_| _ = spawn(async move {
							let refreshedData = refreshUserData(
								user.read().clone(),
								settings.read().clone()
							).await;
							**user.write() = refreshedData;
						}))
						
						.child(
							svg(lucide::refresh_ccw())
								.color(TextColor)
								.height(Size::px(32.0))
								.width(Size::px(32.0))
								.a11y_alt("Refresh")
						)
				)
		)
		
		.child(
			rect()
				.direction(Direction::Vertical)
				.main_align(Alignment::SpaceAround)
				.spacing(10.0)
				.width(Size::percent(55.0))
				
				.child(
					rect()
						.content(Content::Flex)
						.cross_align(Alignment::Center)
						.direction(Direction::Horizontal)
						.main_align(Alignment::Start)
						.spacing(10.0)
						.width(Size::Fill)
						
						.child(
							label()
								.text(format!("Level {}", level))
						)
						
						.child(
							ProgressBar::new(percent)
								//Match the background color until more properties are exposed to customize the text
								.color(RetroAchievementsProgressColorHardcore)
								.progress_background(RetroAchievementsProgressColorHardcore)
								.width(Size::Fill)
						)
				)
				
				.child(
					rect()
						.content(Content::Flex)
						.direction(Direction::Horizontal)
						.main_align(Alignment::Start)
						.width(Size::flex(1.0))
						
						.child(
							label()
								.text_align(TextAlign::Start)
								.width(Size::flex(1.5))
								.text("Platinums")
						)
						
						.child(
							label()
								.text_align(TextAlign::End)
								.width(Size::flex(0.5))
								.text(platinumCount)
						)
				)
		);
}
