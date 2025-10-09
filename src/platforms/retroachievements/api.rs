use std::{collections::HashMap, io::ErrorKind, path::Path};
use anyhow::{Context, Result};
use path_slash::PathExt;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::constants::Icon_Locked;
use crate::data::makeRelative;
use crate::io::{getImagePath, Filename_GameIcon, Path_Avatars, Path_Games};
use crate::platforms::util::cacheImageIfNotExists;
use crate::{error, join, png, pngAlt};
use crate::platforms::retroachievements::data::{Payload_GetGameInfo,
	Payload_GetUserCompletionProgress, Payload_GetUserProfile};
use super::data::RetroAchievementsAuth;

#[derive(Clone, Debug, Default)]
pub struct Api
{
	pub auth: RetroAchievementsAuth,
	pub client: Client,
}

impl From<RetroAchievementsAuth> for Api
{
	fn from(value: RetroAchievementsAuth) -> Self
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
	const BaseUrl: &str = "https://retroachievements.org/API/";
	const MediaUrl: &str = "https://media.retroachievements.org/";
	
	const Endpoint_GetGameInfo: &str = "API_GetGameInfoAndUserProgress.php";
	const Endpoint_GetUserGameCompletion: &str = "API_GetUserCompletionProgress.php";
	const Endpoint_GetUserProfile: &str = "API_GetUserProfile.php";
	
	pub const GetUserGameCompletion_Count: usize = 100;
	
	pub const BadgePath: &str = "Badge";
	pub const BadgeLockedSuffix: &str = "lock";
	
	const Parameter_ApiUsername: &str = "z";
	const Parameter_ApiKey: &str = "y";
	
	pub const Platform: &str = "RetroAchievements";
	
