use std::path::PathBuf;

use clap::{arg, Args, Parser, Subcommand, ValueEnum};
use color_eyre::owo_colors::OwoColorize;
use url::Url;

use crate::models::shared::CATEGORIES;

#[derive(Parser)]
#[command(name = "purr", version, arg_required_else_help(true))]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Query about the ports.yml data source
	Query {
		#[command(subcommand)]
		command: Option<Query>,

		/// Query data for a specific port
		#[arg(long, name = "PORT", conflicts_with = "count", requires = "get")]
		r#for: Option<String>,

		/// Count the number of results
		#[arg(short, long)]
		count: bool,

		/// Extract a specific property each result
		#[arg(short, long, value_enum, default_value_t)]
		get: Key,
	},
	/// Initialize a new port from catppuccin/template
	Init {
		/// Name of the application
		#[arg(long)]
		name: Option<String>,

		/// URL to the application
		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
	/// Userstyles-related subcommands
	Userstyles {
		#[command(subcommand)]
		command: Userstyles,
	},
	/// Convert a theme file to a Whiskers template
	Whiskerify {
		input: PathBuf,

		#[arg(short, long)]
		output: Option<PathBuf>,
	},
}

#[derive(Subcommand)]
pub enum Userstyles {
	/// Query about the userstyles.yml data source
	Query {
		#[command(subcommand)]
		command: Option<UserstylesQuery>,

		/// Query data for a specific userstyle
		#[arg(long, name = "USERSTYLE", conflicts_with = "count", requires = "get")]
		r#for: Option<String>,

		/// Count the number of results
		#[arg(short, long)]
		count: bool,

		/// Extract a specific property each result
		#[arg(short, long, value_enum, default_value_t)]
		get: UserstyleKey,
	},
	/// Initialize a new userstyle from the template
	Init {
		/// Name of the application
		#[arg(long)]
		name: Option<String>,

		/// Categories that represent the application
		#[arg(long = "category", value_delimiter = ',', value_parser = valid_category)]
		categories: Option<Vec<String>>,

		/// Icon for the application (from simpleicons.org)
		#[arg(long)]
		icon: Option<String>,

		/// Name of a Catppuccin color that matches the application's brand color
		#[arg(long)]
		color: Option<String>,

		/// URL to the application
		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
}

#[derive(Subcommand)]
pub enum Query {
	/// Query maintained ports and who maintains them
	Maintained {
		/// Name of the maintainer to search for
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[command(flatten)]
		options: ExtraOptions<Key>,
	},
	/// Query about the Whiskers migration
	Whiskers {
		/// Name of the repository to query
		#[arg(long, name = "REPOSITORY", conflicts_with_all = ["count"])]
		r#for: Option<String>,

		/// Whiskers state to check for
		#[arg(short, long, name = "STATE")]
		is: Option<WhiskersCustomProperty>,

		/// Invert matched results
		#[arg(short, long)]
		not: bool,

		/// Count the number of results
		#[arg(short, long)]
		count: bool,

		#[arg(long, env = "GITHUB_TOKEN")]
		token: String,
	},
	/// Query star counts of the whole organization or per-repository
	Stars {
		/// Name of the repository to query
		#[arg(long, name = "REPOSITORY", conflicts_with = "archived")]
		r#for: Option<String>,

		/// Whether to include archived repositories in the total count
		#[arg(long)]
		archived: bool,

		#[arg(long, env = "GITHUB_TOKEN")]
		token: String,
	},
	/// Query ports with matching fields
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
	/// Query maintained userstyles and who maintains them
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[command(flatten)]
		options: ExtraOptions<UserstyleKey>,
	},
	/// Query ports with matching fields
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
	/// Invert matched results
	#[arg(short, long)]
	pub not: bool,

	/// Count the number of results
	#[arg(short, long)]
	pub count: bool,

	/// Extract a specific property each result
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
