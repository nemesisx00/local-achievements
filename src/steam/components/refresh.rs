use tracing::{info, warn};
use crate::constants::Icon_Locked;
use crate::data::AppData;
use crate::data::secure::getSteamAuth;
use crate::io::{FileName_GameIcon, Path_Games, saveUserData_Steam};
use crate::{join, jpg, jpgAlt};
use crate::net::limiter::request::{DataOperation, DataOperationResult,
	FileLocation, RequestData, SteamOperation};
use crate::steam::SteamApi;

pub async fn handleDataOperation(mut appData: AppData, operation: SteamOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		SteamOperation::GetGameList => {
			let (appData, requests) = refreshGameList(appData).await;
			info!("[Steam API] Refreshed game list");
			
			Some(DataOperationResult
			{
				appData,
				requests,
			})
		}
		
		SteamOperation::GetGlobalPercentages(id) => {
			let appData = refreshGlobalPercentages(appData, id).await;
			info!("[Steam API] Refreshed global percentages for app id {}", id);
			
			Some(DataOperationResult
			{
				appData,
				requests: vec![],
			})
		}
		
		SteamOperation::GetPlayerAchievements(id) => {
			let appData = refreshGameAchievements(appData, id).await;
			info!("[Steam API] Refreshed achievements for app id {}", id);
			
			Some(DataOperationResult
			{
				appData,
				requests: vec![],
			})
		}
		
		SteamOperation::GetPlayerSummary => {
			let (appData, requests) = refreshPlayerSummary(appData).await;
			info!("[Steam API] Refreshed player summary");
			
			Some(DataOperationResult
			{
				appData,
				requests,
			})
		}
		
		SteamOperation::GetSchemaForGame(id)  => {
			let (appData, requests) = refreshGameSchema(appData, id).await;
			info!("[Steam API] Refreshed schema for app id {}", id);
			
			Some(DataOperationResult
			{
				appData,
				requests,
			})
		}
		
		SteamOperation::SaveToFile => {
			match saveUserData_Steam(&appData.user.steam)
			{
				Err(e) => warn!("[Steam] Error saving user data: {:?}", e),
				Ok(_) => info!("[Steam] Saved user data"),
			}
			
			None
		}
		
		SteamOperation::SetGameLoaded(id, loaded) => {
			if let Some(game) = appData.user.steam.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.loaded = loaded;
			}
			
			Some(DataOperationResult
			{
				appData,
				requests: vec![],
			})
		}
	};
}

async fn refreshPlayerSummary(mut appData: AppData) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		
		if let Ok(payload) = api.getPlayerSummaries().await
		{
			if let Some(profile) = payload.response.players.first()
			{
				appData.user.steam.update(
					&profile.steamid,
					&profile.personaname,
					match profile.avatarhash.is_empty()
					{
						false => Some(&profile.avatarhash),
						true => None,
					}
				);
				
				// Cache user avatar images
				for i in 0..3
				{
					let (avatarDestination, avatarUrl) = SteamApi::constructProfileAvatarMetadata(
						appData.user.steam.id.clone(),
						appData.user.steam.avatar.clone().unwrap_or_default(),
						i
					);
					
					requests.push(RequestData
					{
						destination: Some(avatarDestination),
						operation: DataOperation::CacheImage,
						url: Some(avatarUrl),
					});
				}
			}
		}
	}
	
	return (appData, requests);
}

async fn refreshGameList(mut appData: AppData) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getOwnedGames().await
		{
			appData.user.steam.processOwnedGames(payload);
			
			// Cache game icons
			let platform = SteamApi::Platform.to_string();
			let fileName = jpg!(FileName_GameIcon);
			
			for game in appData.user.steam.games.iter()
			{
				let destination = FileLocation
				{
					fileName: fileName.clone(),
					group: join!(Path_Games, game.id),
					platform: platform.clone(),
				};
				
				let url = SteamApi::constructGameIconUrl(game.id, &game.iconHash);
				
				requests.push(RequestData
				{
					destination: Some(destination),
					operation: DataOperation::CacheImage,
					url: Some(url)
				});
			}
		}
	}
	
	return (appData, requests);
}

/*
if let Ok(payload) = api.getRecentlyPlayedGames().await
{
	if !payload.response.games.is_empty()
	{
		// Good for play time in past 2 weeks
		// Will require update to the Game struct
		// Should Option<> and only titles returned from this have a value
		//	when processing from here, first step is to clear all existing values
	}
}
*/

async fn refreshGameSchema(mut appData: AppData, id: u64) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getSchemaForGame(id, &appData.app.settings.language).await
		{
			if let Some(game) = appData.user.steam.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.updateAchievementsMetadata(&payload);
			}
			
			// Cache achievement icons
			if let Some(achievements) = payload.game.availableGameStats.achievements
			{
				let group = join!(Path_Games, id);
				let platform = SteamApi::Platform.to_string();
				
				for achievement in achievements
				{
					//Unlocked
					requests.push(RequestData
					{
						destination: Some(FileLocation
						{
							fileName: jpg!(achievement.name),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage,
						url: Some(achievement.icon),
					});
					
					//Locked
					requests.push(RequestData
					{
						destination: Some(FileLocation
						{
							fileName: jpgAlt!(achievement.name, Icon_Locked),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage,
						url: Some(achievement.icongray),
					});
				}
			}
		}
	}
	
	return (appData, requests);
}

async fn refreshGameAchievements(mut appData: AppData, id: u64) -> AppData
{
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getPlayerAchievements(id, &appData.app.settings.language).await
		{
			if let Some(game) = appData.user.steam.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.updateAchievementsState(&payload);
			}
		}
	}
	
	return appData;
}

async fn refreshGlobalPercentages(mut appData: AppData, id: u64) -> AppData
{
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getGlobalPercentages(id).await
		{
			if let Some(game) = appData.user.steam.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.updateGlobalPercentages(&payload);
			}
		}
	}
	
	return appData;
}
