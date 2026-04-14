use std::collections::VecDeque;
use battlenet::api::BattleNetSettings;
use battlenet::components::content::BattleNetContentElement;
use battlenet::components::refresh::handleBattleNetOperation;
use battlenet::data::io::{loadSettings_BattleNet, loadUserData_BattleNet};
use battlenet::data::user::BattleNetUser;
use data::constants::{BackgroundColor, DefaultHttpRequestRate, TextColor};
use data::enums::{ActiveContent, DataChannel, GamePlatforms};
use data::io::{cacheImage, imagePathExists, loadAppSettings};
use data::localAchievementsTheme;
use data::settings::AppSettings;
use epicgamesstore::components::content::EgsContentElement;
use epicgamesstore::components::refresh::handleEgsOperation;
use epicgamesstore::data::io::loadUserData_EpicGamesStore;
use epicgamesstore::data::user::EgsUser;
use freya::prelude::{App, ChildrenExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, LayerExt,
	Platform, StyleExt, WinitPlatformExt, WritableUtils, rect, spawn,
	use_init_theme, use_side_effect, use_state};
use freya::radio::{Radio, use_init_radio_station, use_radio};
use freya::winit::dpi::PhysicalSize;
use gog::components::content::GogContentElement;
use gog::components::refresh::handleGogOperation;
use gog::data::io::loadUserData_Gog;
use gog::data::user::GogUser;
use net::{DataOperation, RateLimiter, RequestEvent};
use reqwest::Client;
use retroachievements::components::content::RetroAchievementsContent;
use retroachievements::components::refresh::handleRetroAchievementsOperation;
use retroachievements::data::io::loadUserData_RetroAchievements;
use retroachievements::data::user::RetroAchievementsUser;
use rpcs3::components::content::Rpcs3ContentElement;
use rpcs3::data::io::{loadSettings_Rpcs3, loadUserData_Rpcs3};
use rpcs3::data::settings::Rpcs3Settings;
use rpcs3::data::user::Rpcs3User;
use steam::components::content::SteamContent;
use steam::components::refresh::handleSteamOperation;
use steam::data::io::loadUserData_Steam;
use steam::data::user::SteamUser;
use tracing::{info, warn};
use crate::components::ProfileState;
use crate::components::nav::NavBar;
use crate::components::profile::ProfileElement;
use crate::components::settings::AppSettingsElement;

pub struct LocalAchievementsApp;

