use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::Path;
use anyhow::{Context, Result};
use path_slash::PathExt;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::constants::Icon_Locked;
use crate::data::SteamGame;
use crate::io::{getImagePath, Filename_GameIcon, Path_Avatars, Path_Games};
use crate::{error, join, jpg, jpgAlt};
use crate::platforms::steam::data::{AuthData, GameAchievement,
	Payload_GetGlobalPercentages, Payload_GetOwnedGames,
	Payload_GetPlayerAchievements, Payload_GetPlayerSummaries,
	Payload_GetRecentlyPlayedGames, Payload_GetSchemaForGame};
use crate::platforms::util::cacheImageIfNotExists;

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthData,
	pub client: Client,
}

impl From<AuthData> for Api
{
	fn from(value: AuthData) -> Self
	{
		return Self
		{
			auth: value,
			..Default::default()
		};
	}
}

impl Api
{
	pub const Platform: &str = "Steam";
	
	const Protocol: &str = "https://";
	const Domain: &str = "api.steampowered.com/";
	
	const Service_Player: &str = "IPlayerService";
	const Service_User: &str = "ISteamUser";
	const Service_UserStats: &str = "ISteamUserStats";
	
	const Endpoint_GetGlobalAchievementPercentagesForApp: &str = "GetGlobalAchievementPercentagesForApp/v0002";
	const Endpoint_GetOwnedGames: &str = "GetOwnedGames/v0001";
	const Endpoint_GetPlayerAchievements: &str = "GetPlayerAchievements/v0001";
	const Endpoint_GetPlayerSummaries: &str = "GetPlayerSummaries/v0002";
	const Endpoint_GetRecentlyPlayedGames: &str = "GetRecentlyPlayedGames/v0001";
	const Endpoint_GetSchemaForGame: &str = "GetSchemaForGame/v0002";
	
	const Parameter_AppId: &str = "appid";
	const Parameter_Format: &str = "format";
	const Parameter_GameId: &str = "gameid";
	const Parameter_IncludeAppInfo: &str = "include_appinfo";
	const Parameter_IncludePlayedFreeGames: &str = "include_played_free_games";
	const Parameter_Key: &str = "key";
	const Parameter_Language: &str = "l";
	const Parameter_SteamId: &str = "steamid";
	const Parameter_SteamIds: &str = "steamids";
	
	const Format_Json: &str = "json";
	
	#[allow(unused)]
	const Value_False: &str = "0";
	const Value_True: &str = "1";
	
	const Replace_AppId: &str = "{appid}";
	const Replace_Hash: &str = "{hash}";
	const Replace_Size: &str = "{size}";
	
	//const IconUrl_Achievement: &str = "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/apps/{appid}/{hash}.jpg";
	const IconUrl_Game: &str = "https://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg";
	
	const AvatarUrl: &str = "https://avatars.steamstatic.com/{hash}{size}.jpg";
	const AvatarUrl_ReplaceMedium: &str = "_medium";
	const AvatarUrl_ReplaceFull: &str = "_full";
	
	/**
	Retrieve and cache the icon images for a list of `achievements`.
	
	---
	
	Parameter | Description
	:--|:--
	appId | The app id to which these achievements belong.
	achievements | The list of achievements for which to retrieve icon images.
	force | If `TRUE`, retrieve all images and overwrite the cache. Otherwise, only retrieve non-cached images.
	
	---
	
	#### Returns
	
	If any icons result in an error, the list of all games for which no icon
	could be retrieved is returned. Otherwise, returns `NONE`.
	*/
	pub async fn cacheAchievementsIcons(&self, appId: usize, achievements: &Vec<GameAchievement>, force: bool) -> Result<()>
	{
		for achievement in achievements
		{
			let group = join!(Path_Games, appId);
			let filename = jpg!(achievement.name);
			let filenameLocked = jpgAlt!(achievement.name, Icon_Locked);
			let platform = Self::Platform.into();
			
			if let Some(path) = getImagePath(&platform, &group, &filename)
			{
				cacheImageIfNotExists(
					&self.client,
					&achievement.icon,
					&path,
					&platform,
					&group,
					&filename,
					force
				)
					.await?;
			}
			
			if let Some(path) = getImagePath(&platform, &group, &filenameLocked)
			{
				cacheImageIfNotExists(
					&self.client,
					&achievement.icongray,
					&path,
					&platform,
					&group,
					&filenameLocked,
					force
				)
					.await?;
			}
		}
		
		return Ok(());
	}
	
