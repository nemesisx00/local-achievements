use data::constants::{Path_Avatars, Path_Games};
use data::io::FileLocation;
use freya::prelude::spawn;
use macros::{join, jpgAlt};
use net::{DataOperation, DataRequest};
use tracing::{info, warn};
use crate::api::{BattleNetApi, BattleNetSettings, Starcraft2};
use crate::data::io::saveUserData;
use crate::data::operation::BattleNetOperation;
use crate::data::region::Region;
use crate::data::result::BattleNetOperationResult;
use crate::data::user::BattleNetUser;
use crate::secure::getBattleNetSession;

pub async fn handleBattleNetOperation(user: BattleNetUser, settings: BattleNetSettings, dataOperation: DataOperation) -> Option<BattleNetOperationResult>
{
	return match dataOperation.try_into()
	{
		Err(_) => None,
		Ok(operation) => match operation
		{
			BattleNetOperation::GetSc2PlayerAccount => {
				let result = refreshSc2PlayerAccount(user, settings).await;
				
				Some(result)
			}
			
			BattleNetOperation::GetSc2PlayerProfile => {
				let result = refreshSc2PlayerProfile(user, settings).await;
				
				Some(result)
			}
			
			BattleNetOperation::GetSc2StaticProfile => {
				let result = refreshSc2StaticProfile(user, settings).await;
				
				Some(result)
			}
			
			BattleNetOperation::GetUserInfo => {
				let result = refreshUserInfo(user, settings).await;
				
				Some(result)
			}
			
			BattleNetOperation::SaveToFile => {
				match saveUserData(&user)
				{
					Err(e) => warn!("[BattleNet] Error saving user data: {:?}", e),
					Ok(_) => info!("[BattleNet] Saved user data"),
				}
				
				None
			}
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

async fn refreshSc2PlayerAccount(mut user: BattleNetUser, settings: BattleNetSettings) -> BattleNetOperationResult
{
	let mut requests = vec![];
	
	let api = BattleNetApi::new(settings.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match Starcraft2::accountPlayer(
			&api,
			session,
			match user.starcraft2.clone()
			{
				None => settings.defaultRegion,
				Some(profile) => profile.region,
			},
			user.accountId
		).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing StarCraft II account data: {:?}", e),
			
			Ok(payload) => {
				let avatarUrl = payload.avatarUrl.clone();
				if user.starcraft2.is_none()
				{
					user.starcraft2 = Some(payload.into());
				}
				else if let Some(profile) = user.starcraft2.as_mut()
				{
					profile.updateAccount(payload);
				}
				
				info!("[BattleNet] Refreshed StarCraft II account data");
				
				// Cache the profile avatar
				if let Some(profile) = user.starcraft2.clone()
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
	
	return BattleNetOperationResult
	{
		user,
		requests,
	};
}

async fn refreshSc2PlayerProfile(mut user: BattleNetUser, settings: BattleNetSettings) -> BattleNetOperationResult
{
	let api = BattleNetApi::new(settings.clone());
	if let Ok(session) = getBattleNetSession()
	{
		let starcraft2 = user.starcraft2.clone();
		if let Some(profile) = starcraft2
		{
			match Starcraft2::profileProfile(&api, session, profile.region, profile.id).await
			{
				Err(e) => warn!("[BattleNet] Error refreshing StarCraft II profile: {:?}", e),
				
				Ok(payload) => {
					if let Some(profile) = user.starcraft2.as_mut()
					{
						profile.updateProfile(payload);
						info!("[BattleNet] Refreshed StarCraft II profile data");
					}
				}
			}
		}
	}
	
	return user.into();
}

async fn refreshSc2StaticProfile(mut user: BattleNetUser, settings: BattleNetSettings) -> BattleNetOperationResult
{
	let mut requests = vec![];
	
	let api = BattleNetApi::new(settings.clone());
	if let Ok(session) = getBattleNetSession()
	{
		let starcraft2 = user.starcraft2.clone();
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
					
					if let Some(profile) = user.starcraft2.as_mut()
					{
						profile.updateStatic(payload);
						info!("[BattleNet] Refreshed StarCraft II static data");
					}
				}
			}
		}
	}
	
	return BattleNetOperationResult
	{
		user,
		requests,
	};
}

async fn refreshUserInfo(mut user: BattleNetUser, settings: BattleNetSettings) -> BattleNetOperationResult
{
	let api = BattleNetApi::new(settings.clone());
	if let Ok(session) = getBattleNetSession()
	{
		match api.getUserInfo(
			session,
			match user.starcraft2.clone()
			{
				None => settings.defaultRegion,
				Some(profile) => profile.region
			}
		).await
		{
			Err(e) => warn!("[BattleNet] Error refreshing user info: {:?}", e),
			
			Ok(payload) => {
				user.updateUserInfo(payload);
				info!("[BattleNet] User info refreshed");
			}
		}
	}
	
	return user.into();
}