impl App for LocalAchievementsApp
{
	fn render(&self) -> impl IntoElement
	{
		use_init_theme(|| localAchievementsTheme());
		
		use_init_radio_station::<AppSettings, DataChannel>(|| loadAppSettings().unwrap_or_default());
		use_init_radio_station::<Option<ActiveContent>, DataChannel>(Default::default);
		use_init_radio_station::<VecDeque<String>, DataChannel>(Default::default);
		use_init_radio_station::<ProfileState, DataChannel>(Default::default);
		use_init_radio_station::<Option<bool>, DataChannel>(Default::default);
		use_init_radio_station::<PhysicalSize<u32>, DataChannel>(Default::default);
		use_init_radio_station::<RateLimiter, DataChannel>(|| RateLimiter::new(DefaultHttpRequestRate));
		use_init_radio_station::<RequestEvent, DataChannel>(|| RequestEvent::Done);
		
		use_init_radio_station::<BattleNetSettings, GamePlatforms>(loadSettings_BattleNet);
		use_init_radio_station::<BattleNetUser, GamePlatforms>(loadUserData_BattleNet);
		use_init_radio_station::<EgsUser, GamePlatforms>(loadUserData_EpicGamesStore);
		use_init_radio_station::<GogUser, GamePlatforms>(loadUserData_Gog);
		use_init_radio_station::<RetroAchievementsUser, GamePlatforms>(loadUserData_RetroAchievements);
		use_init_radio_station::<Rpcs3Settings, GamePlatforms>(loadSettings_Rpcs3);
		use_init_radio_station::<Rpcs3User, GamePlatforms>(loadUserData_Rpcs3);
		use_init_radio_station::<SteamUser, GamePlatforms>(loadUserData_Steam);
		
		let activeContent = use_radio::<Option<ActiveContent>, DataChannel>(DataChannel::ActiveContent);
		let appSettings = use_radio::<AppSettings, DataChannel>(DataChannel::Settings);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let bnetSettings = use_radio::<BattleNetSettings, GamePlatforms>(GamePlatforms::BattleNet);
		let mut bnetUser = use_radio::<BattleNetUser, GamePlatforms>(GamePlatforms::BattleNet);
		let mut egsUser = use_radio::<EgsUser, GamePlatforms>(GamePlatforms::EpicGamesStore);
		let mut gogUser = use_radio::<GogUser, GamePlatforms>(GamePlatforms::Gog);
		let mut retroAchievementsUser = use_radio::<RetroAchievementsUser, GamePlatforms>(GamePlatforms::RetroAchievements);
		let mut steamUser = use_radio::<SteamUser, GamePlatforms>(GamePlatforms::Steam);
		
		let mut limiterSpawned = use_state(bool::default);
		
		Platform::get().with_window(
			None,
			move |window| **windowSize.write() = window.inner_size()
		);
		
		let active = activeContent.read().clone()
			.unwrap_or(appSettings.read().defaultActivePlatform);
		
		let activeContent: Option<Element> = match active
		{
			ActiveContent::BattleNet => Some(BattleNetContentElement::new().into()),
			ActiveContent::EpicGamesStore => Some(EgsContentElement::new().into()),
			ActiveContent::Gog => Some(GogContentElement::new().into()),
			ActiveContent::RetroAchievements => Some(RetroAchievementsContent::new().into()),
			ActiveContent::Rpcs3 => Some(Rpcs3ContentElement::new().into()),
			ActiveContent::Settings => Some(AppSettingsElement::new().into()),
			ActiveContent::Steam => Some(SteamContent::new().into()),
		};
		
		use_side_effect(move || {
			if !limiterSpawned()
				&& *requestEvent.read() != RequestEvent::Done
				&& !rateLimiter.read().blockingIsEmpty()
			{
				spawn(async move {
					limiterSpawned.set(true);
					
					loop
					{
						if rateLimiter.read().isEmpty().await
						{
							break;
						}
						
						if let Some(request) = rateLimiter.read().next().await
						{
							//Update the request event with the current number of remaining requests, forces redraw of ui elements that rely on this value
							**requestEvent.write() = RequestEvent::Processing(rateLimiter.read().len().await);
							
							match request.operation.clone()
							{
								DataOperation::CacheImage(force) => if let Some(destination) = request.destination
								{
									if let Some(url) = request.url
									{
										if force || !imagePathExists(&destination)
										{
											let client = Client::builder()
												.https_only(true)
												.build()
												.unwrap_or_default();
											
											match cacheImage(&client, &url, &destination).await
											{
												Err(e) => warn!("[Cache] Error caching image {} - {:?}", destination, e),
												Ok(_) => info!("[Cache] Cached image: {}", destination),
											}
										}
										else
										{
											rateLimiter.read().refundUse()
												.await;
										}
									}
								}
								
								DataOperation::Platform(platform, _) => match platform
								{
									GamePlatforms::BattleNet => processBattleNetResult(request.operation, &mut bnetUser, bnetSettings.read().clone(), &rateLimiter).await,
									GamePlatforms::EpicGamesStore => processEgsResult(request.operation, &mut egsUser, &rateLimiter).await,
									GamePlatforms::Gog => processGogResult(request.operation, &mut gogUser, &rateLimiter).await,
									GamePlatforms::RetroAchievements => processRetroAchievementsResult(request.operation, &mut retroAchievementsUser, &rateLimiter).await,
									GamePlatforms::Steam => processSteamResult(request.operation, &mut steamUser, &rateLimiter, appSettings.read().language.clone()).await,
									_ => {}
								}
								
								DataOperation::PlatformGameId(platform, _, _) => match platform
								{
									GamePlatforms::Gog => processGogResult(request.operation, &mut gogUser, &rateLimiter).await,
									GamePlatforms::RetroAchievements => processRetroAchievementsResult(request.operation, &mut retroAchievementsUser, &rateLimiter).await,
									GamePlatforms::Steam => processSteamResult(request.operation, &mut steamUser, &rateLimiter, appSettings.read().language.clone()).await,
									_ => {}
								}
								
								DataOperation::PlatformGameIdBool(platform, _, _, _) => match platform
								{
									GamePlatforms::Steam => processSteamResult(request.operation, &mut steamUser, &rateLimiter, appSettings.read().language.clone()).await,
									_ => {}
								}
								
								DataOperation::PlatformGameIdString(platform, _, _) => match platform
								{
									GamePlatforms::EpicGamesStore => processEgsResult(request.operation, &mut egsUser, &rateLimiter).await,
									_ => {}
								}
								
								DataOperation::PlatformOptionalInt(platform, _, _) => match platform
								{
									GamePlatforms::Gog => processGogResult(request.operation, &mut gogUser, &rateLimiter).await,
									_ => {}
								}
								
								DataOperation::PlatformSaveToFile(platform) => match platform
								{
									GamePlatforms::BattleNet => processBattleNetResult(request.operation, &mut bnetUser, bnetSettings.read().clone(), &rateLimiter).await,
									GamePlatforms::EpicGamesStore => processEgsResult(request.operation, &mut egsUser, &rateLimiter).await,
									GamePlatforms::Gog => processGogResult(request.operation, &mut gogUser, &rateLimiter).await,
									GamePlatforms::RetroAchievements => processRetroAchievementsResult(request.operation, &mut retroAchievementsUser, &rateLimiter).await,
									GamePlatforms::Steam => processSteamResult(request.operation, &mut steamUser, &rateLimiter, appSettings.read().language.clone()).await,
									_ => {}
								}
								
								DataOperation::PlatformThreeInt(platform, _, _, _, _) => match platform
								{
									GamePlatforms::RetroAchievements => processRetroAchievementsResult(request.operation, &mut retroAchievementsUser, &rateLimiter).await,
									_ => {}
								}
							}
						}
					}
					
					**requestEvent.write() = RequestEvent::Done;
					limiterSpawned.set(false);
				});
			}
		});
		
		return rect()
			.background(BackgroundColor)
			.color(TextColor)
			.direction(Direction::Vertical)
			.expanded()
			.layer(1)
			
			.child(NavBar())
			
			.child(
				ProfileElement::new()
					.offset(64.0)
			)
			
			.child(
				rect()
					.direction(Direction::Vertical)
					.expanded()
					.maybe_child(activeContent)
			);
	}
}