	pub async fn cacheIcon_Achievements(&self, gameId: usize, payload: &Payload_GetGameInfo, force: bool) -> Result<()>
	{
		let group = join!(Path_Games, gameId.to_string());
		let platform = Self::Platform.into();
		
		for achievement in payload.Achievements.values()
		{
			let filename = png!(achievement.Title);
			let filenameLocked = pngAlt!(achievement.Title, Icon_Locked);
			
			if let Some(url) = self.buildUrl(&Self::MediaUrl, join!(Self::BadgePath, png!(achievement.BadgeName)).as_str())
			{
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
			
			if let Some(url) = self.buildUrl(&Self::MediaUrl, join!(Self::BadgePath, pngAlt!(achievement.BadgeName, Self::BadgeLockedSuffix)).as_str())
			{
				if let Some(path) = getImagePath(&platform, &group, &filenameLocked)
				{
					cacheImageIfNotExists(
						&self.client,
						&url,
						&path,
						&platform,
						&group,
						&filenameLocked,
						force
					).await?;
				}
			}
		}
		
		return Ok(());
	}
	
	pub async fn cacheIcon_Games(&self, payload: &Payload_GetUserCompletionProgress, force: bool) -> Result<()>
	{
		let filename = png!(Filename_GameIcon);
		let platform = Self::Platform.into();
		
		for game in payload.Results.iter()
		{
			if let Some(url) = self.buildUrl(&Self::MediaUrl, &makeRelative(&game.ImageIcon))
			{
				let group = join!(Path_Games, game.GameID.to_string());
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
		}
		
		return Ok(());
	}
	
	pub async fn cacheProfileAvatar(&self, ulid: &String, endpoint: &String, force: bool) -> Result<()>
	{
		if let Some(url) = self.buildUrl(&Self::MediaUrl, endpoint)
		{
			let filename = png!(ulid);
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
	Call the GetGameInfoAndUserProgress endpoint to retrieve detailed information
	about a specific game and the current user's progress for that game.
	
	---
	
	# [GetGameInfoAndUserProgress](https://api-docs.retroachievements.org/v1/get-game-info-and-user-progress.html)
	
	Example URL: `https://retroachievements.org/API/API_GetGameInfoAndUserProgress.php?a=1&g=#####&u=XXXXXXXX&y=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`
	
	---
	
	### Arguments
	
	Name | Required | Description
	:--|:--|:--
	y | Yes | Your web API key.
	u | Yes | The target username or ULID.
	g | Yes | The target game ID.
	a | No | Set to "1" if user award metadata should be included (default: 0).
	
	You must query the user by either their username or their ULID. Please note the username is not considered a stable value. As of 2025, users can change their usernames. Initially querying by username is a good way to fetch a ULID.
	
	### Return Value
	
	A JSON response with the following properties:
	
	Name | Description
	:--|:--
	ID | int
	Title | String
	ConsoleID | int
	ForumTopicID | int
	Flags | int ; optional
	ImageIcon | String
	ImageTitle | String
	ImageIngame | String
	ImageBoxArt | String
	Publisher | String
	Developer | String
	Genre | String
	Released | Date time string
	ReleasedAtGranularity | String
	IsFinal | bool ; Deprecated, always returns false
	RichPresencePatch | String
	GuideURL | String ; optional
	ConsoleName | String
	ParentGameID | int ; optional
	NumDistinctPlayers | int
	NumAchievements | int
	Achievements | Map\<int, Achievement\>
	NumAwardedToUser | int
	NumAwardedToUserHardcore | int
	NumDistinctPlayersCasual | int
	NumDistinctPlayersHardcore | int
	UserCompletion | String
	UserCompletionHardcore | String
	HighestAwardKind | String ; optional
	HighestAwardDate | Timestamp string ; optional
	
	#### Achievement JSON properties:
	
	Name | Description
	:--|:--
	ID | int
	Title | String
	Description | String
	Points | int
	TrueRatio | int
	Type | String ; optional
	BadgeName | String
	NumAwarded | int
	NumAwardedHardcore | int
	DisplayOrder | int
	Author | String
	AuthorULID | String
	DateCreated | String
	DateModified | String
	MemAddr | String
	DateEarned | String ; optional
	DateEarnedHardcore | String ; optional
	*/
	pub async fn getGameInfo(&self, ulid: &String, gameId: usize) -> Result<Payload_GetGameInfo>
	{
		let mut parameters = self.generateParameterMap();
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), ulid.to_owned());
		parameters.insert("g".into(), gameId.to_string());
		parameters.insert("a".into(), "1".into());
		
		return Ok(self.get::<Payload_GetGameInfo>(
			&Self::Endpoint_GetGameInfo.into(),
			&parameters
		).await
			.context(format!(
				"Error retrieving user profile for username {}",
				self.auth.username
			))?);
	}
	
	/**
	Call the GetUserCompletionProgress endpoint to retrieve the current user's
	completion information for all games associated with their account.
	
	---
	
	# [GetUserCompletionProgress](https://api-docs.retroachievements.org/v1/get-user-completion-progress.html)
	
	Example URL:
	`https://retroachievements.org/API/API_GetUserCompletionProgress.php?u=XXXXXXXX&y=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`
	
	---
	
	### Arguments
	
	Name | Required | Description
	:--|:--|:--
	y | Yes | Your web API key.
	u | Yes | The target username or ULID.
	c | No | Count, number of records to return (default: 100, max: 500).
	o | No | Offset, number of entries to skip (default: 0).
	
	You must query the user by either their username or their ULID. Please note the username is not considered a stable value. As of 2025, users can change their usernames. Initially querying by username is a good way to fetch a ULID.
	
	### Return Value
	
	A JSON response with the following properties:
	
	Name | Description
	:--|:--
	Count | int
	Total | int
	Results | Array\<Game\>
	
	#### Game JSON properties:
	
	Name | Description
	:--|:--
	GameID | int
	Title | String
	ImageIcon | String
	ConsoleID | int
	ConsoleName | String
	MaxPossible | int
	NumAwarded | int
	NumAwardedHardcore | int
	MostRecentAwardedDate | Timestamp string
	HighestAwardKind | String
	HighestAwardDate | Timestamp string
	*/
	pub async fn getUserCompletionProgress(&self, ulid: Option<String>, offset: Option<usize>) -> Result<Payload_GetUserCompletionProgress>
	{
		let mut parameters = self.generateParameterMap();
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), match ulid
		{
			Some(ulid) => ulid,
			None => self.auth.username.to_owned(),
		});
		parameters.insert("c".into(), Self::GetUserGameCompletion_Count.to_string());
		
		if let Some(o) = offset
		{
			parameters.insert("o".into(), o.to_string());
		}
		
		return Ok(self.get::<Payload_GetUserCompletionProgress>(
			&Self::Endpoint_GetUserGameCompletion.into(),
			&parameters
		).await
			.context(format!(
				"Error retrieving user profile for username {}",
				self.auth.username
			))?);
	}
	
