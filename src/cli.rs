use std::path::PathBuf;

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

		#[arg(long, name = "PORT", conflicts_with = "count", requires = "get")]
		r#for: Option<String>,

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
	Whiskerify {
		path: PathBuf,

		#[arg(short, long)]
		dry_run: bool,
	},
}

#[derive(Subcommand)]
pub enum Userstyles {
	Query {
		#[command(subcommand)]
		command: Option<UserstylesQuery>,

		#[arg(long, name = "USERSTYLE", conflicts_with = "count", requires = "get")]
		r#for: Option<String>,

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
		options: ExtraOptions<Key>,
	},
	Whiskers {
		#[arg(long, name = "REPOSITORY", conflicts_with_all = ["count"])]
		r#for: Option<String>,

		#[arg(short, long, name = "STATE")]
		is: Option<WhiskersCustomProperty>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long)]
		count: bool,

		#[arg(long, env = "GITHUB_TOKEN")]
		token: String,
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
		options: ExtraOptions<Key>,
	},
}

#[derive(Subcommand)]
pub enum UserstylesQuery {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[command(flatten)]
		options: ExtraOptions<UserstyleKey>,
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
		options: ExtraOptions<UserstyleKey>,
	},
}

#[derive(Args)]
pub struct ExtraOptions<K: Send + Sync + Default + ValueEnum + 'static> {
	#[arg(short, long)]
	pub not: bool,

	#[arg(short, long)]
	pub count: bool,

	#[arg(short, long, value_enum, default_value_t)]
	pub get: K,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum Key {
	#[default]
	Identifier,
	Name,
	Categories,
	Upstreamed,
	Platform,
	Icon,
	Color,
	Alias,
	Url,
	CurrentMaintainers,
	PastMaintainers,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum UserstyleKey {
	#[default]
	Identifier,
	Name,
	Categories,
	Icon,
	Color,
	AppLink,
	CurrentMaintainers,
	PastMaintainers,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum WhiskersCustomProperty {
	True,
	False,
	NotApplicable,
}

fn valid_url(url: &str) -> Result<String, String> {
	if Url::parse(url).is_ok() {
		Ok(String::from(url))
	} else {
		Err(format!("{url} is not a valid URL"))
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
				String::new()
			}
		))
	}
}
