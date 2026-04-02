use std::collections::VecDeque;
use freya::prelude::{App, ChildrenExt, ContainerSizeExt,
	ContainerWithContentExt, Direction, Element, IntoElement, Platform,
	StyleExt, WinitPlatformExt, rect, spawn, use_init_theme, use_side_effect,
	use_state};
use freya::radio::{RadioStation, use_init_radio_station, use_radio,
	use_share_radio};
use freya::winit::dpi::PhysicalSize;
use reqwest::Client;
use tracing::{info, warn};
use crate::battlenet::{self, BattleNetContentElement};
use crate::components::ProfileState;
use crate::components::nav::NavBar;
use crate::components::settings::AppSettingsElement;
use crate::constants::{AppTheme, BackgroundColor, DefaultHttpRequestRate,
	TextColor};
use crate::data::radio::{AppDataChannel, DataChannel};
use crate::data::{ActiveContent, AppData};
use crate::gog::{self, GogContentElement};
use crate::io::imagePathExists;
use crate::net::limiter::RateLimiter;
use crate::net::limiter::request::{DataOperation, RequestEvent};
use crate::retroachievements::{self, RetroAchievementsContent};
use crate::rpcs3::Rpcs3ContentElement;
use crate::steam::{self, SteamContent};
use crate::util::cacheImage;

pub struct LocalAchievementsApp
{
	radioStation: RadioStation<AppData, AppDataChannel>,
}

impl App for LocalAchievementsApp
{
	fn render(&self) -> impl IntoElement
	{
		use_init_theme(|| AppTheme);
		use_share_radio(move || self.radioStation);
		use_init_radio_station::<Option<ActiveContent>, DataChannel>(Default::default);
		use_init_radio_station::<VecDeque<String>, DataChannel>(Default::default);
		use_init_radio_station::<ProfileState, DataChannel>(Default::default);
		use_init_radio_station::<Option<bool>, DataChannel>(Default::default);
		use_init_radio_station::<PhysicalSize<u32>, DataChannel>(Default::default);
		use_init_radio_station::<RateLimiter, DataChannel>(|| RateLimiter::new(DefaultHttpRequestRate));
		use_init_radio_station::<RequestEvent, DataChannel>(|| RequestEvent::Done);
		
		let settingsData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Settings);
		let rateLimiter = use_radio::<RateLimiter, DataChannel>(DataChannel::RateLimiter);
		let mut requestEvent = use_radio::<RequestEvent, DataChannel>(DataChannel::RateLimiter);
		let mut battleNetData = use_radio::<AppData, AppDataChannel>(AppDataChannel::BattleNet);
		let mut gogData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Gog);
		let mut retroAchievementsData = use_radio::<AppData, AppDataChannel>(AppDataChannel::RetroAchievements);
		let mut steamData = use_radio::<AppData, AppDataChannel>(AppDataChannel::Steam);
		let activeContent = use_radio::<Option<ActiveContent>, DataChannel>(DataChannel::ActiveContent);
		let mut windowSize = use_radio::<PhysicalSize<u32>, DataChannel>(DataChannel::WindowSize);
		
		let mut limiterSpawned = use_state(bool::default);
		
		Platform::get().with_window(
			None,
			move |window| **windowSize.write() = window.inner_size()
		);
		
		let active = activeContent.read().clone()
			.unwrap_or(settingsData.read().app.settings.defaultActivePlatform);
		
		let activeContent: Option<Element> = match active
		{
			ActiveContent::BattleNet => Some(BattleNetContentElement::new().into()),
			//ActiveContent::EpicGamesStore => Some(EgsContentElement::new().into()),
			ActiveContent::Gog => Some(GogContentElement::new().into()),
			ActiveContent::RetroAchievements => Some(RetroAchievementsContent::new().into()),
			ActiveContent::Rpcs3 => Some(Rpcs3ContentElement::new().into()),
			ActiveContent::Settings => Some(AppSettingsElement::new().into()),
			ActiveContent::Steam => Some(SteamContent::new().into()),
			_ => None,
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
							
							match request.operation
							{
								DataOperation::CacheImage => if let Some(destination) = request.destination
								{
									if let Some(url) = request.url
									{
										if !imagePathExists(&destination)
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
								
								DataOperation::BattleNet(operation) => {
									let appData = battleNetData.read().clone();
									if let Some(result) = battlenet::handleDataOperation(appData, operation).await
									{
										battleNetData.write().platform.battleNet = result.appData.platform.battleNet;
										battleNetData.write().user.battleNet = result.appData.user.battleNet;
										rateLimiter.read().pushAll(result.requests).await;
									}
								}
								
								DataOperation::Gog(operation) => {
									let appData = gogData.read().clone();
									if let Some(result) = gog::handleDataOperation(appData, operation).await
									{
										gogData.write().user.gog = result.appData.user.gog;
										rateLimiter.read().pushAll(result.requests).await;
									}
								}
								
								DataOperation::RetroAchievements(operation) => {
									let appData = retroAchievementsData.read().clone();
									if let Some(result) = retroachievements::handleDataOperation(appData, operation).await
									{
										retroAchievementsData.write().user.retroAchievements = result.appData.user.retroAchievements;
										rateLimiter.read().pushAll(result.requests).await;
									}
								}
								
								DataOperation::Steam(operation) => {
									let appData = steamData.read().clone();
									if let Some(result) = steam::handleDataOperation(appData, operation).await
									{
										steamData.write().user.steam = result.appData.user.steam;
										rateLimiter.read().pushAll(result.requests).await;
									}
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
			
			.child(NavBar())
			
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
	pub fn new(
		radioStation: RadioStation<AppData, AppDataChannel>,
	) -> Self
	{
		return Self
		{
			radioStation,
		};
	}
}
