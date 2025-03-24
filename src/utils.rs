use color_eyre::eyre::Result;
use serde::de::DeserializeOwned;

pub fn fetch_text(url: &str) -> Result<String> {
	let response = reqwest::blocking::get(url)?;
	let text = response.text()?;
	Ok(text)
}

pub fn fetch_yaml<T: DeserializeOwned>(url: &str) -> Result<T> {
	let raw = fetch_text(url)?;
	return Ok(serde_yaml::from_str::<T>(&raw)?);
}
