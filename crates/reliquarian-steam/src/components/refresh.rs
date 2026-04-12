use data::constants::{FileName_GameIcon, Icon_Locked, Path_Games};
use data::io::FileLocation;
use macros::{join, jpg, jpgAlt};
use net::{DataOperation, DataRequest};
use tracing::{info, warn};
use crate::api::SteamApi;
use crate::data::io::saveUserData;
use crate::data::operation::SteamOperation;
use crate::data::result::SteamOperationResult;
use crate::data::user::SteamUser;
use crate::secure::getSteamAuth;

pub async fn handleSteamOperation(mut user: SteamUser, dataOperation: DataOperation, language: String) -> Option<SteamOperationResult>
{
	return match dataOperation.try_into()
	{
		Err(_) => None,
		Ok(operation) => match operation
		{
			SteamOperation::GetGameList => {
				let result = refreshGameList(user).await;
				info!("[Steam API] Refreshed game list");
				
				Some(result)
			}
			
			SteamOperation::GetGlobalPercentages(id) => {
				let user = refreshGlobalPercentages(user, id).await;
				info!("[Steam API] Refreshed global percentages for app id {}", id);
				
				Some(user.into())
			}
			
			SteamOperation::GetPlayerAchievements(id) => {
				let user = refreshGameAchievements(user, id, language).await;
				info!("[Steam API] Refreshed achievements for app id {}", id);
				
				Some(user.into())
			}
			
			SteamOperation::GetPlayerSummary => {
				let result = refreshPlayerSummary(user).await;
				info!("[Steam API] Refreshed player summary");
				
				Some(result)
			}
			
			SteamOperation::GetSchemaForGame(id)  => {
				let result = refreshGameSchema(user, id, language).await;
				info!("[Steam API] Refreshed schema for app id {}", id);
				
				Some(result)
			}
			
			SteamOperation::SaveToFile => {
				match saveUserData(&user)
				{
					Err(e) => warn!("[Steam] Error saving user data: {:?}", e),
					Ok(_) => info!("[Steam] Saved user data"),
				}
				
				None
			}
			
			SteamOperation::SetGameLoaded(id, loaded) => {
				if let Some(game) = user.games.iter_mut()
					.find(|g| g.id == id)
				{
					game.loaded = loaded;
				}
				
				Some(user.into())
			}
		}
	};
}

async fn refreshPlayerSummary(mut user: SteamUser) -> SteamOperationResult
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		
		if let Ok(payload) = api.getPlayerSummaries().await
		{
			if let Some(profile) = payload.response.players.first()
			{
				user.update(
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
						user.id.clone(),
						user.avatar.clone().unwrap_or_default(),
						i
					);
					
					requests.push(DataRequest
					{
						destination: Some(avatarDestination),
						operation: DataOperation::CacheImage(true),
						url: Some(avatarUrl),
					});
				}
			}
		}
	}
	
	return SteamOperationResult
	{
		user,
		requests
	};
}

async fn refreshGameList(mut user: SteamUser) -> SteamOperationResult
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getOwnedGames().await
		{
			user.processOwnedGames(payload);
			
			// Cache game icons
			let platform = SteamApi::Platform.to_string();
			let fileName = jpg!(FileName_GameIcon);
			
			for game in user.games.iter()
			{
				let destination = FileLocation
				{
					fileName: fileName.clone(),
					group: join!(Path_Games, game.id),
					platform: platform.clone(),
				};
				
				let url = SteamApi::constructGameIconUrl(game.id, &game.iconHash);
				
				requests.push(DataRequest
				{
					destination: Some(destination),
					operation: DataOperation::CacheImage(false),
					url: Some(url)
				});
			}
		}
	}
	
	return SteamOperationResult
	{
		user,
		requests,
	};
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

async fn refreshGameSchema(mut user: SteamUser, id: u64, language: String) -> SteamOperationResult
{
	let mut requests = vec![];
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getSchemaForGame(id, &language).await
		{
			if let Some(game) = user.games.iter_mut()
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
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: jpg!(achievement.name),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(achievement.icon),
					});
					
					//Locked
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: jpgAlt!(achievement.name, Icon_Locked),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(achievement.icongray),
					});
				}
			}
		}
	}
	
	return SteamOperationResult
	{
		user,
		requests,
	};
}

async fn refreshGameAchievements(mut user: SteamUser, id: u64, language: String) -> SteamUser
{
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getPlayerAchievements(id, &language).await
		{
			if let Some(game) = user.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.updateAchievementsState(&payload);
			}
		}
	}
	
	return user;
}

async fn refreshGlobalPercentages(mut user: SteamUser, id: u64) -> SteamUser
{
	if getSteamAuth().is_ok_and(|a| a.validate())
	{
		let api = SteamApi::default();
		if let Ok(payload) = api.getGlobalPercentages(id).await
		{
			if let Some(game) = user.games.iter_mut()
				.find(|g| g.id == id)
			{
				game.updateGlobalPercentages(&payload);
			}
		}
	}
	
	return user;
}
