use color_eyre::eyre::{bail, Context, Result};
use convert_case::Casing;
use std::{collections::HashMap, env, fs, path::PathBuf};
use url::Url;

use clap::{arg, Parser, Subcommand, ValueEnum};
use inquire::{validator::Validation, MultiSelect, Select, Text};

use serde::{Deserialize, Serialize};

const USERSTYLES_CATEGORIES: [&str; 35] = [
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
		command: Option<Queries>,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
	Init {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category")]
		categories: Option<Vec<String>>,

		#[arg(long)]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long)]
		app_link: Option<String>,
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

	match args.command {
		Commands::Query {
			command,
			count,
			output,
		} => {
			let raw: String = reqwest::blocking::get(
				"https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml",
			)?
			.text()?;
			let data: Root = serde_yaml::from_str(&raw).unwrap();

			match command {
				Some(Queries::Maintained {
					by,
					not,
					count,
					output,
				}) => {
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
										Some(name) => {
											name.to_lowercase().contains(&by.to_lowercase())
										}
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
				None => {
					let result = data
						.userstyles
						.into_iter()
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
			}
		}
		Commands::Init {
			name,
			categories,
			icon,
			color,
			app_link,
		} => {
			let cwd = env::current_dir()?;
			if !cwd.join(PathBuf::from("scripts/userstyles.yml")).exists() {
				bail!("Not in userstyles repository")
			}

			let name = name.unwrap_or_else(|| {
				Text::new("What is the name of this website?")
					.prompt()
					.unwrap()
			});
			let name_kebab = name.to_case(convert_case::Case::Kebab);

			let categories = categories.unwrap_or_else(|| {
				MultiSelect::new(
					"What categories apply to this website?",
					USERSTYLES_CATEGORIES.to_vec(),
				)
				.prompt()
				.unwrap()
				.iter()
				.map(|&s| s.to_string())
				.collect()
			});

			let color = color.unwrap_or_else(|| {
				Select::new(
					"What is the primary brand color of this website?",
					catppuccin::PALETTE
						.mocha
						.colors
						.into_iter()
						.filter(|c| c.accent)
						.map(|c| c.identifier())
						.collect(),
				)
				.prompt()
				.unwrap()
				.to_string()
			});

			let app_link = app_link.unwrap_or_else(|| {
				Text::new("What is the URL of this website?")
					.with_validator(|input: &str| {
						if Url::parse(input).is_ok() {
							Ok(Validation::Valid)
						} else {
							Ok(Validation::Invalid("Input must be a valid URL.".into()))
						}
					})
					.prompt()
					.unwrap()
			});

			let new_directory = cwd.join(PathBuf::from("styles/".to_string() + &name_kebab));
			if new_directory.exists() {
				bail!("Userstyle already exists",)
			} else {
				fs::create_dir(&new_directory)?;
			}

			let mut template: String = reqwest::blocking::get(
				"https://github.com/catppuccin/userstyles/raw/main/template/catppuccin.user.css",
			)?
			.text()?
			.replace("<port-name> Catppuccin", &format!("{} Catppuccin", &name))
			.replace(
				"Soothing pastel theme for <port-name>",
				&format!("Soothing pastel theme for {}", &name),
			)
			.replace("<port-name>", &name_kebab)
			.replace(
				"<website-domain>",
				Url::parse(&app_link)?
					.host_str()
					.expect("App link should be a valid URL"),
			);

			let comment_re = fancy_regex::Regex::new(
				r"\/\*(?:(?!\*\/|==UserStyle==|prettier-ignore)[\s\S])*?\*\/",
			)?;
			template = comment_re.replace_all(&template, "").to_string();

			fs::write(
				new_directory.join(PathBuf::from("catppuccin.user.css")),
				&template,
			)?;

			let metadata = Userstyle {
				name: StringOrStrings::Single(name),
				categories: categories,
				icon: icon,
				color: color,
				readme: Readme {
					app_link: StringOrStrings::Single(app_link),
					current_maintainers: vec![],
					past_maintainers: None,
				},
			};
			let mut bare = HashMap::new();
			bare.insert(name_kebab, metadata);
			println!(
				"{}",
				serde_yaml::to_string(&UserstylesRoot { userstyles: bare }).unwrap()
			)
		}
	}

	Ok(())
}
