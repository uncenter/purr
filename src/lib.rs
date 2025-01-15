use color_eyre::eyre::{Context, Ok, Result};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use cli::Key;
use models::{
	ports::Port,
	shared::{Maintainer, StringOrStrings},
};

pub mod cache;
pub mod cli;
pub mod github;
pub mod init;
pub mod models;
pub mod query;
pub mod whiskerify;

fn matches_current_maintainer(current_maintainers: &[Maintainer], by: &Option<String>) -> bool {
	match &by {
		Some(by) => current_maintainers.iter().any(|maintainer| {
			maintainer
				.url
				.replace("https://github.com/", "")
				.to_lowercase()
				.contains(&by.to_lowercase())
				|| match &maintainer.name {
					Some(name) => name.to_lowercase().contains(&by.to_lowercase()),
					None => false,
				}
		}),
		None => !current_maintainers.is_empty(),
	}
}

fn display_json_or_count(result: &[Value], count: bool) -> Result<()> {
	println!(
		"{}",
		if count {
			result.len().to_string()
		} else {
			serde_json::to_string_pretty(&result).context("Failed to serialize results")?
		}
	);

	Ok(())
}

fn is_booleanish_match(value: Option<String>, expected: &str) -> bool {
	(expected == "true" && value.is_some())
		|| (expected == "false" && value.is_none())
		|| (if let Some(value) = value {
			value == expected
		} else {
			false
		})
}

pub fn get_key(entry: (String, Port), key: Key) -> Value {
	match key {
		Key::Identifier => Value::String(entry.0),
		Key::Name => Value::String(entry.1.name),
		Key::Categories => {
			Value::Array(entry.1.categories.into_iter().map(Value::String).collect())
		}
		Key::Upstreamed => Value::Bool(entry.1.upstreamed.expect("upstreamed should exist")),
		Key::Platform => match entry.1.platform {
			StringOrStrings::Single(platform) => Value::String(platform),
			StringOrStrings::Multiple(platforms) => {
				Value::Array(platforms.into_iter().map(Value::String).collect())
			}
		},
		Key::Icon => Value::String(entry.1.icon.expect("icon should exist")),
		Key::Color => Value::String(entry.1.color),
		Key::Alias => Value::String(entry.1.alias.expect("alias should exist")),
		Key::Url => Value::String(entry.1.url.expect("url exist eixst")),
		Key::CurrentMaintainers => Value::Array(
			entry
				.1
				.current_maintainers
				.into_iter()
				.map(|m| json!({ "name": m.name, "url": m.url }))
				.collect(),
		),
		Key::PastMaintainers => Value::Array(
			entry
				.1
				.past_maintainers
				.into_iter()
				.flatten()
				.map(|m| json!({ "name": m.name, "url": m.url }))
				.collect(),
		),
	}
}

fn fetch_text(url: &str) -> Result<String> {
	let response = reqwest::blocking::get(url)?;
	let text = response.text()?;
	Ok(text)
}

fn fetch_yaml<T: DeserializeOwned>(url: &str) -> Result<T> {
	let raw = fetch_text(url)?;
	return Ok(serde_yaml::from_str::<T>(&raw)?);
}
