#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::Path;
use ::anyhow::{Context, Result};
use ::reqwest::Client;
use ::serde::de::DeserializeOwned;
use crate::error;
use crate::io::{cacheImage, getImagePath};
use crate::data::SteamInfo;
use super::data::{AuthData, GetOwnedGamesPayload, GetPlayerSummariesPayload};

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: AuthData,
	pub client: Client,
}

impl Api
{
	const Platform: &str = "Steam";
	
	const BaseUrl: &str = "https://api.steampowered.com/";
	
	const Service_Player: &str = "IPlayerService/";
	const Service_User: &str = "ISteamUser/";
	
	const Endpoint_GetOwnedGames: &str = "GetOwnedGames/v0001/";
	const Endpoint_GetPlayerSummaries: &str = "GetPlayerSummaries/v0002/";
	
	const Parameter_Format: &str = "format";
	const Parameter_IncludeAppInfo: &str = "include_appinfo";
	const Parameter_IncludePlayedFreeGames: &str = "include_played_free_games";
	const Parameter_Key: &str = "key";
	const Parameter_SteamId: &str = "steamid";
	const Parameter_SteamIds: &str = "steamids";
	
	const Format_Json: &str = "json";
	
	const Value_False: &str = "0";
	const Value_True: &str = "1";
	
	const GameIconUrl: &str = "https://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg";
	const Replace_GameIconHash: &str = "{hash}";
	const Replace_GameIconId: &str = "{appid}";
	
	pub fn new(auth: AuthData) -> Self
	{
		return Self { auth, ..Default::default() };
	}
	
	pub fn iconFileName(appId: usize) -> String
	{
		return format!("{}_icon.jpg", appId);
	}
	
	/**
	
	*/
	pub async fn cacheGameIcons(&self, games: Vec<SteamInfo>)
	{
		for game in games.iter()
		{
			if let Some(path) = getImagePath(Self::Platform.into(), Self::iconFileName(game.id))
			{
				if !Path::new(&path).exists()
				{
					match self.cacheGameIcon(game.id, game.iconHash.to_owned()).await
					{
						Ok(_) => println!("Icon image cached for {}", game.id),
						Err(e) => println!("Error caching icon image for {}: {:?}", game.id, e),
					}
				}
			}
		}
		println!("Done with SteamApi::cacheGameIcons()");
	}
	
	/**
	Retrieve a Steam game's icon and cache it locally.
	
	The url used to retrieve the icon:
	`https://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg`
	*/
	async fn cacheGameIcon(&self, appId: usize, hash: String) -> Result<()>
	{
		let url = Self::GameIconUrl
			.replace(Self::Replace_GameIconId, appId.to_string().as_str())
			.replace(Self::Replace_GameIconHash, &hash);
		
		let response = self.client.get(url)
			.send().await
				.context(format!("Error retrieving Steam Game Icon image for app id {} with hash {}", appId, hash))?
			.bytes().await
				.context(format!("Error converting the Steam Game Icon response into an instance of Bytes for app id: {}", appId))?;
		
		cacheImage(Self::Platform.into(), Self::iconFileName(appId), response.as_ref())
			.context(format!("Error saving Steam Game Icon to file for app id {}", appId))?;
		
		return Ok(());
	}
	
	/**
	Call the GetOwnedGames endpoint to retrieve the current user's list of owned (or played free) games.
	
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
	---|---
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
	pub async fn getOwnedGames(&self) -> Result<GetOwnedGamesPayload>
	{
		if self.auth.validate()
		{
			let mut parameters = self.generateParameterMap();
			parameters.insert(Self::Parameter_IncludeAppInfo.into(), Self::Value_True.into());
			parameters.insert(Self::Parameter_IncludePlayedFreeGames.into(), Self::Value_True.into());
			
			let url = self.buildUrl(Self::Service_Player, Self::Endpoint_GetOwnedGames);
			let response = self.get::<GetOwnedGamesPayload>(url, parameters).await
				.context(format!("Error retrieving list of owned games from Steam Web API for Steam ID {}", self.auth.id))?;
			
			return Ok(response);
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	/**
	Call the GetPlayerSummaries endpoint to retrieve the current user's profile information.
	
	---
	
	# [GetPlayerSummaries (v0002)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetPlayerSummaries_.28v0002.29)
	
	Returns basic profile information for a list of 64-bit Steam IDs.
	
	Example URL:
	`http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key=XXXXXXXXXXXXXXXXXXXXXXX&steamids=76561197960435530`
	(This will show Robin Walker's profile information.)
	
	---
	
	### Arguments
	
	Name | Description
	---|---
	steamids | Comma-delimited list of 64 bit Steam IDs to return profile information for. Up to 100 Steam IDs can be requested.
	format | Output format. json (default), xml or vdf.
	
	---
	
	### Return Value
	
	Some data associated with a Steam account may be hidden if the user has their profile visibility set to "Friends Only" or "Private". In that case, only public data will be returned.
	
	#### Public Data
	
	Name | Description
	---|---
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
	---|---
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
	pub async fn getPlayerSummaries(&self) -> Result<GetPlayerSummariesPayload>
	{
		if self.auth.validate()
		{
			let mut parameters = HashMap::<String, String>::new();
			parameters.insert(Self::Parameter_Key.into(), self.auth.key.clone());
			parameters.insert(Self::Parameter_SteamIds.into(), self.auth.id.clone());
			parameters.insert(Self::Parameter_Format.into(), Self::Format_Json.into());
			
			let url = self.buildUrl(Self::Service_User, Self::Endpoint_GetPlayerSummaries);
			let response = self.get::<GetPlayerSummariesPayload>(url, parameters).await
				.context(format!("Error retrieving player summary from Steam Web API for Steam ID {}", self.auth.id))?;
			
			return Ok(response);
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
	
	fn buildUrl(&self, service: &str, endpoint: &str) -> String
	{
		return format!("{}{}{}", Self::BaseUrl, service, endpoint);
	}
	
	fn generateParameterMap(&self) -> HashMap<String, String>
	{
		let mut map = HashMap::new();
		map.insert(Self::Parameter_Key.into(), self.auth.key.clone());
		map.insert(Self::Parameter_SteamId.into(), self.auth.id.clone());
		map.insert(Self::Parameter_Format.into(), Self::Format_Json.into());
		
		return map;
	}
	
	async fn get<T>(&self, url: String, parameters: HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		let mut params = String::from("?");
		for (k, v) in parameters
		{
			params = format!("{}&{}={}", params, k, v);
		}
		
		let requestUrl = format!("{}{}", url, params);
		let response = self.client.get(requestUrl)
			.send()
			.await
				.context("Error retrieving Steam API response")?
			.json::<T>()
			.await
				.context("Error parsing Steam API response as JSON")?;
		
		return Ok(response);
	}
}
