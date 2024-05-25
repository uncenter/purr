use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const USERSTYLES_CATEGORIES: [&str; 35] = [
	"3d_modelling",
	"analytics",
	"application_launcher",
	"artificial_intelligence",
	"boot_loader",
	"browser",
	"browser_extension",
	"cli",
	"code_editor",
	"desktop_environment",
	"development",
	"discussion_forum",
	"document_viewer",
	"education",
	"email_client",
	"entertainment",
	"file_manager",
	"game",
	"game_development",
	"health_and_fitness",
	"library",
	"music",
	"note_taking",
	"notification_daemon",
	"photo_and_video",
	"productivity",
	"search_engine",
	"self_hosted",
	"social_networking",
	"system",
	"terminal",
	"translation_tool",
	"userstyle",
	"wiki",
	"window_manager",
];

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
	pub collaborators: Vec<Collaborator>,
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserstylesRoot {
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
	pub url: String,
	pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
	Single(String),
	Multiple(Vec<String>),
}

impl Default for StringOrStrings {
	fn default() -> Self {
		StringOrStrings::Single("".to_string())
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Userstyle {
	pub name: StringOrStrings,
	pub categories: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<String>,
	pub color: String,
	pub readme: Readme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Readme {
	pub app_link: StringOrStrings,
	pub current_maintainers: Vec<Maintainer>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub past_maintainers: Option<Vec<Maintainer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maintainer {
	pub name: Option<String>,
	pub url: String,
}
