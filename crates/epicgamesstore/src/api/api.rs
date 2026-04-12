use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;
use ureq::config::RedirectAuthHeaders;
use urlencoding::encode;
use super::endpoint::{Extensions, Variables};
use super::endpoint::achievement::{AchievementVariables, Payload_Achievement};
use super::endpoint::player::{Payload_PlayerProfile, PlayerProfileVariables};
use super::endpoint::private::{Payload_PlayerProfilePrivate,
	PlayerProfilePrivateVariables};
use super::endpoint::progress::{AchievementProgressVariables,
	Payload_AchievementProgress};

#[derive(Clone, Debug, Default)]
pub struct EgsApi;

impl EgsApi
{
	pub const Platform: &str = "EpicGamesStore";
	
	const Endpoint: &str = "https://store.epicgames.com/graphql";
	
	const Error_AccountIdMissing: &str = "Account ID must not be empty";
	
	const Operation_Achievement: &str = "Achievement";
	const Operation_PlayerProfileAchievementsByProductId: &str = "playerProfileAchievementsByProductId";
	const Operation_PlayerProfile: &str = "playerProfile";
	const Operation_PlayerProfilePrivate: &str = "playerProfilePrivate";
	
	const QueryHash_Achievement: &str = "9284d2fe200e351d1496feda728db23bb52bfd379b236fc3ceca746c1f1b33f2";
	const QueryHash_PlayerProfile: &str = "ff954147a23d38a0e5b050962d442099487da001a0ab4b10ccbec8ac49755b3c";
	const QueryHash_PlayerProfilePrivate: &str = "47d0391fa5ec42d829e4a03f399cb586a29cf3cebd940cc4747aed0192c61114";
	const QueryHash_PlayerProfileAchievementsByProductId: &str = "70ff714976f88a85aafa3cb5abb9909d52e12a3ff585d7b49550d2493a528fb0";
	
	/**
	Query the `Achievement` operation.
	
	Retrieves the full list of achievement metadata for a given game.
	*/
	pub fn getAchievementMetadata(accountId: &String, sandboxId: &String) -> Result<Payload_Achievement>
	{
		return match !accountId.is_empty()
		{
			false => Err(anyhow!(Self::Error_AccountIdMissing)),
			true => Self::get(
				Self::Operation_Achievement.into(),
				AchievementVariables
				{
					sandboxId: sandboxId.clone(),
					..Default::default()
				},
				Self::QueryHash_Achievement.to_string().into()
			)
		};
	}
	
	/**
	Query the `playerProfileAchievementsByProductId` operation.
	
	Retrieves the user's unlock state and data for the given game.
	
	## Parameters
	- productId: `String` The product id of the game whose achievement progress is being requested.
	*/
	pub fn getAchievementProgress(accountId: &String, productId: &String) -> Result<Payload_AchievementProgress>
	{
		return match accountId.is_empty()
		{
			false => Self::get(
				Self::Operation_PlayerProfileAchievementsByProductId.into(),
				AchievementProgressVariables
				{
					epicAccountId: accountId.clone(),
					productId: productId.clone(),
				},
				Self::QueryHash_PlayerProfileAchievementsByProductId.to_string().into(),
			),
			true => Err(anyhow!(Self::Error_AccountIdMissing)),
		};
	}
	
	/**
	Query the `playerProfile` operation.
	
	Retrieves the user's display name and avatar image URLs.
	*/
	pub fn getPlayerProfile(accountId: &String) -> Result<Payload_PlayerProfile>
	{
		return match !accountId.is_empty()
		{
			false => Err(anyhow!(Self::Error_AccountIdMissing)),
			true => Self::get(
				Self::Operation_PlayerProfile.into(),
				PlayerProfileVariables
				{
					epicAccountId: accountId.clone(),
				},
				Self::QueryHash_PlayerProfile.to_string().into()
			)
		};
	}
	
	/**
	Query the `playerProfilePrivate` operation.
	
	Retrieves summaries of any games for which the user has unlocked achievements.
	
	### Note
	In an attempt to keep things simple and avoid requiring authentication,
	the requested `epicAccountId` will need its profile privacy set to `Public`
	for this operation to succeed.
	
	Also, while the variables struct contains a parameter for pagination, it
	seems this only affects the `friendsSummaries` section, which this application
	is ignoring. This can be updated later if it turns out to be necessary.
	*/
	pub fn getPlayerProfilePrivate(accountId: &String) -> Result<Payload_PlayerProfilePrivate>
	{
		return match !accountId.is_empty()
		{
			false => Err(anyhow!(Self::Error_AccountIdMissing)),
			true => {
				return Self::get(
					Self::Operation_PlayerProfilePrivate.into(),
					PlayerProfilePrivateVariables
					{
						accountId: accountId.clone(),
						epicAccountId: accountId.clone(),
						..Default::default()
					},
					Self::QueryHash_PlayerProfilePrivate.to_string().into(),
				);
			}
		};
	}
	
	/**
	Execute an HTTP GET request to the GraphQL endpoint.
	
	## Parameters
	- operation: `String` The name of the operation to query.
	- variables: `impl Variables` The set of variables required by the operation. Serialized to JSON before transmission.
	- extensions: `Extensions` Contains the sha-256 hash of the persisted query being requested. Serialized to JSON before transmission.
	*/
	fn get<T>(operation: String, variables: impl Variables, extensions: Extensions) -> Result<T>
		where T: DeserializeOwned + Default
	{
		let jsonVariables = serde_json::to_string(&variables)?;
		let jsonVariables = encode(&jsonVariables);
		
		let jsonExtensions = serde_json::to_string(&extensions)?;
		let jsonExtensions = encode(&jsonExtensions);
		
		let requestUrl = format!(
			"{}?operationName={}&variables={}&extensions={}",
			Self::Endpoint,
			operation,
			jsonVariables.into_owned(),
			jsonExtensions.into_owned()
		);
		
		let response = ureq::get(requestUrl)
			.config()
				.redirect_auth_headers(RedirectAuthHeaders::SameHost)
				.build()
			.call()
				.context("Error retrieving Epic Games Store API response")?
			.body_mut()
			.read_json::<T>()
				.context("Error parsing Epic Games Store API response as JSON")?;
		
		return Ok(response);
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	#[test]
	fn parsePlayerProfilePayload()
	{
		let response = r#"{
	"data":
	{
		"PlayerProfile":
		{
			"playerProfile":
			{
				"epicAccountId": "account id string",
				"displayName": "display name",
				"avatar":
				{
					"small": "small url",
					"medium": "medium url",
					"large": "large url"
				}
			}
		}
	}
}"#;
		let payload = serde_json::from_str::<Payload_PlayerProfile>(response);
		assert!(payload.is_ok());
		
		let profile = payload.unwrap();
		assert_eq!(&profile.data.PlayerProfile.playerProfile.avatar.large, "large url");
		assert_eq!(&profile.data.PlayerProfile.playerProfile.avatar.medium, "medium url");
		assert_eq!(&profile.data.PlayerProfile.playerProfile.avatar.small, "small url");
		assert_eq!(&profile.data.PlayerProfile.playerProfile.displayName, "display name");
		assert_eq!(&profile.data.PlayerProfile.playerProfile.epicAccountId, "account id string");
	}
}
