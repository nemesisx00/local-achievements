use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, IntoElement, Size, TextAlign, TextStyleExt, label, rect,
	use_side_effect, use_state};
use freya::radio::use_radio;
use crate::components::{InputModeHiddenChar, SettingsSwitch};
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

#[derive(Clone, PartialEq)]
pub struct RetroAchievementsSettingsElement
{
	labelWidth: Size,
}

impl Component for RetroAchievementsSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		
		let apiKey = use_state(|| appData.read().platform.retroAchievements.key.clone());
		let username = use_state(|| appData.read().platform.retroAchievements.username.clone());
		let inputModeApiKey = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeUsername = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		
		use_side_effect(move || {
			appData.write().platform.retroAchievements.key = apiKey.read().clone();
			appData.write().platform.retroAchievements.username = username.read().clone();
		});
		
		let labelWidth = self.labelWidth.clone();
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new_all(10.0))
			.spacing(5.0)
			.width(Size::Fill)
			
			.child(
				label()
					.margin(Gaps::new(0.0, 0.0, 5.0, 0.0))
					.text_align(TextAlign::Center)
					.width(Size::Fill)
					.text("RetroAchievements Web API Authentication")
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(7.0, 0.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(labelWidth.clone())
							.text("Username")
					)
					
					.child(
						Input::new(username)
							.mode(inputModeUsername.read().cloned())
							.placeholder("RetroAchievements Username")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(SettingsSwitch(inputModeUsername))
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						label()
							.margin(Gaps::new(7.0, 0.0, 0.0, 0.0))
							.min_width(Size::px(102.0))
							.text_align(TextAlign::End)
							.width(labelWidth.clone())
							.text("Web API Key")
					)
					
					.child(
						Input::new(apiKey)
							.mode(inputModeApiKey.read().cloned())
							.placeholder("RetroAchievements Web API Key")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(SettingsSwitch(inputModeApiKey))
			);
	}
}

impl RetroAchievementsSettingsElement
{
	pub fn new() -> Self
	{
		return Self
		{
			labelWidth: Size::percent(20.0),
		};
	}
	
	#[allow(unused)]
	pub fn labelWidth(mut self, width: impl Into<Size>) -> Self
	{
		self.labelWidth = width.into();
		return self;
	}
}
