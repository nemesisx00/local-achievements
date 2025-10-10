use std::path::Path;
use anyhow::{Context, Result};
use reqwest::Client;
use crate::io::saveImageToCache;

/**
Retrieve the image from a `url` and store it in the cache directory.
*/
pub async fn cacheImage(
		client: &Client,
		url: &String,
		platform: &String,
		group: &String,
		filename: &String,
	) -> Result<()>
{
	let response = client.get(url)
		.send().await
			.context(format!("Error retrieving image at url: {}", url))?
		.bytes().await
			.context(format!("Error converting the image response into an instance of Bytes for url: {}", url))?;
	
	saveImageToCache(platform, group, filename, response.as_ref())
		.context(format!("Error saving image to file from url: {}", url))?;
	
	return Ok(());
}

/**
Retrieve the image from a `url` and store it in the cache directory.

Only retrieves the image if it does not already exist in the cache or if `force`d.
*/
pub async fn cacheImageIfNotExists(
		client: &Client,
		url: &String,
		path: &String,
		platform: &String,
		group: &String,
		filename: &String,
		force: bool
	) -> Result<()>
{
	if force || !Path::new(&path).exists()
	{
		cacheImage(client, url, platform, group, filename).await?;
	}
	
	return Ok(());
}
