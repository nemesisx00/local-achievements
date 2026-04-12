use tracing::{info, warn};
use crate::api::api::Rpcs3Api;
use crate::data::io::saveUserData;
use crate::data::settings::Rpcs3Settings;
use crate::data::user::Rpcs3User;

pub async fn refreshUserData(mut user: Rpcs3User, settings: Rpcs3Settings) -> Rpcs3User
{
	if !settings.appDataDirectory.is_empty()
	{
		let api = Rpcs3Api::from(settings.clone());
		
		match api.generateGameList()
		{
			Err(e) => warn!("[RPCS3] Error generating game list: {:?}", e),
			
			Ok(games) => {
				for npCommId in games.iter().cloned().map(|g| g.npCommId)
				{
					if let Err(e) = api.cacheGameIcons(&npCommId)
					{
						warn!("[RPCS3] Error caching the icons for {}: {:?}", npCommId, e);
					}
				}
				info!("[RPCS3] Cached game icons");
				//notification "Icons Cached"
				
				user.updateGamesList(games);
				info!("[RPCS3] Refreshed game list");
				//notification "Trophy Data Loaded"
			}
		}
		
		match api.getRpcnId()
		{
			Err(e) => warn!("[RPCS3] Failed to read RPCN ID: {:?}", e),
			
			Ok(rpcnId) => {
				user.name = rpcnId;
				info!("[RPCS3] Refreshed RPCN ID");
				//notification "RPCN ID Loaded"
			},
		}
		
		user.accountId = settings.accountId;
		
		match saveUserData(&user)
		{
			Err(e) => warn!("[RPCS3] Error saving user data: {:?}", e),
			Ok(_) => info!("[RPCS3] Saved user data"),
		}
	}
	
	return user;
}
