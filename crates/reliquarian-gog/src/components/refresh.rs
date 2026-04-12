use anyhow::Result;
use data::constants::{Icon_Locked, Path_Avatars, Path_Games};
use data::io::FileLocation;
use freya::prelude::spawn;
use macros::{join, jpg, jpgAlt};
use net::{DataOperation, DataRequest};
use tracing::{error, info, warn};
use crate::api::{GogApi, GogSession};
use crate::data::achievement::GogAchievement;
use crate::data::io::saveUserData;
use crate::data::operation::GogOperation;
use crate::data::result::GogOperationResult;
use crate::data::user::GogUser;
use crate::secure::{getGogSession, removeGogSession, setGogSession};

pub fn exchangeCode(url: String)
{
	if let Some(code) = GogApi::parseAuthCodeUrl(url)
	{
		match GogApi::exchangeCodeForToken(code)
		{
			Err(e) => error!("Failed to exchange authorization code for GOG access token: {:?}", e),
			Ok(session) => {
				_ = setGogSession(session);
				info!("GOG access token acquired");
			},
		}
	}
}

pub async fn handleGogOperation(user: GogUser, dataOperation: DataOperation) -> Option<GogOperationResult>
{
	return match dataOperation.try_into()
	{
		Err(_) => None,
		Ok(operation) => match operation
		{
			GogOperation::RefreshSession => {
				_ = refreshSession();
				info!("[GOG] Refreshed user session");
				
				None
			}
			
			GogOperation::GetAchievements(id) => match getGogSession()
			{
				Err(_) => None,
				Ok(session) => {
					let result = refreshGameAchievements(user, session, id);
					info!("[GOG] Refreshed achievements for game id {}", id);
					
					Some(result)
				}
			}
			
			GogOperation::GetFilteredProducts(page) => match getGogSession()
			{
				Err(_) => None,
				Ok(session) => {
					let result = refreshGameList(user, session, page);
					info!("[GOG] Refreshed game list page {}", match page
					{
						None => 1,
						Some(p) => p,
					});
					
					Some(result)
				}
			}
			
			GogOperation::GetUserInfo => match getGogSession()
			{
				Err(_) => None,
				Ok(session) => {
					let result = refreshUserInfo(user, session);
					info!("[GOG] Refreshed user info");
					
					Some(result)
				}
			}
			
			GogOperation::SaveToFile => {
				match saveUserData(&user)
				{
					Err(e) => warn!("[GOG] Error saving user data: {:?}", e),
					Ok(_) => info!("[GOG] Saved user data"),
				}
				
				None
			}
		}
	};
}

fn refreshGameAchievements(mut user: GogUser, session: GogSession, gameId: u64) -> GogOperationResult
{
	let mut requests = vec![];
	if let Ok(payload) = GogApi::getAchievements(
		&session,
		user.id.clone(),
		gameId
	)
	{
		let group = join!(Path_Games, gameId.to_string());
		let platform = GogApi::Platform.to_lowercase();
		
		let mut achievements = vec![];
		for metadata in payload.items
		{
			requests.push(DataRequest
			{
				destination: Some(FileLocation
				{
					fileName: jpgAlt!(metadata.achievement_id.clone(), Icon_Locked),
					group: group.clone(),
					platform: platform.clone(),
				}),
				operation: DataOperation::CacheImage(false),
				url: Some(metadata.image_url_locked.clone()),
				..Default::default()
			});
			
			requests.push(DataRequest
			{
				destination: Some(FileLocation
				{
					fileName: jpg!(metadata.achievement_id.clone()),
					group: group.clone(),
					platform: platform.clone(),
				}),
				operation: DataOperation::CacheImage(false),
				url: Some(metadata.image_url_unlocked.clone()),
			});
			
			achievements.push(GogAchievement::from(metadata));
		}
		
		user.updateGameAchievements(gameId, achievements);
	}
	
	return GogOperationResult
	{
		user,
		requests
	};
}

fn refreshSession() -> Result<()>
{
	let session = getGogSession()?;
	if session.hasExpired()
	{
		info!("[GOG] Session has expired; Attempting refresh");
		match GogApi::refreshAccessToken(session.refreshToken().clone())
		{
			Err(e) => {
				warn!("[GOG] Session invalidated; Error refreshing session: {:?}", e);
				match removeGogSession()
				{
					Err(e) => warn!("[GOG] Failed to remove session file: {:?}", e),
					Ok(_) => info!("[GOG] Removed expired session file"),
				}
			},
			
			Ok(_) => info!("[GOG] Session refreshed successfully"),
		}
	}
	
	return Ok(());
}

fn refreshUserInfo(mut user: GogUser, session: GogSession) -> GogOperationResult
{
	let mut request = None;
	if let Ok(userInfo) = GogApi::getUserInfo(&session)
	{
		let avatarUrl = userInfo.avatar.large.clone();
		user.updateUserInfo(userInfo);
		
		request = Some(DataRequest
		{
			destination: Some(FileLocation
			{
				fileName: jpg!(user.id),
				group: Path_Avatars.into(),
				platform: GogApi::Platform.to_lowercase(),
			}),
			operation: DataOperation::CacheImage(true),
			url: Some(avatarUrl),
		});
	}
	
	return GogOperationResult
	{
		user,
		requests: match request
		{
			None => vec![],
			Some(request) => vec![request],
		},
	};
}

fn refreshGameList(mut user: GogUser, session: GogSession, page: Option<u64>) -> GogOperationResult
{
	let mut requests = vec![];
	
	if let Ok(payload) = GogApi::getFilteredProducts(&session, page)
	{
		user.updateGames(payload.products.clone());
		
		let lastPage = payload.page >= payload.totalPages;
		
		if !lastPage
		{
			requests.push(GogOperation::GetFilteredProducts(Some(payload.page + 1)).into());
		}
		else
		{
			requests.push(GogOperation::SaveToFile.into());
		}
		
		for product in payload.products
		{
			requests.push(DataRequest
			{
				destination: Some(GogApi::constructGameIconLocation(product.id)),
				operation: DataOperation::CacheImage(false),
				url: Some(GogApi::constructGameIconUrl(product.image.clone())),
				..Default::default()
			});
		}
	}
	
	return GogOperationResult
	{
		user,
		requests,
	};
}

pub fn openBrowserForAuthorization()
{
	spawn(async move {
		match GogApi::openBrowserToAuthorize().await
		{
			Err(e) => error!("[GOG] Failed to open login url in browser: {:?}", e),
			Ok(_) => info!("[GOG] Opened login url in browser"),
		}
	});
}
