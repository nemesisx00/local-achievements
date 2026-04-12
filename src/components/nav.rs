use std::path::PathBuf;
use components::button::icon::IconButton;
use data::constants::{BackgroundColor, BorderColor, TextColor};
use data::enums::{ActiveContent, DataChannel};
use data::settings::AppSettings;
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	BorderWidth, Button, ButtonLayoutThemePartialExt, ChildrenExt,
	CircularLoader, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Element, Event, EventHandler, Gaps, ImageViewer, IntoElement,
	Layer, LayerExt, Position, PressEventData, Size, StyleExt, TextAlign,
	TextStyleExt, WritableUtils, label, rect, use_side_effect, use_state};
use freya::radio::use_radio;
use net::RequestEvent;
use crate::components::ProfileState;
use crate::components::profile::ProfileElement;

pub fn NavBar() -> impl IntoElement
{
	let mut activeContent = use_radio::<Option<ActiveContent>, DataChannel>(DataChannel::ActiveContent);
	let appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
	let mut profileState = use_radio::<ProfileState, DataChannel>(DataChannel::ProfileState);
	let requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut selected = use_state(|| match activeContent.read().clone()
	{
		None => appSettings.read().defaultActivePlatform,
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
	
	let profileIcon = match profileState.read().clone() == ProfileState::Hiding
		|| profileState.read().clone() == ProfileState::Shown
	{
		false => lucide::chevron_right(),
		true => lucide::chevron_left(),
	};
	
	use_side_effect(move || **activeContent.write() = Some(selected()));
	
	return rect()
		.background(BackgroundColor)
		.direction(Direction::Horizontal)
		.height(Size::Fill)
		.layer(Layer::Overlay)
		.margin(Gaps::new(0.0, 0.0, 0.0, 7.5))
		.position(Position::new_absolute()
			.left(0.0)
			.top(0.0)
		)
		.width(Size::px(60.0))
		
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
				.padding(Gaps::new(10.0, 5.0, 5.0, 0.0))
				.spacing(5.0)
				.width(Size::FillMinimum)
				
				// Open/close button
				.child(
					IconButton::new(profileIcon)
						.alt("Show Profiles")
						.color(TextColor)
						.height(Size::px(48.0))
						.innerHeight(Size::px(32.0))
						.innerWidth(Size::px(32.0))
						.width(Size::px(48.0))
						.onPress(move |_| {
							let mut profileState = profileState.write();
							match profileState.clone()
							{
								ProfileState::Hidden => **profileState = ProfileState::Shown,
								ProfileState::Shown => **profileState = ProfileState::Hidden,
								_ => {}
							}
						})
				)
				
				.child(
					IconButton::new(lucide::settings())
						.alt("Settings")
						.color(TextColor)
						.height(Size::px(48.0))
						.innerHeight(Size::px(32.0))
						.innerWidth(Size::px(32.0))
						.width(Size::px(48.0))
						.onPress(move |_| selected.set(ActiveContent::Settings))
				)
				
				.child(
					navButton(
						"assets/battlenet-logo.png",
						"Battle.Net",
						move |_| selected.set(ActiveContent::BattleNet)
					)
				)
				
				.child(
					navButton(
						"assets/egs-logo.png",
						"Epic Games Store",
						move |_| selected.set(ActiveContent::EpicGamesStore)
					)
				)
				
				.child(
					navButton(
						"assets/gog-logo.png",
						"GOG",
						move |_| selected.set(ActiveContent::Gog)
					)
				)
				
				.child(
					Button::new()
						.height(Size::px(48.0))
						.margin(Gaps::new_all(0.0))
						.width(Size::px(48.0))
						.child(
							ImageViewer::new(PathBuf::from("assets/ra-logo.png"))
								.a11y_alt("Retro Achievements")
								.width(Size::px(32.0))
						)
						.on_press(move |_| selected.set(ActiveContent::RetroAchievements))
				)
				
				.child(
					navButton(
						"assets/rpcs3-logo.png",
						"RPCS3",
						move |_| selected.set(ActiveContent::Rpcs3)
					)
				)
				
				.child(
					navButton(
						"assets/steam-logo.png",
						"Steam",
						move |_| selected.set(ActiveContent::Steam)
					)
				)
				
				.maybe_child(taskCounter)
		)
		
		.child(ProfileElement::new());
}

fn navButton(
	path: &'static str,
	alt: &'static str,
	pressHandler: impl Into<EventHandler<Event<PressEventData>>>
) -> impl IntoElement
{
	return Button::new()
		.height(Size::px(48.0))
		.margin(Gaps::new_all(0.0))
		.width(Size::px(48.0))
		.child(
			ImageViewer::new(PathBuf::from(path))
				.a11y_alt(alt)
				.height(Size::px(32.0))
				.width(Size::px(32.0))
		)
		.on_press(pressHandler);
}
