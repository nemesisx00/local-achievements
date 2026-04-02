use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::Path;
use anyhow::{anyhow, Context, Result};
use path_slash::PathExt;
use serde::de::DeserializeOwned;
use crate::data::secure::getRetroAchievementsAuth;
use super::{Payload_GetGameInfo, Payload_GetUserCompletionProgress,
	Payload_GetUserProfile, RetroAchievementsAuth};

pub struct RetroAchievementsApi;

impl RetroAchievementsApi
{
	const BaseUrl: &str = "https://retroachievements.org/API/";
	const MediaUrl: &str = "https://media.retroachievements.org/";
	
	const Endpoint_GetGameInfo: &str = "API_GetGameInfoAndUserProgress.php";
	const Endpoint_GetUserGameCompletion: &str = "API_GetUserCompletionProgress.php";
	const Endpoint_GetUserProfile: &str = "API_GetUserProfile.php";
	
	pub const GetUserGameCompletion_Count: u64 = 100;
	
	pub const BadgePath: &str = "Badge";
	pub const BadgeLockedSuffix: &str = "lock";
	
	const Parameter_ApiKey: &str = "y";
	const Parameter_ApiUsername: &str = "u";
	
	pub const Platform: &str = "RetroAchievements";
	
	pub fn sanitizeIconTitle(title: &String) -> String
	{
		return title
			.replace("/", " - ")
			.replace("\\", " - ");
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
	#[allow(unused)]
	pub fn getGameInfo(ulid: &String, gameId: u64) -> Result<Payload_GetGameInfo>
	{
		let auth = getRetroAchievementsAuth()?;
		let mut parameters = Self::generateParameterMap(&auth);
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), ulid.clone());
		parameters.insert("g".into(), gameId.to_string());
		parameters.insert("a".into(), "1".into());
		
		return Ok(Self::get::<Payload_GetGameInfo>(
			&Self::Endpoint_GetGameInfo.into(),
			&parameters
		)
			.context(format!(
				"Error retrieving game info for {} from username {}",
				gameId,
				auth.username()
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
	pub fn getUserCompletionProgress(ulid: Option<String>, offset: Option<u64>) -> Result<Payload_GetUserCompletionProgress>
	{
		let auth = getRetroAchievementsAuth()?;
		let mut parameters = Self::generateParameterMap(&auth);
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), match ulid.clone()
		{
			Some(ulid) => ulid,
			None => auth.username().clone(),
		});
		parameters.insert("c".into(), Self::GetUserGameCompletion_Count.to_string());
		
		if let Some(o) = offset
		{
			parameters.insert("o".into(), o.to_string());
		}
		
		return Ok(Self::get::<Payload_GetUserCompletionProgress>(
			&Self::Endpoint_GetUserGameCompletion.into(),
			&parameters
		)
			.context(format!(
				"Error retrieving user completion progress for username {} (ulid {})",
				auth.username(),
				ulid.unwrap_or_default(),
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
	pub fn getUserProfile(ulid: Option<String>) -> Result<Payload_GetUserProfile>
	{
		let auth = getRetroAchievementsAuth()?;
		let mut parameters = Self::generateParameterMap(&auth);
		parameters.remove(Self::Parameter_ApiUsername);
		parameters.insert("u".into(), match ulid
		{
			Some(ulid) => ulid,
			None => auth.username().clone(),
		});
		
		return Ok(Self::get::<Payload_GetUserProfile>(
			&Self::Endpoint_GetUserProfile.into(),
			&parameters
		)
			.context(format!(
				"Error retrieving user profile for username {}",
				auth.username()
			))?);
	}
	
	pub fn buildMediaUrl(endpoint: &str) -> Option<String>
	{
		return Self::buildUrl(Self::MediaUrl, endpoint);
	}
	
	/**
	Generate a default parameter map containing the most commonly used parameters.
	*/
	fn buildUrl(base: &str, endpoint: &str) -> Option<String>
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
	fn generateParameterMap(auth: &RetroAchievementsAuth) -> HashMap<String, String>
	{
		return HashMap::from([
			(Self::Parameter_ApiKey.into(), auth.key().clone()),
			(Self::Parameter_ApiUsername.into(), auth.username().clone()),
		]);
	}
	
	/**
	Execute an HTTP GET request.
	*/
	fn get<T>(endpoint: &String, parameters: &HashMap<String, String>) -> Result<T>
		where T: DeserializeOwned
	{
		if let Some(url) = Self::buildUrl(Self::BaseUrl, endpoint)
		{
			let mut params = String::default();
			for (k, v) in parameters
			{
				params = format!("{}&{}={}", params, k, v);
			}
			
			let requestUrl = format!("{}?{}", url, params.split_off(1));
			
			let response = ureq::get(requestUrl)
				.call()
					.context("Error retrieving RetroAchievements API response")?
				.body_mut()
				.read_json::<T>()
					.context("Error parsing RetroAchievements API response as JSON")?;
			
			return Ok(response);
		}
		
		return Err(anyhow!(ErrorKind::InvalidInput));
	}
}
