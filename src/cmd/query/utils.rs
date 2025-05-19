use color_eyre::eyre::{Context, Result};
use serde_json::Value;

use crate::{cli::Key, models::ports::Port};

pub fn matches_current_maintainer(current_maintainers: &[String], by: &Option<String>) -> bool {
	match &by {
		Some(by) => current_maintainers
			.iter()
			.any(|maintainer| maintainer.to_lowercase().contains(&by.to_lowercase())),
		None => !current_maintainers.is_empty(),
	}
}

pub fn display_json_or_count<T: serde::Serialize>(result: Vec<T>, count: bool) -> Result<()> {
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

pub fn is_booleanish_match(value: Option<String>, expected: &str) -> bool {
	(expected == "true" && value.is_some())
		|| (expected == "false" && value.is_none())
		|| (if let Some(value) = value {
			value == expected
		} else {
			false
		})
}

pub fn get_key((identifier, port): (String, Port), key: Key) -> Value {
	fn optional_string(value: Option<String>) -> Value {
		value.map_or(Value::Null, Value::String)
	}
	fn array_of_strings(arr: Vec<String>) -> Value {
		Value::Array(arr.into_iter().map(Value::String).collect())
	}

	match key {
		Key::Identifier => Value::String(identifier),
		Key::Name => Value::String(port.name),
		Key::Categories => array_of_strings(port.categories),
		Key::Upstreamed => port.upstreamed.map_or(Value::Null, Value::Bool),
		Key::Platform => array_of_strings(port.platform),
		Key::Icon => optional_string(port.icon),
		Key::Color => Value::String(port.color),
		Key::Alias => optional_string(port.alias),
		Key::Url => optional_string(port.url),
		Key::CurrentMaintainers => array_of_strings(port.current_maintainers),
		Key::PastMaintainers => Value::Array(
			port.past_maintainers
				.into_iter()
				.flatten()
				.map(Value::String)
				.collect(),
		),
	}
}
