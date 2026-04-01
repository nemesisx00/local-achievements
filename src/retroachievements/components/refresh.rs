use tracing::warn;
use crate::constants::Icon_Locked;
use crate::data::AppData;
use crate::io::{FileName_GameIcon, Path_Avatars, Path_Games};
use crate::net::limiter::request::{DataOperation, FileLocation, RequestData, RetroAchievementsOperation};
use crate::{join, png, pngAlt};
use crate::retroachievements::{RetroAchievementsApi, RetroAchievementsProgressState, makeRelative};

pub fn refreshUserProfile(mut appData: AppData) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	
	if appData.platform.retroAchievements.isValid()
	{
		let api = RetroAchievementsApi::from(appData.platform.retroAchievements.clone());
		let ulid = appData.user.retroAchievements.ulid.clone();
		if let Ok(payload) = api.getUserProfile(ulid.clone())
		{
			appData.user.retroAchievements.processUserProfile(&payload);
			
			if let Some(ulid) = ulid
			{
				if let Some(avatarPath) = appData.user.retroAchievements.avatar.clone()
				{
					requests.push(RequestData
					{
						destination: Some(FileLocation
						{
							fileName: png!(ulid),
							group: Path_Avatars.into(),
							platform: RetroAchievementsApi::Platform.into(),
						}),
						
						operation: DataOperation::CacheImage,
						url: api.buildMediaUrl(&avatarPath)
					});
				}
			}
		}
	}
	
	return (appData, requests);
}

pub fn refreshUserProgress(mut appData: AppData, mut state: RetroAchievementsProgressState)
	-> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	
	if appData.platform.retroAchievements.isValid()
	{
		let api = RetroAchievementsApi::from(appData.platform.retroAchievements.clone());
		match api.getUserCompletionProgress(appData.user.retroAchievements.ulid.clone(), Some(state.offset))
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
					if let Some(url) = api.buildMediaUrl(&makeRelative(&game.ImageIcon))
					{
						requests.push(RequestData
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
	
	return (appData, requests);
}

pub fn refreshGameInfo(mut appData: AppData, id: u64) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	
	if appData.platform.retroAchievements.isValid()
	{
		let api = RetroAchievementsApi::from(appData.platform.retroAchievements.clone());
		
		let ulid = match appData.user.retroAchievements.ulid.clone()
		{
			None => appData.user.retroAchievements.username.clone(),
			Some(ulid) => ulid,
		};
		
		match api.getGameInfo(&ulid, id)
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
					if let Some(url) = api.buildMediaUrl(
						join!(
							RetroAchievementsApi::BadgePath,
							png!(achievement.BadgeName)
						).as_str()
					)
					{
						requests.push(RequestData
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
					if let Some(url) = api.buildMediaUrl(
						join!(
							RetroAchievementsApi::BadgePath,
							pngAlt!(
								achievement.BadgeName,
								RetroAchievementsApi::BadgeLockedSuffix
							)
						).as_str()
					)
					{
						requests.push(RequestData
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
	
	return (appData, requests);
}
