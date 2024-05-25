use std::collections::HashMap;

use clap::{arg, Parser, Subcommand, ValueEnum};
use color_eyre::eyre::{Context, Result};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
	pub collaborators: Vec<Collaborator>,
	pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collaborator {
	pub url: String,
	pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
	Name(String),
	Names(Vec<String>),
}

impl Default for StringOrStrings {
	fn default() -> Self {
		StringOrStrings::Name("".to_string())
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Userstyle {
	pub name: StringOrStrings,
	pub categories: Vec<String>,
	pub icon: Option<String>,
	pub color: String,
	pub readme: Readme,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Readme {
	#[serde(rename = "app-link")]
	pub app_link: StringOrStrings,
	#[serde(rename = "current-maintainers")]
	pub current_maintainers: Vec<Maintainer>,
	#[serde(rename = "past-maintainers")]
	pub past_maintainers: Option<Vec<Maintainer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maintainer {
	pub name: Option<String>,
	pub url: String,
}

#[derive(Parser)]
#[command(version, arg_required_else_help(true))]

struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Query {
		#[command(subcommand)]
		command: Queries,
	},
}

#[derive(Subcommand)]
enum Queries {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
	Json,
	Plain,
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = Cli::parse();

	let raw: String = reqwest::blocking::get(
		"https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml",
	)?
	.text()?;

	let data: Root = serde_yaml::from_str(&raw).unwrap();

	match args.command {
		Commands::Query { command } => match command {
			Queries::Maintained {
				by,
				not,
				count,
				output,
			} => {
				let result = data
					.userstyles
					.into_iter()
					.filter(|userstyle| {
						let current_maintainers = &userstyle.1.readme.current_maintainers;
						let matches = match &by {
							Some(by) => current_maintainers.iter().any(|maintainer| {
								maintainer
									.url
									.replace("https://github.com/", "")
									.to_lowercase()
									.contains(&by.to_lowercase()) || match &maintainer.name {
									Some(name) => name.to_lowercase().contains(&by.to_lowercase()),
									None => false,
								}
							}),
							None => !current_maintainers.is_empty(),
						};

						if not {
							!matches
						} else {
							matches
						}
					})
					.map(|userstyle| userstyle.0)
					.collect::<Vec<_>>();

				println!(
					"{}",
					match count {
						true => result.len().to_string(),
						false => match output {
							OutputFormat::Json => serde_json::to_string_pretty(&result)
								.context("Failed to serialize results")?,
							OutputFormat::Plain => result.join("\n"),
						},
					}
				);
			}
		},
	}

	Ok(())
}
