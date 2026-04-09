use tracing::warn;
use crate::data::{AppState, PlatformState, UserState};
use crate::io::{loadAppSettings, loadSettings_BattleNet,
	loadSettings_Rpcs3, loadUserData_BattleNet, loadUserData_BattleNet_lossy,
	loadUserData_EpicGamesStore, loadUserData_EpicGamesStore_lossy,
	loadUserData_Gog, loadUserData_Gog_lossy, loadUserData_RetroAchievements,
	loadUserData_RetroAchievements_lossy, loadUserData_Rpcs3,
	loadUserData_Rpcs3_lossy, loadUserData_Steam, loadUserData_Steam_lossy};

#[derive(Clone, Debug)]
pub struct AppData
{
	pub app: AppState,
	pub platform: PlatformState,
	pub user: UserState,
}

impl Default for AppData
{
	fn default() -> Self
	{
		return Self
		{
			app: Default::default(),
			platform: Default::default(),
			user: Default::default(),
		}
			.initializeAppSettings()
			.initializeBattleNet()
			.initializeEpicGamesStore()
			.initializeGog()
			.initializeRetroAchievements()
			.initializeRpcs3()
			.initializeSteam();
	}
}

impl AppData
{
	fn initializeAppSettings(mut self) -> Self
	{
		if let Ok(settings) = loadAppSettings()
		{
			self.app.settings = settings;
		}
		
		return self;
	}
	
	fn initializeBattleNet(mut self) -> Self
	{
		if let Ok(auth) = loadSettings_BattleNet()
		{
			self.platform.battleNet = auth;
		}
		
		match loadUserData_BattleNet()
		{
			Err(e) => {
				warn!("Failed loading Battle.Net user data: {:?}", e);
				warn!("Attempting Battle.Net user data lossy load");
				if let Ok(user) = loadUserData_BattleNet_lossy()
				{
					self.user.battleNet = user;
				}
			},
			Ok(user) => self.user.battleNet = user,
		}
		
		return self;
	}
	
	fn initializeEpicGamesStore(mut self) -> Self
	{
		match loadUserData_EpicGamesStore()
		{
			Err(e) => {
				warn!("Failed loading Epic Games Store user data: {:?}", e);
				warn!("Attempting Epic Games Store user data lossy load");
				if let Ok(user) = loadUserData_EpicGamesStore_lossy()
				{
					self.user.egs = user;
				}
			},
			Ok(user) => self.user.egs = user,
		}
		
		return self;
	}
	
	fn initializeGog(mut self) -> Self
	{
		match loadUserData_Gog()
		{
			Err(e) => {
				warn!("Failed loading GOG user data: {:?}", e);
				warn!("Attempting GOG user data lossy load");
				if let Ok(user) = loadUserData_Gog_lossy()
				{
					self.user.gog = user;
				}
			}
			Ok(user) => self.user.gog = user,
		}
		
		return self;
	}
	
	fn initializeRetroAchievements(mut self) -> Self
	{
		match loadUserData_RetroAchievements()
		{
			Err(e) => {
				warn!("Failed loading RetroAchievements user data: {:?}", e);
				warn!("Attempting RetroAchievements user data lossy load");
				if let Ok(user) = loadUserData_RetroAchievements_lossy()
				{
					self.user.retroAchievements = user;
				}
			},
			Ok(user) => self.user.retroAchievements = user,
		}
		
		return self;
	}
	
	fn initializeRpcs3(mut self) -> Self
	{
		if let Ok(settings) = loadSettings_Rpcs3()
		{
			self.platform.rpcs3 = settings;
		}
		
		match loadUserData_Rpcs3()
		{
			Err(e) => {
				warn!("Failed loading RPCS3 user data: {:?}", e);
				warn!("Attempting RPCS3 user data lossy load");
				if let Ok(user) = loadUserData_Rpcs3_lossy()
				{
					self.user.rpcs3 = user;
				}
			},
			Ok(user) => self.user.rpcs3 = user,
		}
		
		return self;
	}
	
	fn initializeSteam(mut self) -> Self
	{
		match loadUserData_Steam()
		{
			Err(e) => {
				warn!("Failed loading Steam user data: {:?}", e);
				warn!("Attempting Steam user data lossy load");
				if let Ok(user) = loadUserData_Steam_lossy()
				{
					self.user.steam = user;
				}
			},
			Ok(user) => self.user.steam = user,
		}
		
		return self;
	}
}
