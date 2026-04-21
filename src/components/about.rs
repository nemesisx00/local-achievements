use components::button::icon::IconButton;
use components::extensions::PressableExt;
use data::constants::{BorderColor, LinkBlue, OverlayBackgroundColor, OverlayGreyoutColor, TextColor};
use data::enums::DataChannel;
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, CornerRadius, Direction, Gaps, IntoElement, Layer,
	LayerExt, Position, Size, StyleExt, TextAlign, TextDecoration, TextStyleExt,
	WritableUtils, label, rect, use_side_effect, use_state};
use freya::radio::{Writable, use_radio};
use freya::winit::dpi::PhysicalSize;

#[derive(Clone, PartialEq)]
pub struct About
{
	close: Writable<bool>,
}

impl Component for About
{
	fn render(&self) -> impl IntoElement
	{
		let windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let mut closeState = use_state(bool::default);
		
		use_side_effect({
			let mut close = self.close.clone();
			move || *close.write() = closeState()
		});
		
		let overlayWidth = windowSize.read().width / 2;
		let overlayHeight = windowSize.read().height / 2;
		
		let left = (windowSize.read().width / 2) - (overlayWidth / 2);
		let top = (windowSize.read().height / 2) - (overlayHeight / 2);
		
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
					.direction(Direction::Vertical)
					.height(Size::px(overlayHeight as f32))
					.padding(Gaps::new_symmetric(10.0, 25.0))
					.spacing(15.0)
					.width(Size::px(overlayWidth as f32))
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.margin(Gaps::new(0.0, 0.0, 10.0, 0.0))
							
							.child(
								label()
									.font_size(32.0)
									.text("About Reliquarian")
									.text_align(TextAlign::Center)
									.width(Size::percent(100.0))
							)
							
							.child(
								IconButton::new(lucide::x())
									.alt("Close")
									.color(TextColor)
									.height(Size::px(32.0))
									.innerHeight(Size::px(32.0))
									.innerWidth(Size::px(32.0))
									.position(Position::new_absolute()
										.top(0.0)
										.right(-15.0)
									)
									.width(Size::px(32.0))
									.onPress(move |_| closeState.set(true))
							)
					)
					
					.child(
						label()
							.text("Reliquarian is a free, open source desktop application for viewing, tracking, and backing up achievements data.")
							.width(Size::percent(100.0))
					)
					
					.child(
						label()
							.text("License: GPL-3.0")
							.width(Size::percent(100.0))
					)
					
					.child(
						rect()
							.direction(Direction::Horizontal)
							.spacing(10.0)
							.width(Size::percent(100.0))
							
							.child("Source:")
							
							.child(
								label()
									.a11y_alt("https://github.com/nemesisx00/reliquarian")
									.color(LinkBlue)
									.text("https://github.com/nemesisx00/reliquarian")
									.text_decoration(TextDecoration::Underline)
									.pressable(move |_| _ = webbrowser::open("https://github.com/nemesisx00/reliquarian"))
							)
					)
			);
	}
}

impl About
{
	pub fn new(close: impl Into<Writable<bool>>) -> Self
	{
		return Self
		{
			close: close.into(),
		};
	}
}
