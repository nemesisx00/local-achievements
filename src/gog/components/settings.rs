use freya::prelude::{Alignment, Button, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, Gaps,
	IntoElement, Size, TextAlign, TextStyleExt, label, rect};
use tracing::{info, warn};
use crate::data::secure::removeGogSession;

#[derive(Clone, PartialEq)]
pub struct GogSettingsElement
{
	labelWidth: Size,
}

impl Component for GogSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let _labelWidth = self.labelWidth.clone();
		
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
					.text("GOG Session Management")
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Center)
					.spacing(10.0)
					.width(Size::percent(75.0))
					
					.child(
						Button::new()
							.on_press(move |_| match removeGogSession()
							{
								Err(e) => warn!("[GOG] Error removing session data: {:?}", e),
								Ok(_) => info!("[GOG] Session data removed"),
							})
							.child("Clear Session")
					)
			);
	}
}

impl GogSettingsElement
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
