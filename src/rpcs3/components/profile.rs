use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Button, ChildrenExt, ContainerSizeExt, ContainerWithContentExt, Content, Direction, IntoElement, ProgressBar, ProgressBarThemePartialExt, Size, TextAlign, TextStyleExt, label, rect, spawn, svg};
use freya::radio::use_radio;
use crate::constants::{RetroAchievementsProgressColorHardcore, TextColor};
use crate::data::AppData;
use crate::data::radio::AppDataChannel;
use crate::rpcs3::components::refresh::refreshUserData;
use crate::util::truncateF32;

pub fn Rpcs3ProfileElement() -> impl IntoElement
{
	let mut appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Rpcs3);
	
	let level = appData.read().user.rpcs3.level();
	let platinumCount = appData.read().user.rpcs3.platinumCount().to_string();
	let pointsToNextLevel = appData.read().user.rpcs3.pointsToNextLevel() as f32;
	let pointTotalForLevel = appData.read().user.rpcs3.pointTotalForLevel() as f32;
	
	let username = match appData.read().user.rpcs3.name.is_empty()
	{
		false => appData.read().user.rpcs3.formatAccountId(),
		true => appData.read().user.rpcs3.name.clone(),
	};
	
	let percent = truncateF32(
		(pointsToNextLevel / pointTotalForLevel) * 100.0,
		2
	);
	
	return rect()
		.direction(Direction::Horizontal)
		.main_align(Alignment::SpaceBetween)
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
							let refreshedData = refreshUserData(appData.read().clone()).await;
							appData.write().user.rpcs3 = refreshedData.user.rpcs3;
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