impl LocalAchievementsApp
{
	pub fn new() -> Self
	{
		return Self {};
	}
}

async fn processBattleNetResult(
	operation: DataOperation,
	userRadio: &mut Radio<BattleNetUser, GamePlatforms>,
	settings: BattleNetSettings,
	rateLimiter: &Radio<RateLimiter, DataChannel>
)
{
	let user = userRadio.read().clone();
	if let Some(result) = handleBattleNetOperation(
		user,
		settings,
		operation
	).await
	{
		**userRadio.write() = result.user.clone();
		rateLimiter.read().pushAll(result.requests).await;
	}
}

async fn processEgsResult(
	operation: DataOperation,
	userRadio: &mut Radio<EgsUser, GamePlatforms>,
	rateLimiter: &Radio<RateLimiter, DataChannel>
)
{
	let user = userRadio.read().clone();
	if let Some(result) = handleEgsOperation(
		user,
		operation
	).await
	{
		**userRadio.write() = result.user.clone();
		rateLimiter.read().pushAll(result.requests).await;
	}
}

async fn processGogResult(
	operation: DataOperation,
	userRadio: &mut Radio<GogUser, GamePlatforms>,
	rateLimiter: &Radio<RateLimiter, DataChannel>
)
{
	let user = userRadio.read().clone();
	if let Some(result) = handleGogOperation(
		user,
		operation
	).await
	{
		**userRadio.write() = result.user.clone();
		rateLimiter.read().pushAll(result.requests).await;
	}
}

async fn processRetroAchievementsResult(
	operation: DataOperation,
	userRadio: &mut Radio<RetroAchievementsUser, GamePlatforms>,
	rateLimiter: &Radio<RateLimiter, DataChannel>
)
{
	let user = userRadio.read().clone();
	if let Some(result) = handleRetroAchievementsOperation(
		user,
		operation
	).await
	{
		**userRadio.write() = result.user.clone();
		rateLimiter.read().pushAll(result.requests).await;
	}
}

async fn processSteamResult(
	operation: DataOperation,
	userRadio: &mut Radio<SteamUser, GamePlatforms>,
	rateLimiter: &Radio<RateLimiter, DataChannel>,
	language: String,
)
{
	let user = userRadio.read().clone();
	if let Some(result) = handleSteamOperation(
		user,
		operation,
		language
	).await
	{
		**userRadio.write() = result.user.clone();
		rateLimiter.read().pushAll(result.requests).await;
	}
}
