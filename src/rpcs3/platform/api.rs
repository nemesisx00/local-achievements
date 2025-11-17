use std::fs::{File, copy, read, read_dir, read_to_string};
use std::io::{self, Cursor};
use std::path::Path;
use anyhow::Result;
use saphyr::{LoadableYamlNode, Scalar, Yaml};
use crate::join;
use crate::io::{Path_Games, generateImageCacheDir, getImagePath};
use crate::rpcs3::data::game::Game;
use crate::rpcs3::data::settings::Settings;
use crate::rpcs3::platform::data::conf::{TrophyConf, TrophyMetadata};
use crate::rpcs3::platform::data::user::{DatFile, EntryType6};

const DefaultAccountId: u64 = 1;

#[derive(Clone, Debug)]
pub struct Api
{
	pub accountId: u64,
	pub rootDir: String,
}

impl Default for Api
{
	fn default() -> Self
	{
		return Self
		{
			accountId: DefaultAccountId,
			rootDir: Default::default(),
		};
	}
}

impl From<Settings> for Api
{
	fn from(value: Settings) -> Self
	{
		return Self
		{
			accountId: value.accountId,
			rootDir: value.appDataDirectory.to_owned(),
		};
	}
}

impl Api
{
	/**
	The ticks value representing 1970-01-01 00:00:00.000000
	
	Used to convert timestamps stored in TROPUSR.DAT into a Unix-compatible
	timestamp, in microseconds.
	*/
	const TicksMagicOffset: u64 = 62135596800000000;
	
	pub const Platform: &str = "RPCS3";
	
	pub const GameIconFileName: &str = "ICON0.PNG";
	pub const TrophyIconPrefix: &str = "TROP";
	
	const ConfFileName: &str = "TROPCONF.SFM";
	const DatFileName: &str = "TROPUSR.DAT";
	const RelativeConfigDir: &str = "config/rpcs3/";
	const RelativeHomeDir: &str = "dev_hdd0/home";
	const RelativeUserTrophyDir: &str = "trophy";
	const RpcnFileName: &str = "rpcn.yml";
	const RpcnIdYamlKey: &str = "NPID";
	
	pub fn cacheGameIcons(&self, npCommId: String) -> Result<()>
	{
		let group = join!(Path_Games, npCommId);
		let platform = Self::Platform.into();
		
		generateImageCacheDir(&platform, &group)?;
		
		let gamePath = Path::new(&self.rootDir)
			.join(Self::RelativeConfigDir)
			.join(Self::RelativeHomeDir)
			.join(self.formatAccountId())
			.join(Self::RelativeUserTrophyDir)
			.join(npCommId);
		
		let paths = read_dir(gamePath)?;
		
		for path in paths
		{
			let entry = path?;
			
			if let Some(fullPath) = entry.path().to_str()
			{
				if fullPath.ends_with(".PNG")
				{
					if let Ok(fileName) = entry.file_name().into_string()
					{
						if let Some(imagePath) = getImagePath(&platform, &group, &fileName)
						{
							if !Path::new(&imagePath).exists()
							{
								copy(fullPath, imagePath)?;
							}
						}
					}
				}
			}
		}
		
		return Ok(());
	}
	
	pub fn generateGameList(&self) -> Result<Vec<Game>>
	{
		let mut games: Vec<Game> = vec![];
		
		match self.getNpCommIdList()
		{
			Err(e) => println!("Error reading the NpCommId list (RPCS3): {:?}", e),
			Ok(npCommIds) => {
				for npCommId in npCommIds
				{
					match self.parseTrophyConf(npCommId.to_owned())
					{
						Err(e) => println!("Error parsing the TROPHYCONF.SFM for {}: {:?}", npCommId, e),
						Ok(trophyConf) => games.push(trophyConf.into()),
					}
					
					match self.parseTrophies(npCommId.to_owned())
					{
						Err(e) => println!("Error parsing the trophies for {}: {:?}", npCommId, e),
						Ok(trophies) => {
							for (metadata, type6) in trophies
							{
								if let Some(game) = games.iter_mut()
									.find(|g| g.npCommId == npCommId)
								{
									if let Some(trophy) = game.trophies.iter_mut()
										.find(|t| t.id == metadata.id as u64)
									{
										trophy.unlocked = type6.trophyState > 0;
										if trophy.unlocked
										{
											trophy.unlockedTimestamp = Some(type6.timestamp2 - Self::TicksMagicOffset);
										}
									}
								}
							}
						}
					}
				}
			}
		}
		
		return Ok(games);
	}
	
	pub fn getNpCommIdList(&self) -> Result<Vec<String>>
	{
		let trophiesPath = Path::new(&self.rootDir)
			.join(Self::RelativeConfigDir)
			.join(Self::RelativeHomeDir)
			.join(self.formatAccountId())
			.join(Self::RelativeUserTrophyDir);
		
		let paths = read_dir(trophiesPath)?;
		
		let mut npCommIds = vec![];
		
		for path in paths
		{
			match path?.file_name().into_string()
			{
				Err(e) => println!("Failed to into_string() this path: {:?}", e),
				Ok(p) => npCommIds.push(p),
			}
		}
		
		return Ok(npCommIds);
	}
	
