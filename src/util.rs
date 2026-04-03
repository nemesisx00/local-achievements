use std::fs::exists;

use anyhow::{Context, Result};
use reqwest::Client;
use crate::io::saveImageToCache;
use crate::net::limiter::request::FileLocation;

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

pub fn truncateF32(value: impl Into<f32>, decimalPlaces: impl Into<i32>) -> f32
{
	let pow = 10u32.pow(decimalPlaces.into() as u32) as f32;
	return (value.into() * pow).round() / pow;
}

#[allow(unused)]
pub fn truncateF64(value: impl Into<f64>, decimalPlaces: impl Into<i32>) -> f64
{
	let pow = 10u32.pow(decimalPlaces.into() as u32) as f64;
	return (value.into() * pow).round() / pow;
}
