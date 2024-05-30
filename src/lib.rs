use color_eyre::eyre::{Context, Ok, Result};
use serde_json::Value;

use cli::{Key, UserstyleKey};
use models::{
	ports::Port,
	shared::{Maintainer, StringOrStrings},
	userstyles::Userstyle,
};

pub mod cli;
pub mod github;
pub mod models;
pub mod ports;
pub mod userstyles;

fn matches_current_maintainer(current_maintainers: &Vec<Maintainer>, by: Option<String>) -> bool {
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

fn display_list_or_count(result: Vec<Value>, count: bool) -> Result<()> {
	println!(
		"{}",
		match count {
			true => result.len().to_string(),
			false =>
				serde_json::to_string_pretty(&result).context("Failed to serialize results")?,
		}
	);

	Ok(())
}

fn booleanish_match(value: Option<String>, expected: String) -> bool {
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
	}
}

pub fn get_userstyle_key(entry: (String, Userstyle), key: UserstyleKey) -> Value {
	match key {
		UserstyleKey::Identifier => Value::String(entry.0),
		UserstyleKey::Name => match entry.1.name {
			StringOrStrings::Single(name) => Value::String(name),
			StringOrStrings::Multiple(names) => {
				Value::Array(names.into_iter().map(Value::String).collect())
			}
		},
		UserstyleKey::Categories => {
			Value::Array(entry.1.categories.into_iter().map(Value::String).collect())
		}
		UserstyleKey::Icon => Value::String(entry.1.icon.expect("icon should exist")),
		UserstyleKey::Color => Value::String(entry.1.color),
		UserstyleKey::AppLink => match entry.1.readme.app_link {
			StringOrStrings::Single(link) => Value::String(link),
			StringOrStrings::Multiple(links) => {
				Value::Array(links.into_iter().map(Value::String).collect())
			}
		},
	}
}
