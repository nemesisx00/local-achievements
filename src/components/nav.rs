use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	BorderWidth, Button, ButtonLayoutThemePartialExt, ChildrenExt,
	CircularLoader, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Element, Gaps, IntoElement, Layer, LayerExt, Position, Size,
	StyleExt, TextAlign, TextStyleExt, label, rect, svg, use_side_effect,
	use_state};
use freya::radio::use_radio;
use crate::components::profile::{ProfileElement, ProfileState};
use crate::constants::{BackgroundColor, BorderColor, TextColor};
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::data::{ActiveContent, AppData};
use crate::net::limiter::request::RequestEvent;

pub fn NavBar() -> impl IntoElement
{
	let mut profileState = use_radio::<ProfileState, DataChannel>(DataChannel::ProfileState);
	let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
	let requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	let mut activeContent = use_radio::<Option<ActiveContent>, DataChannel>(DataChannel::ActiveContent);
	
	let mut selected = use_state(|| match activeContent.read().clone()
	{
		None => appData.read().app.settings.defaultActivePlatform,
		Some(ac) => ac,
	});
	
	let taskCounter: Option<Element> = match requestEvent.read().clone()
	{
		RequestEvent::Processing(count) => Some(
			rect()
				.cross_align(Alignment::Center)
				.direction(Direction::Vertical)
				.main_align(Alignment::End)
				.height(Size::Fill)
				.width(Size::percent(100.0))
				
				.child(
					CircularLoader::new()
						.size(32.0)
				)
				
				.child(
					label()
						.font_size(12.0)
						.text(format!("{} processing", count))
						.text_align(TextAlign::Center)
						.width(Size::percent(100.0))
				)
				.into()
		),
		
		_ => None,
	};
	
	use_side_effect(move || **activeContent.write() = Some(selected()));
	
	return rect()
		.background(BackgroundColor)
		.direction(Direction::Horizontal)
		.height(Size::Fill)
		.layer(Layer::Overlay)
		.margin(Gaps::new(0.0, 0.0, 0.0, 7.5))
		.max_width(Size::px(100.0))
		.min_width(Size::px(50.0))
		.position(Position::new_absolute()
			.left(0.0)
			.top(0.0)
		)
		.width(Size::FillMinimum)
		
		.child(
			rect()
				.border(Some(
					Border::new()
						.alignment(BorderAlignment::Center)
						.fill(BorderColor)
						.width(BorderWidth
						{
							bottom: 0.0,
							top: 0.0,
							right: 1.0,
							left: 0.0,
						})
				))
				.direction(Direction::Vertical)
				.height(Size::Fill)
				.padding(Gaps::new(10.0, 10.0, 5.0, 0.0))
				.spacing(10.0)
				.width(Size::FillMinimum)
				
				// Open/close button
				.child(
					Button::new()
						.width(Size::Fill)
						.on_press(move |_| {
							let mut profileState = profileState.write();
							match profileState.clone()
							{
								ProfileState::Hidden => **profileState = ProfileState::Shown,
								ProfileState::Shown => **profileState = ProfileState::Hidden,
								_ => {}
							}
						})
						
						.maybe_child((
								profileState.read().clone() == ProfileState::Hiding
								|| profileState.read().clone() == ProfileState::Shown
							)
							.then(||
								svg(lucide::chevron_left())
									.color(TextColor)
									.height(Size::px(32.0))
									.width(Size::px(32.0))
							))
						
						.maybe_child((
								profileState.read().clone() == ProfileState::Hidden
								|| profileState.read().clone() == ProfileState::Showing
							)
							.then(||
								svg(lucide::chevron_right())
									.color(TextColor)
									.height(Size::px(32.0))
									.width(Size::px(32.0))
							))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.on_press(move |_| selected.set(ActiveContent::Settings))
						.child(
							svg(lucide::settings())
								.color(TextColor)
								.height(Size::px(32.0))
								.width(Size::px(32.0))
								.a11y_alt("Settings")
						)
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("BNet")
						.on_press(move |_| selected.set(ActiveContent::BattleNet))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("EGS")
						.on_press(move |_| selected.set(ActiveContent::EpicGamesStore))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("GOG")
						.on_press(move |_| selected.set(ActiveContent::Gog))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("RA")
						.on_press(move |_| selected.set(ActiveContent::RetroAchievements))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("RPCS3")
						.on_press(move |_| selected.set(ActiveContent::Rpcs3))
				)
				
				.child(
					Button::new()
						.width(Size::percent(100.0))
						.child("Steam")
						.on_press(move |_| selected.set(ActiveContent::Steam))
				)
				
				.maybe_child(taskCounter)
		)
		
		.child(ProfileElement::new());
}
