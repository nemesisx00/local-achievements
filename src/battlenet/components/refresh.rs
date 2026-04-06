use freya::prelude::spawn;
use tracing::{info, warn};
use crate::battlenet::data::region::Region;
use crate::{join, jpgAlt};
use crate::battlenet::BattleNetSettings;
use crate::battlenet::platform::api::BattleNetApi;
use crate::battlenet::platform::starcraft2::Starcraft2;
use crate::data::AppData;
use crate::data::secure::getBattleNetSession;
use crate::io::{Path_Avatars, Path_Games, saveUserData_BattleNet};
use crate::net::limiter::request::{BattleNetOperation, DataOperation,
	DataOperationResult, FileLocation, DataRequest};

pub async fn handleDataOperation(appData: AppData, operation: BattleNetOperation) -> Option<DataOperationResult>
{
	return match operation
	{
		BattleNetOperation::GetSc2PlayerAccount => {
			let result = refreshSc2PlayerAccount(appData).await;
			
			Some(result)
		}
		
		BattleNetOperation::GetSc2PlayerProfile => {
			let result = refreshSc2PlayerProfile(appData).await;
			
			Some(result)
		}
		
		BattleNetOperation::GetSc2StaticProfile => {
			let result = refreshSc2StaticProfile(appData).await;
			
			Some(result)
		}
		
		BattleNetOperation::GetUserInfo => {
			let result = refreshUserInfo(appData).await;
			
			Some(result)
		}
		
		BattleNetOperation::SaveToFile => {
			match saveUserData_BattleNet(&appData.user.battleNet)
			{
				Err(e) => warn!("[BattleNet] Error saving user data: {:?}", e),
				Ok(_) => info!("[BattleNet] Saved user data"),
			}
			
			None
		}
	};
}

pub fn openBrowserForAuthorization(settings: BattleNetSettings, region: Region)
{
	spawn(async move {
		let api = BattleNetApi::new(settings);
		_ = api.authorize(region).await;
	});
}

async fn refreshSc2PlayerAccount(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match Starcraft2::accountPlayer(
			&api,
			session,
			match appData.user.battleNet.starcraft2.clone()
			{
				None => appData.platform.battleNet.defaultRegion,
				Some(profile) => profile.region,
			},
			appData.user.battleNet.accountId
		).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing StarCraft II account data: {:?}", e),
			
			Ok(payload) => {
				let avatarUrl = payload.avatarUrl.clone();
				if appData.user.battleNet.starcraft2.is_none()
				{
					appData.user.battleNet.starcraft2 = Some(payload.into());
				}
				else if let Some(profile) = appData.user.battleNet.starcraft2.as_mut()
				{
					profile.updateAccount(payload);
				}
				
				info!("[BattleNet] Refreshed StarCraft II account data");
				
				// Cache the profile avatar
				if let Some(profile) = appData.user.battleNet.starcraft2.clone()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: jpgAlt!(Starcraft2::GamePrefix, profile.id),
							group: Path_Avatars.into(),
							platform: BattleNetApi::Platform.to_lowercase(),
						}),
						operation: DataOperation::CacheImage(true),
						url: Some(avatarUrl)
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

async fn refreshSc2PlayerProfile(mut appData: AppData) -> DataOperationResult
{
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		let starcraft2 = appData.user.battleNet.starcraft2.clone();
		if let Some(profile) = starcraft2
		{
			match Starcraft2::profileProfile(&api, session, profile.region, profile.id).await
			{
				Err(e) => warn!("[BattleNet] Error refreshing StarCraft II profile: {:?}", e),
				
				Ok(payload) => {
					if let Some(profile) = appData.user.battleNet.starcraft2.as_mut()
					{
						profile.updateProfile(payload);
						info!("[BattleNet] Refreshed StarCraft II profile data");
					}
				}
			}
		}
	}
	
	return appData.into();
}

async fn refreshSc2StaticProfile(mut appData: AppData) -> DataOperationResult
{
	let mut requests = vec![];
	
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		let starcraft2 = appData.user.battleNet.starcraft2.clone();
		if let Some(profile) = starcraft2
		{
			match Starcraft2::profileStatic(&api, session, profile.region).await
			{
				Err(e) => warn!("[BattleNet] Error refreshing StarCraft II static data: {:?}", e),
				
				Ok(payload) => {
					let group = join!(Path_Games, Starcraft2::GamePrefix);
					
					// Cache achievement icons
					for achievement in payload.achievements.iter()
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: jpgAlt!(BattleNetApi::AchievementPrefix, achievement.id),
								group: group.clone(),
								platform: BattleNetApi::Platform.to_lowercase(),
							}),
							operation: DataOperation::CacheImage(false),
							url: Some(achievement.imageUrl.clone())
						});
					}
					
					// Cache reward icons
					
					if let Some(profile) = appData.user.battleNet.starcraft2.as_mut()
					{
						profile.updateStatic(payload);
						info!("[BattleNet] Refreshed StarCraft II static data");
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

async fn refreshUserInfo(mut appData: AppData) -> DataOperationResult
{
	let api = BattleNetApi::new(appData.platform.battleNet.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match api.getUserInfo(
			session,
			match appData.user.battleNet.starcraft2.clone()
			{
				None => appData.platform.battleNet.defaultRegion,
				Some(profile) => profile.region
			}
		).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing user info: {:?}", e),
			
			Ok(payload) => {
				appData.user.battleNet.updateUserInfo(payload);
				info!("[BattleNet] User info refreshed");
			}
		}
	}
	
	return appData.into();
}
