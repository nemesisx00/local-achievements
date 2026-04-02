use tracing::{info, warn};
use crate::constants::Icon_Locked;
use crate::data::AppData;
use crate::data::secure::getRetroAchievementsAuth;
use crate::io::{FileName_GameIcon, Path_Avatars, Path_Games,
	saveUserData_RetroAchievements};
use crate::net::limiter::request::{DataOperation, DataOperationResult,
	FileLocation, DataRequest, RetroAchievementsOperation};
use crate::{join, png, pngAlt};
use crate::retroachievements::{RetroAchievementsApi,
	RetroAchievementsProgressState, makeRelative};

pub async fn handleDataOperation(appData: AppData, operation: RetroAchievementsOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		RetroAchievementsOperation::GetGameInfo(id) => {
			let result = refreshGameInfo(appData, id);
			info!("[RetroAchievements] Refreshed game info for {}", id);
			
			Some(result)
		}
		
		RetroAchievementsOperation::GetUserProfile => {
			let result = refreshUserProfile(appData);
			info!("[RetroAchievements] Refreshed user profile");
			
			Some(result)
		}
		
		RetroAchievementsOperation::GetUserProgress(state) => {
			let result = refreshUserProgress(appData, state.clone());
			info!("[RetroAchievements] Refreshed user progress");
			
			Some(result)
		}
		
		RetroAchievementsOperation::SaveToFile => {
			match saveUserData_RetroAchievements(&appData.user.retroAchievements)
			{
				Err(e) => warn!("[RetroAchievements] Error saving user data: {:?}", e),
				Ok(_) => info!("[RetroAchievements] Saved user data"),
			}
			
			None
		}
	};
}

fn refreshUserProfile(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		let ulid = appData.user.retroAchievements.ulid.clone();
		if let Ok(payload) = RetroAchievementsApi::getUserProfile(ulid.clone())
		{
			appData.user.retroAchievements.processUserProfile(&payload);
			
			if let Some(ulid) = ulid
			{
				if let Some(avatarPath) = appData.user.retroAchievements.avatar.clone()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: png!(ulid),
							group: Path_Avatars.into(),
							platform: RetroAchievementsApi::Platform.into(),
						}),
						
						operation: DataOperation::CacheImage,
						url: RetroAchievementsApi::buildMediaUrl(&avatarPath)
					});
				}
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}

fn refreshUserProgress(mut appData: AppData, mut state: RetroAchievementsProgressState) -> DataOperationResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		match RetroAchievementsApi::getUserCompletionProgress(appData.user.retroAchievements.ulid.clone(), Some(state.offset))
		{
			Err(e) => warn!("[RetroAchievements] Error retrieving user completion progress: {:?}", e),
			
			Ok(payload) => {
				// Update progress state
				state.received += payload.Count;
				state.offset += payload.Count;
				state.total = payload.Total;
				
				appData.user.retroAchievements.processUserCompletionProgress(&payload);
				
				match state.reachedEnd()
				{
					false => requests.push(RetroAchievementsOperation::GetUserProgress(state).into()),
					true => requests.push(RetroAchievementsOperation::SaveToFile.into()),
				}
				
				// Cache game icons
				let fileName = png!(FileName_GameIcon);
				let platform = RetroAchievementsApi::Platform.to_string();
				
				for game in payload.Results
				{
					if let Some(url) = RetroAchievementsApi::buildMediaUrl(&makeRelative(&game.ImageIcon))
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: fileName.clone(),
								group: join!(Path_Games, game.GameID.to_string()),
								platform: platform.clone(),
							}),
							operation: DataOperation::CacheImage,
							url: Some(url),
						});
					}
				}
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}

fn refreshGameInfo(mut appData: AppData, id: u64) -> DataOperationResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		let ulid = match appData.user.retroAchievements.ulid.clone()
		{
			None => appData.user.retroAchievements.username.clone(),
			Some(ulid) => ulid,
		};
		
		match RetroAchievementsApi::getGameInfo(&ulid, id)
		{
			Err(e) => warn!("[RetroAchievements] Error getting game info for {}: {:?}", id, e),
			
			Ok(payload) => {
				match appData.user.retroAchievements.games.iter_mut()
					.find(|g| g.id == id)
				{
					None => appData.user.retroAchievements.games.push(payload.clone().into()),
					Some(game) => game.updateDetailed(&payload),
				}
				
				// Cache achievement icons
				let group = join!(Path_Games, id.to_string());
				let platform = RetroAchievementsApi::Platform.to_string();
				
				for achievement in payload.Achievements.values()
				{
					let sanitizedTitle = RetroAchievementsApi::sanitizeIconTitle(&achievement.Title);
					
					// Unlocked
					if let Some(url) = RetroAchievementsApi::buildMediaUrl(
						join!(
							RetroAchievementsApi::BadgePath,
							png!(achievement.BadgeName)
						).as_str()
					)
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: png!(sanitizedTitle),
								group: group.clone(),
								platform: platform.clone(),
							}),
							
							operation: DataOperation::CacheImage,
							url: Some(url),
						});
					}
					
					// Locked
					if let Some(url) = RetroAchievementsApi::buildMediaUrl(
						join!(
							RetroAchievementsApi::BadgePath,
							pngAlt!(
								achievement.BadgeName,
								RetroAchievementsApi::BadgeLockedSuffix
							)
						).as_str()
					)
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: pngAlt!(sanitizedTitle, Icon_Locked),
								group: group.clone(),
								platform: platform.clone(),
							}),
							
							operation: DataOperation::CacheImage,
							url: Some(url),
						});
					}
				}
			}
		}
	}
	
	return DataOperationResult
	{
		appData,
		requests,
	};
}
