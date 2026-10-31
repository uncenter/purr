use serde::de::DeserializeOwned;

use crate::error::{OutdatedModel, Result};

pub fn fetch_text(url: &str) -> Result<String> {
	let response = reqwest::blocking::get(url)?;
	let text = response.text()?;
	Ok(text)
}

pub fn fetch_yaml<T: DeserializeOwned>(url: &str) -> Result<T> {
	let raw = fetch_text(url)?;
	Ok(serde_yaml::from_str::<T>(&raw).map_err(|e| OutdatedModel::new(url, e))?)
}
