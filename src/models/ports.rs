use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shared::{Maintainer, StringOrStrings};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
	pub collaborators: Vec<Maintainer>,
	pub ports: HashMap<String, Port>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Port {
	pub name: String,
	pub categories: Vec<String>,
	pub upstreamed: Option<bool>,
	pub platform: StringOrStrings,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	pub links: Option<Vec<Link>>,
	pub icon: Option<String>,
	pub color: String,
	pub alias: Option<String>,
	pub current_maintainers: Vec<Maintainer>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub past_maintainers: Option<Vec<Maintainer>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
	pub name: String,
	pub color: Option<String>,
	pub icon: Option<String>,
	pub url: String,
}
