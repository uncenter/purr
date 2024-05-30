use serde::{Deserialize, Serialize};

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

pub const CATEGORIES: [&str; 35] = [
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
pub struct Maintainer {
	pub name: Option<String>,
	pub url: String,
}
