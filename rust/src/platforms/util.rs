#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::path::Path;
use ::anyhow::{Context, Result};
use ::reqwest::Client;
use crate::io::saveImageToCache;

/**
Retrieve the image from a `url` and store it in the cache directory.
*/
pub async fn doCacheImage(client: &Client, url: String, platform: String, group: String, filename: String) -> Result<()>
{
	let response = client.get(&url)
		.send().await
			.context(format!("Error retrieving image at url: {}", url))?
		.bytes().await
			.context(format!("Error converting the image response into an instance of Bytes for url: {}", url))?;
	
	saveImageToCache(platform, group, filename, response.as_ref())
		.context(format!("Error saving image to file from url: {}", url))?;
	
	return Ok(());
}

/**

*/
pub async fn cacheImage(client: &Client, url: String, path: String, platform: String, group: String, filename: String, force: bool) -> Result<()>
{
	if force || !Path::new(&path).exists()
	{
		doCacheImage(client, url, platform, group, filename.to_owned()).await?;
	}
	
	return Ok(());
}
