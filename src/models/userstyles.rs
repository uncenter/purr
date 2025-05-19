use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::ports::Port;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
	pub collaborators: Vec<String>,
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserstylesRoot {
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Userstyle {
	pub name: String,
	pub categories: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<String>,
	pub color: String,
	pub link: String,
	pub note: Option<String>,
	pub supports: Option<HashMap<String, SupportedWebsite>>,
	pub current_maintainers: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub past_maintainers: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportedWebsite {
	pub name: String,
	pub link: String,
}

impl From<Userstyle> for Port {
	fn from(userstyle: Userstyle) -> Self {
		Port {
			name: userstyle.name,
			categories: userstyle.categories,
			upstreamed: Some(false),
			platform: vec!["web".to_string()],
			url: Some(userstyle.link),
			links: None,
			icon: userstyle.icon,
			color: userstyle.color,
			alias: None,
			current_maintainers: userstyle.current_maintainers,
			past_maintainers: userstyle.past_maintainers,
		}
	}
}
