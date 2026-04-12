use data::constants::{FileName_GameIcon, Path_Avatars, Path_Games};
use data::io::FileLocation;
use macros::{join, jpg, png};
use net::{DataOperation, DataRequest};
use tracing::{info, warn};
use crate::api::{EgsApi, KeyImageType};
use crate::data::io::saveUserData;
use crate::data::operation::EgsOperation;
use crate::data::result::EgsOperationResult;
use crate::data::user::EgsUser;
use crate::secure::getEpicGamesStoreAccountId;

pub async fn handleEgsOperation(user: EgsUser, dataOperation: DataOperation) -> Option<EgsOperationResult>
{
	return match dataOperation.try_into()
	{
		Err(_) => None,
		Ok(operation) => match operation
		{
			EgsOperation::GetAchievementProgress(productId) => {
				let result = refreshEgsAchievementProgress(user, &productId);
				info!("[EpicGamesStore] Refreshed achievements progress for {}", productId);
				
				Some(result)
			}
			
			EgsOperation::GetAchievementsList(sandboxId) => {
				let result = refreshEgsAchievementMetadata(user, &sandboxId);
				info!("[EpicGamesStore] Refreshed achievements metadata for {}", sandboxId);
				
				Some(result)
			}
			
			EgsOperation::GetPlayerProfile => {
				let result = refreshEgsPlayerProfile(user);
				info!("[EpicGamesStore] Refreshed user profile data");
				
				Some(result)
			}
			
			EgsOperation::GetPlayerProfilePrivate => {
				let result = refreshEgsPlayerProfilePrivate(user);
				info!("[EpicGamesStore] Refreshed user game list data");
				
				Some(result)
			}
			
			EgsOperation::SaveToFile => {
				match saveUserData(&user)
				{
					Err(e) => warn!("[EpicGamesStore] Error saving user data: {:?}", e),
					Ok(_) => info!("[EpicGamesStore] Saved user data"),
				}
			
				None
			}
		}
	};
}

fn refreshEgsAchievementMetadata(mut user: EgsUser, sandboxId: &String) -> EgsOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getAchievementMetadata(&accountId, sandboxId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store achievements metadata data: {:?}", e),
			Ok(payload) => {
				
				// Cache icons
				let group = join!(Path_Games, sandboxId.clone());
				let platform = EgsApi::Platform.to_lowercase();
				
				for container in payload.data.Achievement
					.productAchievementsRecordBySandbox.achievements.iter()
				{
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: container.achievement.unlockedIconId.clone(),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(container.achievement.unlockedIconLink.clone()),
					});
					
					requests.push(DataRequest
					{
						destination: Some(FileLocation
						{
							fileName: container.achievement.lockedIconId.clone(),
							group: group.clone(),
							platform: platform.clone(),
						}),
						operation: DataOperation::CacheImage(false),
						url: Some(container.achievement.lockedIconLink.clone()),
					});
					
					if let Some(game) = user.games.iter_mut()
						.find(|g| &g.sandboxId == sandboxId)
					{
						match game.achievements.iter_mut()
							.find(|a| a.id == container.achievement.name)
						{
							None => game.achievements.push((&container.achievement).into()),
							Some(chievo) => chievo.updateMetadata(&container.achievement),
						}
					}
				}
				
				if let Some(game) = user.games.iter_mut()
					.find(|g| &g.sandboxId == sandboxId)
				{
					game.achievementsCount = payload.data.Achievement.productAchievementsRecordBySandbox.totalAchievements;
					game.maxXp = payload.data.Achievement.productAchievementsRecordBySandbox.totalProductXP;
					game.platinumRarity = (payload.data.Achievement.productAchievementsRecordBySandbox.platinumRarity.percent * 10.0) as u64;
					game.productId = payload.data.Achievement.productAchievementsRecordBySandbox.productId.clone();
				}
			}
		}
	}
	
	return EgsOperationResult
	{
		user,
		requests,
	};
}

fn refreshEgsAchievementProgress(mut user: EgsUser, productId: &String) -> EgsOperationResult
{
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getAchievementProgress(&accountId, productId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store achievements progress data: {:?}", e),
			Ok(payload) => {
				if let Some(game) = user.games.iter_mut()
					.find(|g| &g.productId == productId)
				{
					game.currentXp = payload.data.PlayerProfile.playerProfile.productAchievements.data.totalXP;
					game.achievementsUnlocked = payload.data.PlayerProfile.playerProfile.productAchievements.data.totalUnlocked;
				}
				
				for achievement in payload.data.PlayerProfile.playerProfile
					.productAchievements.data.playerAchievements.iter()
				{
					if let Some(game) = user.games.iter_mut()
						.find(|g| &g.productId == productId)
					{
						match game.achievements.iter_mut()
							.find(|a| a.id == achievement.playerAchievement.achievementName)
						{
							None => game.achievements.push(achievement.into()),
							Some(chievo) => chievo.updateProgress(achievement),
						}
					}
				}
			}
		}
	}
	
	return user.into();
}

fn refreshEgsPlayerProfile(mut user: EgsUser) -> EgsOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getPlayerProfile(&accountId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store player profile data: {:?}", e),
			Ok(payload) => {
				// Cache icon
				requests.push(DataRequest
				{
					destination: Some(FileLocation
					{
						fileName: png!(accountId),
						group: Path_Avatars.into(),
						platform: EgsApi::Platform.to_lowercase(),
					}),
					operation: DataOperation::CacheImage(true),
					url: Some(payload.data.PlayerProfile.playerProfile.avatar.large.clone()),
				});
				
				user.updateProfile(payload);
			}
		}
	}
	
	return EgsOperationResult
	{
		user,
		requests,
	};
}

fn refreshEgsPlayerProfilePrivate(mut user: EgsUser) -> EgsOperationResult
{
	let mut requests = vec![];
	
	if let Ok(accountId) = getEpicGamesStoreAccountId()
	{
		match EgsApi::getPlayerProfilePrivate(&accountId)
		{
			Err(e) => warn!("Failed to refresh Epic Games Store private player profile data: {:?}", e),
			Ok(payload) => {
				//Cache game icons
				for summary in payload.data.PlayerProfile.playerProfile.achievementsSummaries.data.iter()
				{
					if let Some(keyImage) = summary.baseOfferForSandbox.keyImages.iter()
						.find(|ki| ki.r#type == KeyImageType::OfferImageWide)
					{
						requests.push(DataRequest
						{
							destination: Some(FileLocation
							{
								fileName: jpg!(FileName_GameIcon),
								group: join!(Path_Games, summary.sandboxId),
								platform: EgsApi::Platform.to_lowercase(),
							}),
							operation: DataOperation::CacheImage(false),
							url: Some(keyImage.url.clone())
						});
					}
				}
				
				user.updateProfilePrivate(payload);
			}
		}
	}
	
	return EgsOperationResult
	{
		user,
		requests,
	};
}