	/**
	Retrieve and cache the icon images for a list of `games`.
	
	---
	
	Parameter | Description
	:--|:--
	games | The list of games for which to retrieve icon images.
	force | If `TRUE`, retrieve all images and overwrite the cache. Otherwise, only retrieve non-cached images.
	
	---
	
	#### Returns
	
	If any icons result in an error, the list of all games for which no icon
	could be retrieved is returned. Otherwise, returns `NONE`.
	*/
	pub async fn cacheGameIcons(&self, games: &Vec<SteamGame>, force: bool) -> Option<Vec<SteamGame>>
	{
		let mut failed = vec![];
		
		let platform = Self::Platform.into();
		let fileName = jpg!(Filename_GameIcon);
		for game in games.iter()
		{
			let group = join!(Path_Games, game.id);
			
			if let Some(path) = getImagePath(&platform, &group, &fileName)
			{
				let url = Self::IconUrl_Game
					.replace(Self::Replace_AppId, game.id.to_string().as_str())
					.replace(Self::Replace_Hash, &game.iconHash);
				
				if let Err(_) = cacheImageIfNotExists(
						&self.client,
						&url,
						&path,
						&platform,
						&group,
						&fileName,
						force
					).await
				{
					failed.push(game.clone());
				}
			}
		}
		
		return match failed.is_empty()
		{
			true => None,
			false => Some(failed),
		}
	}
	
	/**
	Retrieve and cache all three sizes of avatar image for the given `steamId` and `hash`.
	
	---
	
	Parameter | Description
	:--|:--
	steamId | The 64-bit Steam ID identifying the user whose avatar images are being retrieved.
	hash | The hash value used to build the URL for retrieving the avatar images.
	force | If `TRUE`, retrieve all the images and overwrite the cache. Otherwise, only retrieve non-cached images.
	*/
	pub async fn cacheProfileAvatar(&self, steamId: &String, hash: &String, force: bool) -> Result<()>
	{
		let mut nameMod = String::default();
		for i in 0..3
		{
			match i
			{
				1 => nameMod = Self::AvatarUrl_ReplaceMedium.into(),
				2 => nameMod = Self::AvatarUrl_ReplaceFull.into(),
				_ => {},
			}
			
			let url = Self::AvatarUrl
				.replace(Self::Replace_Hash, hash.as_str())
				.replace(Self::Replace_Size, nameMod.as_str());
			
			let filename = format!("{}{}.jpg", steamId, nameMod);
			let group = Path_Avatars.into();
			let platform = Self::Platform.into();
			
			if let Some(path) = getImagePath(&platform, &group, &filename)
			{
				cacheImageIfNotExists(
					&self.client,
					&url,
					&path,
					&platform,
					&group,
					&filename,
					force
				).await?;
			}
		}
		
		return Ok(());
	}
	
	/**
	Get the global completion percentages of each achievment for an individual game.
	
	---
	
	# [GetGlobalAchievementPercentagesForApp (v0002)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetGlobalAchievementPercentagesForApp_.28v0002.29)
	
	Returns on global achievements overview of a specific game in percentages.
	
	Example URL:
	`http://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?gameid=440&format=xml`
	
	---
	
	## Arguments
	
	Name | Description
	:--|:--
	gameid | AppID of the game you want the news of.
	format | Output format. json (default), xml or vdf.
	*/
	pub async fn getGlobalPercentages(&self, appId: usize) -> Result<Payload_GetGlobalPercentages>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.remove(Self::Parameter_Key);
			parameters.remove(Self::Parameter_SteamId);
			parameters.insert(Self::Parameter_GameId.into(), appId.to_string());
			
			if let Some(url) = self.buildUrl(
				Self::Service_UserStats,
				Self::Endpoint_GetGlobalAchievementPercentagesForApp
			)
			{
				return Ok(self.get::<Payload_GetGlobalPercentages>(&url, &parameters).await
					.context(format!(
						"Error retrieving list of global percentages from Steam Web API for Game ID {}",
						appId
					))?);
			}
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	Call the GetOwnedGames endpoint to retrieve the current user's list of owned
	(or played free) games.
	
	---
	
	# [GetOwnedGames (v0001)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetOwnedGames_.28v0001.29)
	
	GetOwnedGames returns a list of games a player owns along with some playtime
	information, if the profile is publicly visible. Private, friends-only, and
	other privacy settings are not supported unless you are asking for your own
	personal details (ie the WebAPI key you are using is linked to the steamid
	you are requesting).
	
	Example URL:
	`http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key=XXXXXXXXXXXXXXXXX&steamid=76561197960434622&format=json`
	
	---
	
	## Arguments
	
