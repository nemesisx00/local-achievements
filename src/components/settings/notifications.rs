use components::input::number::NumericInput;
use data::enums::DataChannel;
use data::settings::AppSettings;
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps,
	IntoElement, Size, TextAlign, TextStyleExt, label, rect, use_side_effect,
	use_state};
use freya::radio::use_radio;

#[derive(Clone, PartialEq)]
pub struct NotificationSettings
{
	labelWidth: Size,
}

impl Component for NotificationSettings
{
	fn render(&self) -> impl IntoElement
	{
		let mut appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
		
		let duration = use_state(|| appSettings.read().notificationDuration);
		
		use_side_effect(move || appSettings.write().notificationDuration = duration());
		
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
					.text("Notifications")
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
							.width(labelWidth)
							.text("Duration")
					)
					
					.child(
						NumericInput::new(duration)
							.max(5000u64)
							.placeholder("1000")
							.width(Size::flex(1.0))
					)
			);
	}
}

impl NotificationSettings
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
