use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps, Input,
	IntoElement, Size, TextAlign, TextStyleExt, label, rect, use_side_effect,
	use_state};
use freya::radio::use_radio;
use crate::components::NumericInput;
use crate::data::AppData;
use crate::data::radio::AppDataChannel;

#[derive(Clone, PartialEq)]
pub struct Rpcs3SettingsElement
{
	labelWidth: Size,
}

impl Component for Rpcs3SettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let mut appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		
		let accountId = use_state(|| appData.read().platform.rpcs3.accountId);
		let appDataDir = use_state(|| appData.read().platform.rpcs3.appDataDirectory.clone());
		
		use_side_effect(move || {
			appData.write().platform.rpcs3.accountId = accountId();
			appData.write().platform.rpcs3.appDataDirectory = appDataDir.read().clone();
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
					.text("RPCS3")
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
							.text("Account ID")
					)
					
					.child(
						NumericInput::new(accountId)
							.width(Size::flex(1.0))
					)
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
							.text("App Data Directory")
					)
					
					.child(
						Input::new(appDataDir)
							.placeholder("RPCS3 Application Data Directory")
							.width(Size::flex(1.0))
					)
			)
		;
	}
}

impl Rpcs3SettingsElement
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