	Name | Description
	:--|:--
	steamid | The SteamID of the account.
	include_appinfo | Include game name and logo information in the output. The default is to return appids only.
	include_played_free_games | By default, free games like Team Fortress 2 are excluded (as technically everyone owns them). If include_played_free_games is set, they will be returned if the player has played them at some point. This is the same behavior as the games list on the Steam Community.
	format | Output format. json (default), xml or vdf.
	appids_filter | You can optionally filter the list to a set of appids. Note that these cannot be passed as a URL parameter, instead you must use the JSON format described in Steam_Web_API#Calling_Service_interfaces. The expected input is an array of integers (in JSON: "appids_filter: [ 440, 500, 550 ]" )
	
	---
	
	## Result layout
	
	- game_count: The total number of games the user owns (including free games they've played, if include_played_free_games was passed)
	- games: A games array, with the following contents (note that if "include_appinfo" was not passed in the request, only appid, playtime_2weeks, and playtime_forever will be returned)
		- appid: Unique identifier for the game
		- name: The name of the game
		- playtime_2weeks: The total number of minutes played in the last 2 weeks
		- playtime_forever: The total number of minutes played "on record", since Steam began tracking total playtime in early 2009.
		- img_icon_url, img_logo_url: These are the filenames of various images for the game. To construct the URL to the image, use this format: http://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg. For example, the TF2 logo is returned as "07385eb55b5ba974aebbe74d3c99626bda7920b8", which maps to the URL: [1]
		- has_community_visible_stats: Indicates there is a stats page with achievements or other game stats available for this game. The uniform URL for accessing this data is http://steamcommunity.com/profiles/{steamid}/stats/{appid}. For example, Robin's TF2 stats can be found at: http://steamcommunity.com/profiles/76561197960435530/stats/440. You may notice that clicking this link will actually redirect to a vanity URL like /id/robinwalker/stats/TF2
	*/
	pub async fn getOwnedGames(&self) -> Result<Payload_GetOwnedGames>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.insert(Self::Parameter_IncludeAppInfo.into(), Self::Value_True.into());
			parameters.insert(Self::Parameter_IncludePlayedFreeGames.into(), Self::Value_True.into());
			
			if let Some(url) = self.buildUrl(
				Self::Service_Player,
				Self::Endpoint_GetOwnedGames
			)
			{
				return Ok(self.get::<Payload_GetOwnedGames>(&url, &parameters).await
					.context(format!(
						"Error retrieving list of owned games from Steam Web API for Steam ID {}",
						self.auth.id
					))?);
			}
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	Call the GetPlayerAchievements endpoint to retrieve the current user's
	achievement information for the given app id.
	
	# [GetPlayerAchievements (v0001)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetPlayerAchievements_.28v0001.29)
	
	Returns a list of achievements for this user by app id
	
	Example URL:
	`http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?appid=440&key=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX&steamid=76561197972495328`
	
	---
	
	## Arguments
	
	Name | Description
	:--|:--
	steamid | 64 bit Steam ID for which to return the achievement list.
	appid | The ID of the game you're requesting
	l | (Optional) Language. If specified, it will return language data for the requested language.
	
	---
	
	## Result Data
	
	A list of achievements.
	
	Name | Description
	:--|:--
	apiname | The API name of the achievement
	achieved | Whether or not the achievement has been completed.
	unlocktime | Date when the achievement was unlocked.
	name | **Optional** Localized achievement name
	description | **Optional** Localized description of the achievement
	*/
	pub async fn getPlayerAchievements(&self, appId: usize, language: &String) -> Result<Payload_GetPlayerAchievements>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.insert(Self::Parameter_AppId.into(), appId.to_string());
			parameters.insert(Self::Parameter_Language.into(), language.to_owned());
			
			if let Some(url) = self.buildUrl(
				Self::Service_UserStats,
				Self::Endpoint_GetPlayerAchievements
			)
			{
				return Ok(self.get::<Payload_GetPlayerAchievements>(&url, &parameters).await
					.context(format!(
						"Error retrieving the list of achievements from Stema Web API for App ID {}",
						appId
					))?);
			}
		}
		
