use serde::{Deserialize, Serialize};

use super::{ports::Port, userstyles::Userstyle};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
	Single(String),
	Multiple(Vec<String>),
}

impl Default for StringOrStrings {
	fn default() -> Self {
		StringOrStrings::Single(String::new())
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
	"news_and_journalism",
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
	"wiki",
	"window_manager",
];

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maintainer {
	pub url: String,
}

impl From<Userstyle> for Port {
	fn from(userstyle: Userstyle) -> Self {
		Port {
			name: match userstyle.name {
				StringOrStrings::Single(s) => s,
				StringOrStrings::Multiple(s) => s.join("/"),
			},
			categories: userstyle.categories,
			upstreamed: Some(false),
			platform: StringOrStrings::Single("agnostic".to_string()),
			url: Some(match userstyle.readme.app_link {
				StringOrStrings::Single(s) => s,
				StringOrStrings::Multiple(s) => s[0].clone(),
			}),
			links: None,
			icon: userstyle.icon,
			color: userstyle.color,
			alias: None,
			current_maintainers: userstyle.current_maintainers,
			past_maintainers: userstyle.past_maintainers,
		}
	}
}
