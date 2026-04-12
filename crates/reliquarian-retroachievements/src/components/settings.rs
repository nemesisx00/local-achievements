use components::settings::switch::SettingsSwitch;
use data::constants::InputModeHiddenChar;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, IntoElement, Size, TextAlign, TextStyleExt, WritableUtils, label,
	rect, use_hook, use_side_effect, use_state};
use freya::radio::IntoWritable;
use crate::secure::{getRetroAchievementsAuth, setRetroAchievementsApiKey,
	setRetroAchievementsUsername};

#[derive(Clone, PartialEq)]
pub struct RetroAchievementsSettingsElement
{
	labelWidth: Size,
}

impl Component for RetroAchievementsSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut apiKey = use_state(String::default);
		let mut username = use_state(String::default);
		let inputModeApiKey = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeUsername = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		
		use_side_effect(move || {
			_ = setRetroAchievementsApiKey(apiKey.read().clone());
			_ = setRetroAchievementsUsername(username.read().clone());
		});
		
		use_hook(|| {
			if let Ok(auth) = getRetroAchievementsAuth()
			{
				apiKey.set(auth.key().clone());
				username.set(auth.username().clone());
			}
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
					
					.child(SettingsSwitch(inputModeUsername.into_writable()))
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
					
					.child(SettingsSwitch(inputModeApiKey.into_writable()))
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