		return Err(error!(ErrorKind::InvalidData));
	}
	
	/**
	Call the GetPlayerSummaries endpoint to retrieve the current user's profile
	information.
	
	---
	
	# [GetPlayerSummaries (v0002)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetPlayerSummaries_.28v0002.29)
	
	Returns basic profile information for a list of 64-bit Steam IDs.
	
	Example URL:
	`http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key=XXXXXXXXXXXXXXXXXXXXXXX&steamids=76561197960435530`
	(This will show Robin Walker's profile information.)
	
	---
	
	### Arguments
	
	Name | Description
	:--|:--
	steamids | Comma-delimited list of 64 bit Steam IDs to return profile information for. Up to 100 Steam IDs can be requested.
	format | Output format. json (default), xml or vdf.
	
	---
	
	### Return Value
	
	Some data associated with a Steam account may be hidden if the user has their profile visibility set to "Friends Only" or "Private". In that case, only public data will be returned.
	
	#### Public Data
	
	Name | Description
	:--|:--
	steamid | 64bit SteamID of the user
	personaname | The player's persona name (display name)
	profileurl | The full URL of the player's Steam Community profile.
	avatar | The full URL of the player's 32x32px avatar. If the user hasn't configured an avatar, this will be the default ? avatar.
	avatarmedium | The full URL of the player's 64x64px avatar. If the user hasn't configured an avatar, this will be the default ? avatar.
	avatarfull | The full URL of the player's 184x184px avatar. If the user hasn't configured an avatar, this will be the default ? avatar.
	personastate | The user's current status. 0 - Offline, 1 - Online, 2 - Busy, 3 - Away, 4 - Snooze, 5 - looking to trade, 6 - looking to play. If the player's profile is private, this will always be "0", except is the user has set their status to looking to trade or looking to play, because a bug makes those status appear even if the profile is private.
	communityvisibilitystate | This represents whether the profile is visible or not, and if it is visible, why you are allowed to see it. Note that because this WebAPI does not use authentication, there are only two possible values returned: 1 - the profile is not visible to you (Private, Friends Only, etc), 3 - the profile is "Public", and the data is visible. Mike Blaszczak's post on Steam forums says, "The community visibility state this API returns is different than the privacy state. It's the effective visibility state from the account making the request to the account being viewed given the requesting account's relationship to the viewed account."
	profilestate | If set, indicates the user has a community profile configured (will be set to '1')
	lastlogoff | The last time the user was online, in unix time.
	commentpermission | If set, indicates the profile allows public comments.
	
	#### Private Data
	
	Name | Description
	:--|:--
	realname | The player's "Real Name", if they have set it.
	primaryclanid | The player's primary group, as configured in their Steam Community profile.
	timecreated | The time the player's account was created.
	gameid | If the user is currently in-game, this value will be returned and set to the gameid of that game.
	gameserverip | The ip and port of the game server the user is currently playing on, if they are playing on-line in a game using Steam matchmaking. Otherwise will be set to "0.0.0.0:0".
	gameextrainfo | If the user is currently in-game, this will be the name of the game they are playing. This may be the name of a non-Steam game shortcut.
	cityid | This value will be removed in a future update (see loccityid)
	loccountrycode | If set on the user's Steam Community profile, The user's country of residence, 2-character ISO country code
	locstatecode | If set on the user's Steam Community profile, The user's state of residence
	loccityid | An internal code indicating the user's city of residence. A future update will provide this data in a more useful way.
	
	#### loccityid
	- steam_location gem/package makes player location data readable for output.
		- An updated readable list can be found at quer's steam location
	- Getting locstatecode and loccityid, can now be done from https://steamcommunity.com/actions/QueryLocations/<loccountrycode>/<locstatecode>/
	*/
	pub async fn getPlayerSummaries(&self) -> Result<Payload_GetPlayerSummaries>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.remove(Self::Parameter_SteamId);
			parameters.insert(Self::Parameter_SteamIds.into(), self.auth.id.to_owned());
			
			if let Some(url) = self.buildUrl(
				Self::Service_User,
				Self::Endpoint_GetPlayerSummaries
			)
			{
				return Ok(self.get::<Payload_GetPlayerSummaries>(&url, &parameters).await
					.context(format!(
						"Error retrieving player summary from Steam Web API for Steam ID {}",
						self.auth.id
					))?);
			}
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	
	
	---
	
	# [GetRecentlyPlayedGames (v0001)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetRecentlyPlayedGames_.28v0001.29)
	
	GetRecentlyPlayedGames returns a list of games a player has played in the
	last two weeks, if the profile is publicly visible. Private, friends-only,
	and other privacy settings are not supported unless you are asking for your
	own personal details (ie the WebAPI key you are using is linked to the
	steamid you are requesting).
	
	Example URL:
	`http://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v0001/?key=XXXXXXXXXXXXXXXXX&steamid=76561197960434622&format=json`
	
	---
	
	## Arguments
	
	Name | Description
	:--|:--
	steamid | The SteamID of the account.
	count | Optionally limit to a certain number of games (the number of games a person has played in the last 2 weeks is typically very small)
	format | Output format. json (default), xml or vdf.
	
	## Result layout
	
	- total_count - the total number of unique games the user has played in the last two weeks. This is mostly significant if you opted to return a limited number of games with the count input parameter
	- games - A games array, with the following contents:
		- appid - Unique identifier for the game
		- name - The name of the game
		- playtime_2weeks - The total number of minutes played in the last 2 weeks
		- playtime_forever - The total number of minutes played "on record", since Steam began tracking total playtime in early 2009.
		- img_icon_url, img_logo_url - These are the filenames of various images for the game. To construct the URL to the image, use this format: `http://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg`. For example, the TF2 logo is returned as `07385eb55b5ba974aebbe74d3c99626bda7920b8`, which maps to the URL: `http://media.steampowered.com/steamcommunity/public/images/apps/440/07385eb55b5ba974aebbe74d3c99626bda7920b8.jpg`
	*/
	pub async fn getRecentlyPlayedGames(&self) -> Result<Payload_GetRecentlyPlayedGames>
	{
		if self.auth.validate()
		{
			if let Some(url) = self.buildUrl(
				Self::Service_Player,
				Self::Endpoint_GetRecentlyPlayedGames
			)
			{
				return Ok(self.get::<Payload_GetRecentlyPlayedGames>(&url, &self.generateParameterMap()).await
					.context(format!(
						"Error retrieving recently played games from Steam Web API for Steam ID {}",
						self.auth.id
					))?)
			}
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	Cal the GetSchemaForGame endpoint to retrieve detailed information about the
	given app id's achievements
	
	# [GetSchemaForGame (v0002)](https://wiki.teamfortress.com/wiki/WebAPI/GetSchemaForGame)
	
	GET `http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2`
	
	---
	
	## Method-specific parameters
	
	Name | Description
	:--|:--
	appid | **uint32** appid of game
	l | **Optional string** localized language to return (english, french, etc.)
	
	---
	
	## Result data
	
	- game
		- gameName (string) Steam internal (non-localized) name of game.
		- gameVersion (int) Steam release version number currently live on Steam.
		- availableGameStats
			- achievements (Optional) (array)
				- name (string) API Name of achievement.
				-  defaultvalue (int) Always 0 (player's default state is unachieved).
				-  displayName (string) Display title string of achievement.
				-  hidden (int) If achievement is hidden to the user before earning achievement, value is 1. 0 if public.
				-  description (string) Display description string of achievement.
				-  icon (string) Absolute URL of earned achievement icon art.
				-  icongray (string) Absolute URL of un-earned achievement icon art.
			- stats (Optional) (array)
				-  name (string) API name of stat.
				-  defaultvalue (int) Default value of stat.
				-  displayName (string) Developer provided name of string.
	*/
	pub async fn getSchemaForGame(&self, appId: usize, language: &String) -> Result<Payload_GetSchemaForGame>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.remove(Self::Parameter_SteamId);
			parameters.insert(Self::Parameter_AppId.into(), appId.to_string());
			parameters.insert(Self::Parameter_Language.into(), language.to_owned());
			
			if let Some(url) = self.buildUrl(
				Self::Service_UserStats,
				Self::Endpoint_GetSchemaForGame
			)
			{
				return Ok(self.get::<Payload_GetSchemaForGame>(&url, &parameters).await
					.context(format!(
						"Error retrieving game schema from Steam Web API for Game ID {}",
						appId
					))?);
			}
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	Construct the fully qualified URL for an endpoint.
	*/
	fn buildUrl(&self, service: &str, endpoint: &str) -> Option<String>
	{
		return Some(format!(
			"{}{}",
			Self::Protocol,
			Path::new(Self::Domain)
				.join(service)
				.join(endpoint)
				.to_slash()?
				.into_owned()
		));
	}
	
	/**
	Generate a default parameter map containing the most commonly used parameters.
	*/
	fn generateParameterMap(&self) -> HashMap<String, String>
	{
		return HashMap::from([
			(Self::Parameter_Key.into(), self.auth.key.to_owned()),
			(Self::Parameter_SteamId.into(), self.auth.id.to_owned()),
			(Self::Parameter_Format.into(), Self::Format_Json.into()),
		]);
	}
	
	/**
	Execute an HTTP GET request.
	*/
	async fn get<T>(&self, url: &String, parameters: &HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		let mut params = String::from("?");
		for (k, v) in parameters
		{
			params = format!("{}&{}={}", params, k, v);
		}
		
		let requestUrl = format!("{}{}", url, params);
		let response = self.client.get(requestUrl)
			.send().await
				.context("Error retrieving Steam API response")?
			.json::<T>().await
				.context("Error parsing Steam API response as JSON")?;
		
		return Ok(response);
	}
}
