use battlenet::components::settings::BattleNetSettingsElement;
use data::enums::DataChannel;
use data::settings::AppSettings;
use epicgamesstore::components::settings::EgsSettingsElement;
use freya::prelude::{ChildrenExt, Component, ContainerSizeExt,
	ContainerWithContentExt, Direction, IntoElement, ScrollView, Size, rect};
use freya::radio::use_radio;
use gog::components::settings::GogSettingsElement;
use retroachievements::components::settings::RetroAchievementsSettingsElement;
use rpcs3::components::settings::Rpcs3SettingsElement;
use steam::components::settings::SteamSettingsElement;
use crate::components::settings::local::LocalInfo;
use crate::components::settings::notifications::NotificationSettings;
use crate::components::settings::ui::UiSettings;

#[derive(Clone, Default, PartialEq)]
pub struct AppSettingsElement
{
	labelWidth: Option<Size>,
}

impl Component for AppSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
		
		let battleNet = match appSettings.read().enabledPlatforms.battleNet
		{
			false => None,
			true => Some(BattleNetSettingsElement::new()),
		};
		
		let egs = match appSettings.read().enabledPlatforms.epicGamesStores
		{
			false => None,
			true => Some(EgsSettingsElement::new()),
		};
		
		let gog = match appSettings.read().enabledPlatforms.gog
		{
			false => None,
			true => Some(GogSettingsElement::new()),
		};
		
		let ra = match appSettings.read().enabledPlatforms.retroAchievements
		{
			false => None,
			true => Some(RetroAchievementsSettingsElement::new()),
		};
		
		let rpcs3 = match appSettings.read().enabledPlatforms.rpcs3
		{
			false => None,
			true => Some(Rpcs3SettingsElement::new()),
		};
		
		let steam = match appSettings.read().enabledPlatforms.steam
		{
			false => None,
			true => Some(SteamSettingsElement::new()),
		};
		
		return rect()
			.direction(Direction::Vertical)
			.width(Size::Fill)
			
			.child(
				ScrollView::new()
					.spacing(15.0)
					.child(UiSettings::new())
					.child(NotificationSettings::new())
					.maybe_child(battleNet)
					.maybe_child(egs)
					.maybe_child(gog)
					.maybe_child(ra)
					.maybe_child(rpcs3)
					.maybe_child(steam)
					.child(LocalInfo::new())
			);
	}
}

impl AppSettingsElement
{
	pub fn new() -> Self
	{
		return Self::default();
	}
	
	#[allow(unused)]
	pub fn labelWidth(mut self, width: impl Into<Size>) -> Self
	{
		self.labelWidth = Some(width.into());
		return self;
	}
}
