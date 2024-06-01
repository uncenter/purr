use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shared::{Maintainer, StringOrStrings};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
	pub collaborators: Vec<Maintainer>,
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserstylesRoot {
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Userstyle {
	pub name: StringOrStrings,
	pub categories: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon: Option<String>,
	pub color: String,
	pub readme: Readme,
	pub current_maintainers: Vec<Maintainer>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub past_maintainers: Option<Vec<Maintainer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Readme {
	pub app_link: StringOrStrings,
}
