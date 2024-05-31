use clap::{arg, Args, Parser, Subcommand, ValueEnum};
use color_eyre::owo_colors::OwoColorize;
use url::Url;

use crate::models::shared::CATEGORIES;

#[derive(Parser)]
#[command(version, arg_required_else_help(true))]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Query {
		#[command(subcommand)]
		command: Option<Query>,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, value_enum, default_value_t)]
		get: Key,
	},
	Init {
		#[arg(long)]
		name: Option<String>,

		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
	Userstyles {
		#[command(subcommand)]
		command: Userstyles,
	},
}

#[derive(Subcommand)]
pub enum Userstyles {
	Query {
		#[command(subcommand)]
		command: Option<UserstylesQuery>,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, value_enum, default_value_t)]
		get: UserstyleKey,
	},
	Init {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category", value_delimiter = ',', value_parser = valid_category)]
		categories: Option<Vec<String>>,

		#[arg(long)]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
}

#[derive(Subcommand)]
pub enum Query {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[command(flatten)]
		options: ExtraOptions,
	},
	Whiskers {
		#[arg(long, env = "GITHUB_TOKEN")]
		token: String,

		#[arg(short, long, name = "STATE")]
		is: WhiskersCustomProperty,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, conflicts_with = "count")]
		percentage: bool,
	},
	Stars {
		#[arg(long, name = "REPOSITORY", conflicts_with = "archived")]
		r#for: Option<String>,

		#[arg(long)]
		archived: bool,

		#[arg(long, env = "GITHUB_TOKEN")]
		token: String,
	},
	Has {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category", value_delimiter = ',', value_parser = valid_category)]
		categories: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		upstreamed: Option<bool>,

		#[arg(long)]
		platform: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		alias: Option<String>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true", value_parser = valid_url)]
		url: Option<String>,

		#[command(flatten)]
		options: ExtraOptions,
	},
}

#[derive(Subcommand)]
pub enum UserstylesQuery {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[command(flatten)]
		options: ExtraUserstyleOptions,
	},
	Has {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category", value_delimiter = ',', value_parser = valid_category)]
		categories: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, value_parser = valid_url)]
		app_link: Option<String>,

		#[command(flatten)]
		options: ExtraUserstyleOptions,
	},
}

#[derive(Args)]
pub struct ExtraOptions {
	#[arg(short, long)]
	pub not: bool,

	#[arg(short, long)]
	pub count: bool,

	#[arg(short, long, value_enum, default_value_t)]
	pub get: Key,
}

#[derive(Args)]
pub struct ExtraUserstyleOptions {
	#[arg(short, long)]
	pub not: bool,

	#[arg(short, long)]
	pub count: bool,

	#[arg(short, long, value_enum, default_value_t)]
	pub get: UserstyleKey,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Key {
	Identifier,
	Name,
	Categories,
	Upstreamed,
	Platform,
	Icon,
	Color,
	Alias,
	Url,
}

impl Default for Key {
	fn default() -> Self {
		Key::Identifier
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum UserstyleKey {
	Identifier,
	Name,
	Categories,
	Icon,
	Color,
	AppLink,
}

impl Default for UserstyleKey {
	fn default() -> Self {
		UserstyleKey::Identifier
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum WhiskersCustomProperty {
	True,
	False,
	NotApplicable,
}

fn valid_url(u: &str) -> Result<String, String> {
	if Url::parse(u).is_ok() {
		Ok(String::from(u))
	} else {
		Err(format!("{} is not a valid URL", u))
	}
}

fn valid_category(c: &str) -> Result<String, String> {
	if CATEGORIES.contains(&c) {
		Ok(String::from(c))
	} else {
		use std::cmp::Ordering;

		let mut distances = CATEGORIES
			.into_iter()
			.map(|category: &str| (category, strsim::jaro(c, category)))
			.filter(|(_, confidence)| *confidence > 0.7)
			.collect::<Vec<_>>();

		distances.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
		let best = distances.first().map(|a| a.0.to_owned());

		Err(format!(
			"not a valid category{}",
			if let Some(best) = best {
				format!(". Did you mean '{}'?", best.green())
			} else {
				"".to_string()
			}
		))
	}
}
