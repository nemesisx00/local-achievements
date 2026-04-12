use data::constants::TextColor;
use freya::prelude::{AccessibilityExt, Button, ButtonLayoutThemePartialExt,
	Bytes, ChildrenExt, Color, Component, ContainerSizeExt, Event, EventHandler,
	IntoElement, PressEventData, Size, svg};

#[derive(Clone, PartialEq)]
pub struct IconButton
{
	alt: Option<String>,
	color: Color,
	height: Size,
	icon: Bytes,
	innerHeight: Size,
	innerWidth: Size,
	onPress: Option<EventHandler<Event<PressEventData>>>,
	width: Size,
}

impl Component for IconButton
{
	fn render(&self) -> impl IntoElement
	{
		let pressHandler = self.onPress.clone()
			.unwrap_or(EventHandler::new(move |_| {}));
		
		return Button::new()
			.on_press(pressHandler)
			.height(self.height.clone())
			.width(self.width.clone())
			
			.child(
				svg(self.icon.clone())
					.color(self.color.clone())
					.height(self.innerHeight.clone())
					.width(self.innerWidth.clone())
					.a11y_alt(self.alt.clone().unwrap_or_default())
			);
	}
}

#[allow(unused)]
impl IconButton
{
	pub fn new(icon: impl Into<Bytes>) -> Self
	{
		return Self
		{
			alt: Default::default(),
			color: TextColor,
			height: Size::px(64.0),
			icon: icon.into(),
			innerHeight: Size::px(32.0),
			innerWidth: Size::px(32.0),
			onPress: Default::default(),
			width: Size::px(64.0),
		};
	}
	
	pub fn alt(mut self, alt: impl Into<String>) -> Self
	{
		self.alt = Some(alt.into());
		return self;
	}
	
	pub fn color(mut self, color: impl Into<Color>) -> Self
	{
		self.color = color.into();
		return self;
	}
	
	pub fn height(mut self, size: impl Into<Size>) -> Self
	{
		self.height = size.into();
		return self;
	}
	
	pub fn innerHeight(mut self, size: impl Into<Size>) -> Self
	{
		self.innerHeight = size.into();
		return self;
	}
	
	pub fn innerWidth(mut self, size: impl Into<Size>) -> Self
	{
		self.innerWidth = size.into();
		return self;
	}
	
	pub fn onPress(mut self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self
	{
		self.onPress = Some(handler.into());
		return self;
	}
	
	pub fn width(mut self, size: impl Into<Size>) -> Self
	{
		self.width = size.into();
		return self;
	}
}
