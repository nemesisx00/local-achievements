use freya::prelude::{ChildrenExt, Component, ContainerExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Gaps, InputMode, IntoElement,
	ScrollView, Size, State, Switch, rect, spawn, use_side_effect};
use freya::radio::use_radio;
use tracing::{info, warn};
use crate::components::settings::local::LocalInfo;
use crate::components::settings::notifications::NotificationSettings;
use crate::components::settings::ui::UiSettings;
use crate::data::radio::AppDataChannel;
use crate::data::{AppData, AppSettings, PlatformState};
use crate::gog::GogSettingsElement;
use crate::io::{saveAppSettings,// saveAuthData_BattleNet,
	saveAuthData_RetroAchievements, saveAuthData_Steam, saveSettings_Rpcs3};
use crate::retroachievements::RetroAchievementsSettingsElement;
use crate::rpcs3::Rpcs3SettingsElement;
use crate::steam::SteamSettingsElement;

pub const InputModeHiddenChar: char = '*';

#[derive(Clone, Default, PartialEq)]
pub struct AppSettingsElement
{
	labelWidth: Option<Size>,
}

impl Component for AppSettingsElement
{
	fn render(&self) -> impl IntoElement
	{
		let appData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		
		use_side_effect(move || saveChanges(
			appData.read().app.settings.clone(),
			appData.read().platform.clone()
		));
		
		return rect()
			.direction(Direction::Vertical)
			.width(Size::Fill)
			
			.child(
				ScrollView::new()
					.spacing(15.0)
					.child(UiSettings::new())
					.child(NotificationSettings::new())
					//.child(BattleNetSettingsElement::new())
					.child(GogSettingsElement::new())
					.child(RetroAchievementsSettingsElement::new())
					.child(Rpcs3SettingsElement::new())
					.child(SteamSettingsElement::new())
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

pub fn SettingsSwitch(mut inputMode: State<InputMode>) -> impl IntoElement
{
	let value = inputMode.read().clone();
	
	return rect()
		.margin(Gaps::new(4.0, 0.0, 0.0, 0.0))
		.width(Size::FillMinimum)
		.child(
			Switch::new()
				.toggled(value == InputMode::Shown)
				.on_toggle(move |_| match value
				{
					InputMode::Shown => inputMode.set(InputMode::Hidden(InputModeHiddenChar)),
					InputMode::Hidden(_) => inputMode.set(InputMode::Shown),
				})
		);
}
	
fn saveChanges(app: AppSettings, platform: PlatformState)
{
	spawn(async move {
		match saveAppSettings(&app)
		{
			Err(e) => warn!("[Local Achievements] Error saving app settings: {:?}", e),
			Ok(_) => info!("[Local Achievements] Saved app settings"),
		}
		
		/*
		match saveAuthData_BattleNet(&platform.battleNetAuth)
		{
			Err(e) => warn!("[Battle.Net] Error saving authentication settings: {:?}", e),
			Ok(_) => info!("[Battle.Net] Saved authentication settings"),
		}
		*/
		
		match saveAuthData_RetroAchievements(&platform.retroAchievements)
		{
			Err(e) => warn!("[RetroAchievements] Error saving authentication settings: {:?}", e),
			Ok(_) => info!("[RetroAchievements] Saved authentication settings"),
		}
		
		match saveSettings_Rpcs3(&platform.rpcs3)
		{
			Err(e) => warn!("[RPCS3] Error saving settings: {:?}", e),
			Ok(_) => info!("[RPCS3] Saved settings"),
		}
		
		match saveAuthData_Steam(&platform.steam)
		{
			Err(e) => warn!("[Steam] Error saving authentication settings: {:?}", e),
			Ok(_) => info!("[Steam] Saved authentication settings"),
		}
	});
}
