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
pub struct ConfirmRefresh
{
	cancelled: Writable<bool>,
	confirmed: Writable<bool>,
	
	/// Set the `cancelled` `Writable` to `false`` when Cancel is clicked rather than `true`.
	invertCancelled: bool,
}

impl Component for ConfirmRefresh
{
	fn render(&self) -> impl IntoElement
	{
		let windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let mut cancelledState = use_state(bool::default);
		let mut confirmedState = use_state(bool::default);
		
		use_side_effect({
			let mut cancelled = self.cancelled.clone();
			let mut confirmed = self.confirmed.clone();
			move || {
				*cancelled.write() = cancelledState();
				*confirmed.write() = confirmedState();
			}
		});
		
		let overlayWidth = windowSize.read().width / 2;
		let overlayHeight = windowSize.read().height / 2;
		
		let left = (windowSize.read().width / 2) - (overlayWidth / 2);
		let top = (windowSize.read().height / 2) - (overlayHeight / 2);
		
		let invertCancelled = self.invertCancelled;
		
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
							.text("Refresh Confirmation")
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
									.text("The application may become unresponsive while the refresh is in progress.")
							)
							
							.child(
								label()
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
									.text("Do you want to continue?")
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
									.on_press(move |_| confirmedState.set(true))
									.width(Size::flex(0.5))
									.child("Ok")
							)
							
							.child(
								Button::new()
									.on_press(move |_| cancelledState.set(!invertCancelled))
									.width(Size::flex(0.5))
									.child("Cancel")
							)
					)
			);
	}
}

#[allow(unused)]
impl ConfirmRefresh
{
	pub fn new(
		cancelled: Writable<bool>,
		confirmed: Writable<bool>
	) -> Self
	{
		return Self {
			cancelled,
			confirmed,
			invertCancelled: false,
		};
	}
	
	pub fn invertCancelled(mut self, invert: impl Into<bool>) -> Self
	{
		self.invertCancelled = invert.into();
		return self;
	}
}