	pub fn parseDatFile(&self, npCommId: String) -> Result<DatFile>
	{
		let datPath = Path::new(&self.rootDir)
			.join(Self::RelativeConfigDir)
			.join(Self::RelativeHomeDir)
			.join(self.formatAccountId())
			.join(Self::RelativeUserTrophyDir)
			.join(npCommId)
			.join(Self::DatFileName);
		
		let buffer = read(&datPath)?;
		let mut cursor = Cursor::new(buffer);
		let datFile = DatFile::readFromCursor(&mut cursor)?;
		
		return Ok(datFile);
	}
	
	pub fn getRpcnId(&self) -> Result<String>
	{
		let rpcnPath = Path::new(&self.rootDir)
			.join(Self::RelativeConfigDir)
			.join(Self::RpcnFileName);
		
		let file = File::open(rpcnPath)?;
		let data = io::read_to_string(file)?;
		let yaml = Yaml::load_from_str(&data.as_str())?;
		
		let rpcnId = match yaml.iter().find(|y| y.is_mapping())
		{
			Some(Yaml::Mapping(map)) => match map.get(&Yaml::Value(Scalar::String(Self::RpcnIdYamlKey.into())))
			{
				Some(Yaml::Value(Scalar::String(id))) => id.to_string(),
				_ => String::default(),
			},
			_ => String::default(),
		};
		
		return Ok(rpcnId);
	}
	
	pub fn parseTrophies(&self, npCommId: String) -> Result<Vec<(TrophyMetadata, EntryType6)>>
	{
		let datFile = self.parseDatFile(npCommId.to_owned())?;
		let trophyConf = self.parseTrophyConf(npCommId.to_owned())?;
		
		let mut trophies = vec![];
		
		for metadata in trophyConf.trophies
		{
			if let Some(entry) = datFile.type6.iter()
				.find(|entry| entry.trophyId == metadata.id)
			{
				trophies.push((metadata.to_owned(), entry.to_owned()));
			}
		}
		
		return Ok(trophies);
	}
	
	pub fn parseTrophyConf(&self, npCommId: String) -> Result<TrophyConf>
	{
		let confPath = Path::new(&self.rootDir)
			.join(Self::RelativeConfigDir)
			.join(Self::RelativeHomeDir)
			.join(self.formatAccountId())
			.join(Self::RelativeUserTrophyDir)
			.join(npCommId)
			.join(Self::ConfFileName);
		
		let xml = read_to_string(confPath)?;
		let trophyConf = serde_xml_rs::from_str::<TrophyConf>(&xml)?;
		return Ok(trophyConf);
	}
	
	fn formatAccountId(&self) -> String
	{
		return format!("{:08}", self.accountId);
	}
}

#[cfg(test)]
mod tests
{
	use std::env;
	use crate::rpcs3::data::trophy::TrophyGrade;
	use super::*;
	
	#[test]
	fn accountId()
	{
		let api = Api { accountId: 1, ..Default::default() };
		let accountId = api.formatAccountId();
		
		assert_eq!(accountId, "00000001");
	}
	
	/**
	Requires the following environment variable to be set in order to run successfully.
	
	- `RPCS3_TEST_ROOT`: The absolute path to the RPCS3 app data directory.
	- `RPCS3_TEST_RPCN_ID`: The expected RPCN ID. Used to verify the parsed value.
	*/
	#[ignore]
	#[test]
	fn rpcnId()
	{
		let rootDir = env::var("RPCS3_TEST_ROOT").unwrap();
		let expected = env::var("RPCS3_TEST_RPCN_ID").unwrap();
		
		let api = Api { rootDir, ..Default::default() };
		let rpcnId = api.getRpcnId();
		assert!(rpcnId.is_ok());
		assert_eq!(rpcnId.unwrap(), expected);
	}
	
	/**
	Requires several environment variables to be set in order to run successfully.
	
	- `RPCS3_TEST_ROOT`: The absolute path to the RPCS3 app data directory.
	- `RPCS3_TEST_ACCOUNTID`: An integer value matching the relevant RPCS3 account id.
	- `RPCS3_TEST_NPCOMMID`: An NpCommId representing the game whose trophy data should be used in this test.
	*/
	#[ignore]
	#[test]
	fn trophyList()
	{
		let rootDir = env::var("RPCS3_TEST_ROOT").unwrap();
		let accountIdString = env::var("RPCS3_TEST_ACCOUNTID").unwrap();
		let accountId = accountIdString.parse::<u64>().unwrap();
		let npCommId = env::var("RPCS3_TEST_NPCOMMID").unwrap();
		
		let api = Api { rootDir, accountId, };
		let trophies = api.parseTrophies(npCommId.to_owned()).unwrap();
		
		assert_ne!(trophies.len(), 0);
		if let Some((metadata, type6)) = trophies.first()
		{
			let grade: TrophyGrade = metadata.ttype.clone().into();
			assert_eq!(metadata.id, 0);
			assert_ne!(grade, TrophyGrade::Unknown);
			assert!(!metadata.detail.is_empty());
			assert_ne!(metadata.hidden, TrophyMetadata::HiddenTrue);
			assert!(!metadata.name.is_empty());
			assert_eq!(type6.timestamp1, 0);
			assert_eq!(type6.timestamp2, 0);
			
			match grade == TrophyGrade::Platinum
			{
				false => assert_ne!(metadata.pid, -1),
				true => assert_eq!(metadata.pid, -1),
			}
		}
	}
}
