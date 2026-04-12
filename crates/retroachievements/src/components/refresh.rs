use data::constants::{FileName_GameIcon, Icon_Locked, Path_Avatars, Path_Games};
use data::io::FileLocation;
use macros::{join, png, pngAlt};
use net::{DataOperation, DataRequest};
use tracing::{info, warn};
use crate::api::RetroAchievementsApi;
use crate::data::io::saveUserData;
use crate::data::makeRelative;
use crate::data::operation::RetroAchievementsOperation;
use crate::data::progress::RetroAchievementsProgressState;
use crate::data::result::RetroAchievementsResult;
use crate::data::user::RetroAchievementsUser;
use crate::secure::getRetroAchievementsAuth;

pub async fn handleRetroAchievementsOperation(
	user: RetroAchievementsUser,
	dataOperation: DataOperation
) -> Option<RetroAchievementsResult>
{
	return match dataOperation.try_into()
	{
		Err(_) => None,
		Ok(operation) => match operation
		{
			RetroAchievementsOperation::GetGameInfo(id) => {
				let result = refreshGameInfo(user, id);
				info!("[RetroAchievements] Refreshed game info for {}", id);
				
				Some(result)
			}
			
			RetroAchievementsOperation::GetUserProfile => {
				let result = refreshUserProfile(user);
				info!("[RetroAchievements] Refreshed user profile");
				
				Some(result)
			}
			
			RetroAchievementsOperation::GetUserProgress(state) => {
				let result = refreshUserProgress(user, state.clone());
				info!("[RetroAchievements] Refreshed user progress");
				
				Some(result)
			}
			
			RetroAchievementsOperation::SaveToFile => {
				match saveUserData(&user)
				{
					Err(e) => warn!("[RetroAchievements] Error saving user data: {:?}", e),
					Ok(_) => info!("[RetroAchievements] Saved user data"),
				}
				
				None
			}
		}
	};
}

fn refreshUserProfile(mut user: RetroAchievementsUser) -> RetroAchievementsResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		let ulid = user.ulid.clone();
		if let Ok(payload) = RetroAchievementsApi::getUserProfile(ulid.clone())
		{
			user.processUserProfile(&payload);
			
			if let Some(ulid) = ulid
			{
				if let Some(avatarPath) = user.avatar.clone()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: png!(ulid),
							group: Path_Avatars.into(),
							platform: RetroAchievementsApi::Platform.into(),
						}),
						
						operation: DataOperation::CacheImage(true),
						url: RetroAchievementsApi::buildMediaUrl(&avatarPath)
					});
				}
			}
		}
	}
	
	return RetroAchievementsResult
	{
		user,
		requests,
	};
}

fn refreshUserProgress(
	mut user: RetroAchievementsUser,
	mut state: RetroAchievementsProgressState
) -> RetroAchievementsResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		match RetroAchievementsApi::getUserCompletionProgress(user.ulid.clone(), Some(state.offset))
		{
			Err(e) => warn!("[RetroAchievements] Error retrieving user completion progress: {:?}", e),
			
			Ok(payload) => {
				// Update progress state
				state.received += payload.Count;
				state.offset += payload.Count;
				state.total = payload.Total;
				
				user.processUserCompletionProgress(&payload);
				
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
							operation: DataOperation::CacheImage(false),
							url: Some(url),
						});
					}
				}
			}
		}
	}
	
	return RetroAchievementsResult
	{
		user,
		requests,
	};
}

fn refreshGameInfo(mut user: RetroAchievementsUser, id: u64) -> RetroAchievementsResult
{
	let mut requests = vec![];
	
	if getRetroAchievementsAuth().is_ok_and(|a| a.isValid())
	{
		let ulid = match user.ulid.clone()
		{
			None => user.username.clone(),
			Some(ulid) => ulid,
		};
		
		match RetroAchievementsApi::getGameInfo(&ulid, id)
		{
			Err(e) => warn!("[RetroAchievements] Error getting game info for {}: {:?}", id, e),
			
			Ok(payload) => {
				match user.games.iter_mut()
					.find(|g| g.id == id)
				{
					None => user.games.push(payload.clone().into()),
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
							
							operation: DataOperation::CacheImage(false),
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
							
							operation: DataOperation::CacheImage(false),
							url: Some(url),
						});
					}
				}
			}
		}
	}
	
	return RetroAchievementsResult
	{
		user,
		requests,
	};
}
