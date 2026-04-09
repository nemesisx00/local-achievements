use freya::prelude::{Alignment, Border, BorderAlignment, Button,
	ButtonLayoutThemePartialExt, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, CornerRadius, Direction,
	FontWeight, Gaps, IntoElement, Layer, LayerExt, Position, Size, StyleExt,
	TextAlign, TextStyleExt, WritableUtils, label, rect, use_side_effect,
	use_state};
use freya::radio::{Writable, use_radio};
use freya::winit::dpi::PhysicalSize;
use crate::constants::{BorderColor, OverlayBackgroundColor, OverlayGreyoutColor};
use crate::data::radio::DataChannel;

#[derive(Clone, PartialEq)]
pub struct OAuth2Overlay
{
	cancelled: Writable<bool>,
	done: Writable<bool>,
	platformName: Option<String>,
}

impl Component for OAuth2Overlay
{
	fn render(&self) -> impl IntoElement
	{
		let windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let mut cancelledState = use_state(|| self.cancelled.peek().clone());
		
		use_side_effect({
			let mut cancelled = self.cancelled.clone();
			move || {
				*cancelled.write() = cancelledState();
			}
		});
		
		let overlayWidth = windowSize.read().width / 2;
		let overlayHeight = windowSize.read().height / 2;
		
		let left = (windowSize.read().width / 2) - (overlayWidth / 2);
		let top = (windowSize.read().height / 2) - (overlayHeight / 2);
		
		let instructionText = match &self.platformName
		{
			None => "Use the opened browser tab to log in to your account".to_string(),
			Some(name) => format!("Use the opened browser tab to log in to your {} account", name),
		};
		
		// TODO: Figure out a way to reliably detect the "state" of the authorization process and update the UI accordingly
		let buttonText = "Done";
		/*
		let buttonText = match self.done.read().clone()
		{
			false => "Cancel",
			true => "Done",
		};
		*/
		
		return rect()
			.position(Position::new_global()
				.left(0.0)
				.top(0.0)
			)
			.background(OverlayGreyoutColor)  
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.height(Size::px(windowSize.read().height as f32))
			.layer(Layer::Overlay)
			.main_align(Alignment::SpaceEvenly)
			.width(Size::px(windowSize.read().width as f32))
			
			.child(
				rect()
					.position(Position::new_global()
						.left(left as f32)
						.top(top as f32)
					)
					.background(OverlayBackgroundColor)
					.border(Border::new()
						.alignment(BorderAlignment::Center)
						.fill(BorderColor)
						.width(1.0)
					)
					.corner_radius(CornerRadius::new_all(10.0))
					.center()
					.direction(Direction::Vertical)
					.height(Size::px(overlayHeight as f32))
					.padding(Gaps::new_symmetric(5.0, 20.0))
					.spacing(15.0)
					.width(Size::px(overlayWidth as f32))
					
					.child(
						label()
							.font_size(24.0)
							.font_weight(FontWeight::BOLD)
							.text_align(TextAlign::Center)
							.width(Size::percent(100.0))
							.text("OAuth2 Authorization Flow")
					)
					
					.child(
						rect()
							.direction(Direction::Vertical)
							.main_align(Alignment::SpaceAround)
							.width(Size::percent(100.0))
							.spacing(10.0)
							
							.child(
								label()
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									.text(instructionText)
							)
					)
					
					.child(
						rect()
							.content(Content::Flex)
							.direction(Direction::Horizontal)
							.main_align(Alignment::SpaceEvenly)
							.spacing(15.0)
							.width(Size::percent(100.0))
							
							.child(
								Button::new()
									.on_press(move |_| cancelledState.set(true))
									.width(Size::flex(0.5))
									.child(buttonText)
							)
					)
			);
	}
}

#[allow(unused)]
impl OAuth2Overlay
{
	pub fn new(
		cancelled: Writable<bool>,
		done: Writable<bool>,
	) -> Self
	{
		return Self {
			cancelled,
			done,
			platformName: None,
		};
	}
	
	pub fn platformName(mut self, name: impl Into<String>) -> Self
	{
		self.platformName = Some(name.into());
		return self;
	}
}
