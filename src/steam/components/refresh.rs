use crate::constants::Icon_Locked;
use crate::data::AppData;
use crate::data::secure::getSteamAuth;
use crate::io::{FileName_GameIcon, Path_Games};
use crate::{join, jpg, jpgAlt};
use crate::net::limiter::request::{DataOperation, FileLocation, RequestData};
use crate::steam::SteamApi;

pub async fn refreshPlayerSummary(mut appData: AppData) -> (AppData, Vec<RequestData>)
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

pub async fn refreshGameList(mut appData: AppData) -> (AppData, Vec<RequestData>)
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

pub async fn refreshGameSchema(mut appData: AppData, id: u64) -> (AppData, Vec<RequestData>)
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

pub async fn refreshGameAchievements(mut appData: AppData, id: u64) -> AppData
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

pub async fn refreshGlobalPercentages(mut appData: AppData, id: u64) -> AppData
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
