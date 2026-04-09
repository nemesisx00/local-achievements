use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	InputMode, IntoElement, Size, TextAlign, TextStyleExt, WritableUtils, label,
	rect, use_hook, use_side_effect, use_state};
use crate::components::{InputModeHiddenChar, SettingsSwitch};
use crate::data::secure::{getSteamAuth, setSteamApiKey, setSteamId};

#[derive(Clone, PartialEq)]
pub struct SteamSettingsElement
{
	labelWidth: Size,
}

impl Component for SteamSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut apiKey = use_state(String::default);
		let mut id = use_state(String::default);
		
		let inputModeApiKey = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		let inputModeId = use_state(|| InputMode::Hidden(InputModeHiddenChar));
		
		use_hook(|| {
			if let Ok(auth) = getSteamAuth()
			{
				apiKey.set(auth.key().clone());
				id.set(auth.id().clone());
			}
		});
		
		use_side_effect(move || {
			_ = setSteamApiKey(apiKey.read().clone());
			_ = setSteamId(id.read().clone());
		});
		
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
					.text("Steam Web API Authentication")
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
							.width(self.labelWidth.clone())
							.text("Steam ID")
					)
					
					.child(
						Input::new(id)
							.mode(inputModeId.read().clone())
							.placeholder("Steam ID")
							.width(Size::flex(1.0))
					)
					
					.child(
						label()
							.margin(Gaps::new(5.0, 0.0, 0.0, 0.0))
							.text_align(TextAlign::End)
							.width(Size::FillMinimum)
							.text("Show")
					)
					
					.child(SettingsSwitch(inputModeId))
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
							.width(self.labelWidth.clone())
							.text("Steam API Key")
					)
					
					.child(
						Input::new(apiKey)
							.mode(inputModeApiKey.read().clone())
							.placeholder("Steam Web API Key")
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
			)
	}
}

impl SteamSettingsElement
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
