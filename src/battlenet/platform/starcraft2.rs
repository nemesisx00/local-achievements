use std::str::FromStr;
use anyhow::{anyhow, Result};
use reqwest::Url;
use tracing::{error, info, warn};
use crate::battlenet::BattleNetSession;
use crate::battlenet::data::region::Region;
use crate::battlenet::platform::data::starcraft2::account::PayloadPlayer;
use crate::battlenet::platform::data::starcraft2::profile::metadata::PayloadStatic;
use crate::battlenet::platform::data::starcraft2::profile::profile::PayloadProfile;
use super::api::BattleNetApi;

/**
Contains all implemented endpoints which retrieve StarCraft 2 data.
*/
pub struct Starcraft2;

impl Starcraft2
{
	const RootUriApi: &str = ".api.blizzard.com";
	const RootUriApiChina: &str = "gateway.battlenet.com.cn";
	
	const UriAccountPlayer: &str = "/sc2/player/";
	const UriProfileProfile: &str = "/sc2/profile/";
	const UriProfileStatic: &str = "/sc2/static/profile/";
	
	/**
	Returns metadata for an individual's account.
	
	Metadata like the profile ID, region ID, and realm ID.
	*/
	pub async fn accountPlayer(api: &BattleNetApi, session: BattleNetSession, accountId: u64) -> Result<PayloadPlayer>
	{
		let url = Url::from_str(format!(
			"{}{}{}{}{}",
			BattleNetApi::Https,
			Region::US.shortString(),
			Self::RootUriApi,
			Self::UriAccountPlayer,
			accountId
		).as_str())?;
		
		let payloads = api.get::<Vec<PayloadPlayer>>(url, session).await?;
		
		return match payloads.first()
		{
			None => Err(anyhow!("Failed to retrieve player profile.")),
			Some(payload) => Ok(payload.clone()),
		};
	}
	
	pub async fn profileProfile(api: &BattleNetApi, session: BattleNetSession, region: Region, profileId: u64) -> Result<PayloadProfile>
	{
		let url = Url::from_str(format!(
			"{}{}{}{}/{}/{}/{}",
			BattleNetApi::Https,
			region.shortString(),
			Self::RootUriApi,
			Self::UriProfileProfile,
			region.regionId(),
			region.realmId(),
			profileId
		).as_str())?;
		
		let payload = api.get::<PayloadProfile>(url, session).await?;
		return Ok(payload);
	}
	
	/**
	Returns all static SC2 profile data (achievements, categories, criteria, and rewards).
	*/
	pub async fn profileStatic(api: &BattleNetApi, session: BattleNetSession, region: Region) -> Result<PayloadStatic>
	{
		let url = Url::from_str(format!(
			"{}{}{}{}{}",
			BattleNetApi::Https,
			region.shortString(),
			Self::RootUriApi,
			Self::UriProfileStatic,
			region.regionId()
		).as_str())?;
		
		let payload = api.get::<PayloadStatic>(url, session).await?;
		return Ok(payload);
	}
}
