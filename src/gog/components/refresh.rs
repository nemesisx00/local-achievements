use anyhow::Result;
use freya::prelude::spawn;
use tracing::{error, info, warn};
use crate::constants::Icon_Locked;
use crate::data::secure::{getGogSession, removeGogSession, setGogSession};
use crate::data::AppData;
use crate::gog::{GogAchievement, GogApi, GogSession};
use crate::io::{Path_Avatars, Path_Games};
use crate::{join, jpg, jpgAlt};
use crate::net::limiter::request::{DataOperation, FileLocation, GogOperation,
	RequestData};

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

pub fn refreshGameAchievements(mut appData: AppData, session: GogSession, gameId: u64) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	if let Ok(payload) = GogApi::getAchievements(
		&session,
		appData.user.gog.id.clone(),
		gameId
	)
	{
		let group = join!(Path_Games, gameId.to_string());
		let platform = GogApi::Platform.to_lowercase();
		
		let mut achievements = vec![];
		for metadata in payload.items
		{
			requests.push(RequestData
			{
				destination: Some(FileLocation
				{
					fileName: jpgAlt!(metadata.achievement_id.clone(), Icon_Locked),
					group: group.clone(),
					platform: platform.clone(),
				}),
				operation: DataOperation::CacheImage,
				url: Some(metadata.image_url_locked.clone()),
				..Default::default()
			});
			
			requests.push(RequestData
			{
				destination: Some(FileLocation
				{
					fileName: jpg!(metadata.achievement_id.clone()),
					group: group.clone(),
					platform: platform.clone(),
				}),
				operation: DataOperation::CacheImage,
				url: Some(metadata.image_url_unlocked.clone()),
			});
			
			achievements.push(GogAchievement::from(metadata));
		}
		
		appData.user.gog.updateGameAchievements(gameId, achievements);
	}
	
	return (appData, requests);
}

pub fn refreshSession() -> Result<()>
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

pub fn refreshUserInfo(mut appData: AppData, session: GogSession) -> (AppData, Option<RequestData>)
{
	let mut request = None;
	if let Ok(userInfo) = GogApi::getUserInfo(&session)
	{
		let avatarUrl = userInfo.avatar.large.clone();
		appData.user.gog.updateUserInfo(userInfo);
		
		request = Some(RequestData
		{
			destination: Some(FileLocation
			{
				fileName: jpg!(appData.user.gog.id),
				group: Path_Avatars.into(),
				platform: GogApi::Platform.to_lowercase(),
			}),
			operation: DataOperation::CacheImage,
			url: Some(avatarUrl),
		});
	}
	
	return (appData, request);
}

pub fn refreshGameList(mut appData: AppData, session: GogSession, page: Option<u64>) -> (AppData, Vec<RequestData>)
{
	let mut requests = vec![];
	
	if let Ok(payload) = GogApi::getFilteredProducts(&session, page)
	{
		appData.user.gog.updateGames(payload.products.clone());
		
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
			requests.push(RequestData
			{
				destination: Some(GogApi::constructGameIconLocation(product.id)),
				operation: DataOperation::CacheImage,
				url: Some(GogApi::constructGameIconUrl(product.image.clone())),
				..Default::default()
			});
		}
	}
	
	return (appData, requests);
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
