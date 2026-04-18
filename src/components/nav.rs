use std::path::PathBuf;
use components::button::icon::IconButton;
use data::constants::{BackgroundColor, BorderColor, TextColor};
use data::enums::{ActiveContent, DataChannel};
use data::settings::AppSettings;
use freya::icons::lucide;
use freya::prelude::{AccessibilityExt, Alignment, Border, BorderAlignment,
	BorderWidth, Button, ButtonLayoutThemePartialExt, ChildrenExt,
	CircularLoader, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
	Direction, Event, EventHandler, Gaps, ImageViewer, IntoElement, Layer,
	LayerExt, Position, PressEventData, Size, StyleExt, TextAlign, TextStyleExt,
	WritableUtils, label, rect, use_side_effect, use_state};
use freya::radio::{IntoWritable, Writable, use_radio};
use net::RequestEvent;
use super::ProfileState;
use super::about::About;

pub fn NavBar() -> impl IntoElement
{
	let mut activeContent = use_radio::<Option<ActiveContent>, DataChannel>(DataChannel::ActiveContent);
	let appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
	let mut profileState = use_radio::<ProfileState, DataChannel>(DataChannel::ProfileState);
	let requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
	
	let mut closeAbout = use_state(|| false);
	let mut selected = use_state(|| match activeContent.read().clone()
	{
		None => appSettings.read().defaultActivePlatform,
		Some(ac) => ac,
	});
	let mut showAbout = use_state(|| false);
	
	let aboutOverlay = match showAbout()
	{
		false => None,
		true => Some(About::new(closeAbout.into_writable()))
	};
	
	let count = match requestEvent.read().clone()
	{
		RequestEvent::Processing(count) => Some(count),
		_ => None,
	};
	
	let profileIcon = match profileState.read().clone() == ProfileState::Hiding
		|| profileState.read().clone() == ProfileState::Shown
	{
		false => lucide::chevron_right(),
		true => lucide::chevron_left(),
	};
	
	let bnet = match appSettings.read().enabledPlatforms.battleNet
	{
		false => None,
		true => Some(navButton(
			"assets/battlenet-logo.png",
			"Battle.Net",
			move |_| selected.set(ActiveContent::BattleNet)
		))
	};
	
	let egs = match appSettings.read().enabledPlatforms.epicGamesStores
	{
		false => None,
		true => Some(navButton(
			"assets/egs-logo.png",
			"Epic Games Store",
			move |_| selected.set(ActiveContent::EpicGamesStore)
		))
	};
	
	let gog = match appSettings.read().enabledPlatforms.gog
	{
		false => None,
		true => Some(navButton(
			"assets/gog-logo.png",
			"GOG",
			move |_| selected.set(ActiveContent::Gog)
		))
	};
	
	let ra = match appSettings.read().enabledPlatforms.retroAchievements
	{
		false => None,
		true => Some(Button::new()
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
	};
	
	let rpcs3 = match appSettings.read().enabledPlatforms.rpcs3
	{
		false => None,
		true => Some(navButton(
			"assets/rpcs3-logo.png",
			"RPCS3",
			move |_| selected.set(ActiveContent::Rpcs3)
		))
	};
	
	let steam = match appSettings.read().enabledPlatforms.steam
	{
		false => None,
		true => Some(navButton(
			"assets/steam-logo.png",
			"Steam",
			move |_| selected.set(ActiveContent::Steam)
		))
	};
	
	use_side_effect(move || {
		**activeContent.write() = Some(selected());
		
		if showAbout() && closeAbout()
		{
			closeAbout.set(false);
			showAbout.set(false);
		}
	});
	
	return rect()
		.background(BackgroundColor)
		.direction(Direction::Horizontal)
		.height(Size::Fill)
		.layer(Layer::Overlay)
		.position(Position::new_absolute()
			.left(0.0)
			.top(0.0)
		)
		.width(Size::px(64.0))
		
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
				.padding(Gaps::new(10.0, 7.5, 5.0, 7.5))
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
							let value = profileState.read().cloned();
							match value
							{
								ProfileState::Hidden => **profileState.write() = ProfileState::Showing,
								ProfileState::Shown => **profileState.write() = ProfileState::Hiding,
								_ => {}
							}
						})
				)
				
				.maybe_child(bnet)
				.maybe_child(egs)
				.maybe_child(gog)
				.maybe_child(ra)
				.maybe_child(rpcs3)
				.maybe_child(steam)
				
				.child(navBottom(
					showAbout.into_writable(),
					count,
					selected.into_writable()
				))
		)
		
		.maybe_child(aboutOverlay);
}

fn navBottom(show: impl Into<Writable<bool>>, count: Option<impl Into<usize>>, selected: impl Into<Writable<ActiveContent>>) -> impl IntoElement
{
	let count = match count
	{
		None => None,
		Some(count) => Some(count.into()),
	};
	let mut show = show.into();
	let mut selected = selected.into();
	
	let loader = match count
	{
		None => None,
		Some(_) => Some(
			CircularLoader::new()
				.size(32.0)
		)
	};
	
	let taskCounter = match count
	{
		None => None,
		Some(count) => Some(
			label()
				.font_size(12.0)
				.text(count.to_string())
				.text_align(TextAlign::Center)
				.width(Size::percent(100.0))
		)
	};
	
	return rect()
		.cross_align(Alignment::Center)
		.direction(Direction::Vertical)
		.main_align(Alignment::End)
		.height(Size::Fill)
		.spacing(5.0)
		.width(Size::percent(100.0))
		
		.maybe_child(loader)
		.maybe_child(taskCounter)
		
		.child(
			IconButton::new(lucide::circle_question_mark())
				.alt("About Reliquarian")
				.color(TextColor)
				.height(Size::px(48.0))
				.width(Size::px(48.0))
				.onPress(move |_| show.set(true))
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
		);
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
