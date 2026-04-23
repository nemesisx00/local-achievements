use data::enums::{ActiveContent, DataChannel};
use data::io::saveAppSettings;
use data::settings::{AppSettings, Language};
use freya::prelude::{Alignment, ChildrenExt, Component, ContainerExt,
	ContainerSizeExt, ContainerWithContentExt, Content, Direction, FontWeight,
	Gaps, IntoElement, MenuItem, Select, Size, Switch, TextAlign, TextStyleExt,
	WritableUtils, label, rect, spawn, use_side_effect, use_state};
use freya::radio::{IntoWritable, use_radio};
use strum::IntoEnumIterator;
use tracing::{info, warn};
use super::toggles::PlatformToggles;

#[derive(Clone, PartialEq)]
pub struct UiSettings
{
	labelWidth: Size,
}

impl Component for UiSettings
{
	fn render(&self) -> impl IntoElement
	{
		let mut appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
		
		let mut defaultActiveContent = use_state(|| appSettings.read().defaultActivePlatform);
		let mut displayGamesWithoutAchievements = use_state(|| appSettings.read().displayGamesWithoutAchievements);
		let enabledBNet = use_state(|| appSettings.read().enabledPlatforms.battleNet);
		let enabledEgs = use_state(|| appSettings.read().enabledPlatforms.epicGamesStores);
		let enabledGog = use_state(|| appSettings.read().enabledPlatforms.gog);
		let enabledRA = use_state(|| appSettings.read().enabledPlatforms.retroAchievements);
		let enabledRpcs3 = use_state(|| appSettings.read().enabledPlatforms.rpcs3);
		let enabledSteam = use_state(|| appSettings.read().enabledPlatforms.steam);
		let mut language = use_state(|| appSettings.read().language.clone());
		
		use_side_effect(move || {
			appSettings.write().defaultActivePlatform = defaultActiveContent.read().clone();
			appSettings.write().displayGamesWithoutAchievements = displayGamesWithoutAchievements();
			appSettings.write().enabledPlatforms.battleNet = enabledBNet();
			appSettings.write().enabledPlatforms.epicGamesStores = enabledEgs();
			appSettings.write().enabledPlatforms.gog = enabledGog();
			appSettings.write().enabledPlatforms.retroAchievements = enabledRA();
			appSettings.write().enabledPlatforms.rpcs3 = enabledRpcs3();
			appSettings.write().enabledPlatforms.steam = enabledSteam();
			appSettings.write().language = language.read().clone();
			
			spawn(async move {
				match saveAppSettings(&appSettings.read())
				{
					Err(e) => warn!("[Reliquarian] Error saving app settings: {:?}", e),
					Ok(_) => info!("[Reliquarian] Saved app settings"),
				}
			});
		});
		
		return rect()
			.cross_align(Alignment::Center)
			.direction(Direction::Vertical)
			.margin(Gaps::new_all(10.0))
			.spacing(5.0)
			.width(Size::Fill)
			
			.child(
				label()
					.font_size(24.0)
					.font_weight(FontWeight::BOLD)
					.text("Application")
					.text_align(TextAlign::Center)
					.width(Size::percent(75.0))
			)
			
			.child(
				rect()
					.content(Content::Flex)
					.cross_align(Alignment::Center)
					.direction(Direction::Horizontal)
					.main_align(Alignment::Start)
					.width(Size::percent(75.0))
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child("Language")
							
							.child(
								Select::new()
									.selected_item(
										match Language::iter()
											.find(|lang| lang == &language())
										{
											None => Language::default().to_string(),
											Some(lang) => lang.to_string(),
										}
									)
									
									.children(
										Language::iter().map(|lang| {
											MenuItem::new()
												.selected(lang == language())
												.on_press(move |_| language.set(lang))
												.child(lang.to_string())
												.into()
										})
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child("Starting Tab")
							
							.child(
								Select::new()
									.selected_item(
										match ActiveContent::iter()
											.find(|ac| ac == &defaultActiveContent())
										{
											None => ActiveContent::default().to_string(),
											Some(ac) => ac.to_string(),
										}
									)
									
									.children(
										ActiveContent::iter().map(|ac| {
											MenuItem::new()
												.selected(ac == defaultActiveContent())
												.on_press(move |_| defaultActiveContent.set(ac))
												.child(ac.to_string())
												.into()
										})
									)
							)
					)
					
					.child(
						rect()
							.cross_align(Alignment::Center)
							.direction(Direction::Horizontal)
							.main_align(Alignment::Center)
							.spacing(10.0)
							.width(Size::flex(1.0))
							
							.child("Show All Games")
							
							.child(
								Switch::new()
									.toggled(*displayGamesWithoutAchievements.read())
									.on_toggle(move |_| {
										let value = *displayGamesWithoutAchievements.read();
										*displayGamesWithoutAchievements.write() = !value;
									})
							)
					)
			)
			
			.child(PlatformToggles::new(
				enabledBNet.into_writable(),
				enabledEgs.into_writable(),
				enabledGog.into_writable(),
				enabledRA.into_writable(),
				enabledRpcs3.into_writable(),
				enabledSteam.into_writable()
			));
	}
}

impl UiSettings
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