	/**
	Call the GetUserProfile endpoint to retrieve the current user's profile
	information.
	
	---
	
	# [GetUserProfile](https://api-docs.retroachievements.org/v1/get-user-profile.html)
	
	Example URL:
	`https://retroachievements.org/API/API_GetUserProfile.php?u=XXXXXXXX&y=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`
	
	---
	
	### Arguments
	
	Name | Required | Description
	:--|:--|:--
	y | Yes | Your web API key.
	u | Yes | The target username or ULID.
	
	You must query the user by either their username or their ULID. Please note the username is not considered a stable value. As of 2025, users can change their usernames. Initially querying by username is a good way to fetch a ULID.
	
	### Return Value
	
	A JSON response with the following properties:
	
	Name | Description
	:--|:--
	User | String
	ULID | String
	UserPic | String
	MemberSince | Date time string
	RichPresenceMsg | String
	LastGameID | int
	ContribCount | int
	ContribYield | int
	TotalPoints | int
	TotalSoftcorePoints | int
	TotalTruePoints | int
	Permissions | int
	ID | int
	UserWallActive | bool
	Motto | String
	*/
	pub async fn getUserProfile(&self, ulid: Option<String>) -> Result<Payload_GetUserProfile>
	{
		let mut parameters = self.generateParameterMap();
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), match ulid
		{
			Some(ulid) => ulid,
			None => self.auth.username.to_owned(),
		});
		
		return Ok(self.get::<Payload_GetUserProfile>(
			&Self::Endpoint_GetUserProfile.into(),
			&parameters
		).await
			.context(format!(
				"Error retrieving user profile for username {}",
				self.auth.username
			))?);
	}
	
	/**
	Generate a default parameter map containing the most commonly used parameters.
	*/
	fn buildUrl(&self, base: &str, endpoint: &str) -> Option<String>
	{
		return Some(
			Path::new(base)
				.join(endpoint)
				.to_slash()?
				.into_owned()
		);
	}
	
	/**
	Generate a default parameter map containing the most commonly used parameters.
	*/
	fn generateParameterMap(&self) -> HashMap<String, String>
	{
		return HashMap::from([
			(Self::Parameter_ApiKey.into(), self.auth.key.to_owned()),
			(Self::Parameter_ApiUsername.into(), self.auth.username.to_owned()),
		]);
	}
	
	/**
	Execute an HTTP GET request.
	*/
	async fn get<T>(&self, endpoint: &String, parameters: &HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		if let Some(url) = self.buildUrl(Self::BaseUrl, endpoint)
		{
			let mut params = String::from("?");
			for (k, v) in parameters
			{
				params = format!("{}&{}={}", params, k, v);
			}
			
			let requestUrl = format!("{}{}", url, params);
			
			let response = self.client.get(requestUrl)
				.send().await
					.context("Error retrieving RetroAchievements API response")?
				.json::<T>().await
					.context("Error parsing RetroAchievements API response as JSON")?;
			
			return Ok(response);
		}
		
		return Err(error!(ErrorKind::InvalidInput));
	}
}
