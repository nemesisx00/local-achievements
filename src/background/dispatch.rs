#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use ::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::userData;
use crate::platforms::retroachievements::RetroAchievementsApi;
use crate::platforms::steam::SteamApi;
use crate::state::{readAuth_RetroAchievements, readAuth_Steam, saveUserData};
use super::{ApiCommand, CommandResponse, Internal, SteamEndpoint};

/**
The dispatcher which handles all ApiCommands in the background.
*/
#[derive(Debug)]
pub struct Dispatcher
{
	receiver: UnboundedReceiver<ApiCommand>,
	retro: RetroAchievementsApi,
	sender: UnboundedSender<CommandResponse>,
	steam: SteamApi,
}

impl Dispatcher
{
	pub fn new(rx: UnboundedReceiver<ApiCommand>, tx: UnboundedSender<CommandResponse>) -> Self
	{
		return Self
		{
			receiver: rx,
			retro: match readAuth_RetroAchievements()
			{
				Ok(auth) => RetroAchievementsApi::new(auth),
				Err(_) => RetroAchievementsApi::default()
			},
			sender: tx,
			steam: match readAuth_Steam()
			{
				Ok(auth) => SteamApi::new(auth),
				Err(_) => SteamApi::default()
			},
		};
	}
	
	pub async fn start(&mut self)
	{
		loop {
			let command = &mut self.receiver.recv().await;
			match command
			{
				Some(cmd) => match cmd
				{
					ApiCommand::Metadata(metadata) => self.handleMetadata(metadata.to_owned()),
					ApiCommand::Print(message) => println!("Message: {}", message),
					ApiCommand::Steam(endpoint) => self.handleSteamEndpoint(endpoint.to_owned()).await,
				},
				None => {},
			}
		}
	}
	
	fn handleMetadata(&mut self, metadata: Internal)
	{
		match metadata
		{
			Internal::GetSteamAuth => self.transmitSteamAuth(),
			Internal::SaveUserData => self.saveUserData(),
			Internal::SteamAuth(auth) => self.steam.auth = auth.to_owned(),
			Internal::UpdateSteamApiKey(apiKey) => self.steam.auth.key = apiKey.to_owned(),
			Internal::UpdateSteamId(id) => self.steam.auth.id = id.to_owned(),
		}
	}
	
	async fn handleSteamEndpoint(&self, endpoint: SteamEndpoint)
	{
		match endpoint
		{
			SteamEndpoint::GlobalPercentages(appId) => self.steamGlobalPercentages(appId).await,
			SteamEndpoint::OwnedGames => self.steamOwnedGames().await,
			SteamEndpoint::SchemaForGame(appId, language) => self.steamSchemaForGame(appId, language).await,
			SteamEndpoint::PlayerAchievements(appId, language) => self.steamPlayerAchievements(appId, language).await,
			SteamEndpoint::PlayerSummaries => self.steamPlayerSummaries().await,
			SteamEndpoint::RecentlyPlayedGames => self.steamRecentlyPlayedGames().await,
		}
	}
	
	fn saveUserData(&self)
	{
		if let Ok(user) = userData().lock()
		{
			match saveUserData(user.clone())
			{
				Ok(_) => println!("User data saved!"),
				Err(e) => println!("Error saving user data: {:?}", e),
			}
		}
	}
	
	async fn steamGlobalPercentages(&self, appId: usize)
	{
		if let Ok(payload) = self.steam.getGlobalPercentages(appId).await
		{
			println!("{:?}", payload.asMap());
		}
	}
	
	async fn steamOwnedGames(&self)
	{
		if let Ok(payload) = self.steam.getOwnedGames().await
		{
			let mut steamInfos = vec![];
			if let Ok(mut user) = userData().lock()
			{
				if !payload.response.games.is_empty()
				{
					user.processSteamGames(payload.response.games);
				}
				
				steamInfos = user.getAllSteamInfo();
			}
			
			match self.steam.cacheGameIcons(steamInfos, false).await
			{
				Some(games) => {
					let idList = games.iter().fold(String::default(), |acc, game| format!("{}, {}", acc, game.id));
					println!("SteamApi: Error caching icon images for {}", idList[2..].to_string());
					let _ = self.sender.send(CommandResponse::Refresh);
				},
				None => println!("SteamApi: Icon images cached for owned games!"),
			}
		}
	}
	
	async fn steamPlayerAchievements(&self, appId: usize, language: String)
	{
		if let Ok(payload) = self.steam.getPlayerAchievements(appId, language).await
		{
			println!("{:?}", payload);
			if let Ok(mut user) = userData().lock()
			{
				user.processSteamAchievements(appId, payload.playerstats.achievements);
				let _ = self.sender.send(CommandResponse::Refresh);
			}
		}
	}
	
	async fn steamPlayerSummaries(&self)
	{
		if let Ok(payload) = self.steam.getPlayerSummaries().await
		{
			println!("{:?}", payload);
			if let Some(profile) = payload.response.players.first()
			{
				if let Ok(mut user) = userData().lock()
				{
					user.steam.update(
						profile.steamid.to_owned(),
						profile.personaname.to_owned(),
						match profile.avatarhash.is_empty()
						{
							true => None,
							false => Some(profile.avatarhash.to_owned()),
						}
					);
				}
				
				if !profile.avatarhash.is_empty()
				{
					match self.steam.cacheProfileAvatar(profile.steamid.to_owned(), profile.avatarhash.to_owned(), false).await
					{
						Ok(_) => {
							println!("Avatars cached");
							let _ = self.sender.send(CommandResponse::Refresh);
						},
						Err(e) => println!("Error caching avatars: {:?}", e),
					}
				}
			}
		}
	}
	
	async fn steamSchemaForGame(&self, appId: usize, language: String)
	{
		if let Ok(payload) = self.steam.getSchemaForGame(appId, language).await
		{
			println!("{:?}", payload);
			if let Some(achievements) = payload.game.availableGameStats.achievements
			{
				if let Ok(mut user) = userData().lock()
				{
					user.processSteamAchievementMetadata(appId, achievements.to_owned());
				}
				
				let _ = self.steam.cacheAchievementsIcons(appId, achievements, false).await;
				println!("Done caching achievement icons!");
				let _ = self.sender.send(CommandResponse::Refresh);
			}
		}
	}
	
	async fn steamRecentlyPlayedGames(&self)
	{
		if let Ok(payload) = self.steam.getRecentlyPlayedGames().await
		{
			println!("{:?}", payload);
		}
	}
	
	fn transmitSteamAuth(&self)
	{
		let _ = self.sender.send(CommandResponse::SteamAuth(self.steam.auth.to_owned()));
	}
}
