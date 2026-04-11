use std::fs::{self, File, create_dir_all, exists};
use std::io::{BufReader, BufWriter, Cursor, ErrorKind, Write};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use image::ImageReader;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::constants::{SecretsKeyFileName, SecretsVaultFileName};
use crate::io::FileLocation;
use crate::settings::AppSettings;

const Application: &str = "local-achievements";
const Organization: &str = "";
const Qualifier: &str = "";

/**
Retrieve the image from a `url` and store it in the cache directory.
*/
pub async fn cacheImage(
		client: &Client,
		url: &String,
		destination: &FileLocation
	) -> Result<()>
{
	let response = client.get(url)
		.send().await
			.context(format!("Error retrieving image at url: {}", url))?
		.bytes().await
			.context(format!("Error converting the image response into an instance of Bytes for url: {}", url))?;
	
	// Validate that the response is actually an image
	let _image = ImageReader::new(Cursor::new(response.as_ref()))
		.with_guessed_format()
			.context("Error guessing image format based on response")?
		.decode()
			.context("Error decoding response as an image")?;
	
	saveImageToCache(destination, response.as_ref())
		.context(format!("Error saving image to file from url: {}", url))?;
	
	return Ok(());
}

pub fn filePathExists(path: &Option<String>) -> bool
{
	return path.clone()
		.is_some_and(|p| exists(p)
			.is_ok_and(|b| b));
}

pub fn generateImageCacheDir(
	platform: &String,
	group: &String
) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let path = Path::new(dir.as_str())
			.join(platform.to_lowercase())
			.join(group);
		
		if !path.exists()
		{
			create_dir_all(&path)?;
		}
	}
	
	return Ok(());
}

/**
Get the cache directory specific to this application.
*/
pub fn getCacheDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.cache_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

/**
Get the configuration directory specific to this application.
*/
pub fn getConfigDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.config_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

/**
Get the data directory specific to this application.
*/
pub fn getDataDir(create: bool) -> Option<String>
{
	let dirs = ProjectDirs::from(Qualifier, Organization, Application)?;
	let path = dirs.data_dir().to_str()?.to_string();
	
	if create
	{
		_ = create_dir_all(&path);
	}
	
	return Some(path);
}

pub fn getSecretsKeyPath() -> Option<PathBuf>
{
	return Some(Path::new(&getConfigDir(true)?)
		.join(SecretsKeyFileName));
}

pub fn getSecretsVaultPath() -> Option<PathBuf>
{
	return Some(Path::new(&getConfigDir(true)?)
		.join(SecretsVaultFileName));
}

pub fn imagePathExists(location: &FileLocation) -> bool
{
	return match getCacheDir(false)
	{
		None => false,
		Some(dir) => Path::new(dir.as_str())
			.join(location.platform.to_lowercase())
			.join(location.group.clone())
			.join(location.fileName.clone())
			.exists()
	};
}

/**
Get the absolute path to a cached image, if it exists.
*/
pub fn getImagePath(location: &FileLocation) -> Option<String>
{
	return Some(
		Path::new(getCacheDir(false)?.as_str())
			.join(location.platform.to_lowercase())
			.join(location.group.clone())
			.join(location.fileName.clone())
			.to_str()?
			.into()
	);
}

/**
Read the application's settings data from file.
*/
pub fn loadAppSettings() -> Result<AppSettings>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => readDataFromFile(dir, AppSettings::FileName.into()),
	};
}

/**
Generic method to read data from file and deserialize it into a given type which
implements `DeserializeOwned`.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
*/
pub fn readDataFromFile<T>(directory: String, fileName: String) -> Result<T>
	where T: DeserializeOwned
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let file = File::open(&path)
		.context(format!(
			"Failed opening file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	let reader = BufReader::new(file);
	
	return Ok(serde_json::from_reader(reader)
		.context(format!(
			"Failed parsing as JSON file at: '{}'",
			path.as_path().to_str().unwrap()
		))?);
}

/**
Read data from a file.

This is primarily used to preserve data when deserializing to a specific type
fails.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
*/
#[allow(unused)]
pub fn readRawDataFromFile(directory: String, fileName: String) -> Result<String>
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let json = fs::read_to_string(&path)
		.context(format!(
			"Failed opening file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	return Ok(json);
}

/**
Write the application's settings data to file.
*/
pub fn saveAppSettings(settings: &AppSettings) -> Result<()>
{
	return match getConfigDir(false)
	{
		None => Err(anyhow!(ErrorKind::NotFound)),
		Some(dir) => writeDataToFile(dir, AppSettings::FileName.into(), settings),
	};
}

/**
Save an image to file in the cache directory specific to this application.
*/
pub fn saveImageToCache(
		destination: &FileLocation,
		buffer: &[u8]
	) -> Result<()>
{
	if let Some(dir) = getCacheDir(true)
	{
		let mut path = Path::new(dir.as_str())
			.join(destination.platform.to_lowercase())
			.join(destination.group.clone());
		
		if !path.exists()
		{
			_ = create_dir_all(&path);
		}
		
		path = path.join(destination.fileName.clone());
		
		let mut file = File::create(&path)
			.context(format!(
				"Error opening file for writing: {}/{}",
				destination.platform.to_lowercase(),
				destination.fileName
			))?;
		
		file.write_all(buffer)
			.context(format!(
				"Error writing to file: {}/{}",
				destination.platform.to_lowercase(),
				destination.fileName
			))?;
		
		return Ok(());
	}
	
	return Err(anyhow!(ErrorKind::NotFound));
}

/**
Generic method to write a given type which implements `Serialize` to file.

## Parameters
- directory: `String` Absolute path to the directory which contains the desired file.
- fileName: `String` File name with extension of the desired file.
- data: `&T` The data to be written.
*/
pub fn writeDataToFile<T>(directory: String, fileName: String, data: &T) -> Result<()>
	where T: Serialize
{
	let path = Path::new(directory.as_str())
		.join(fileName);
	
	let file = File::create(&path)
		.context(format!(
			"Failed creating or truncating the file at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	let buffer = BufWriter::new(file);
	
	serde_json::to_writer(buffer, data)
		.context(format!(
			"serde_json failed writing data to BufWriter at: '{}'",
			path.as_path().to_str().unwrap()
		))?;
	
	return Ok(());
}
