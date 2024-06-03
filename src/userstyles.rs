use std::collections::HashMap;
use std::{env, fs, path};

use color_eyre::eyre::{bail, Context, Result};
use convert_case::Casing;
use inquire::validator::Validation;
use inquire::{MultiSelect, Select, Text};
use url::Url;

use crate::cli::{UserstyleKey, UserstylesQuery};
use crate::models::shared::{StringOrStrings, CATEGORIES};
use crate::models::userstyles::{Readme, Root, Userstyle, UserstylesRoot};
use crate::{
	display_json_or_count, get_userstyle_key, is_booleanish_match, matches_current_maintainer,
};

pub fn query(
	command: Option<UserstylesQuery>,
	r#for: Option<String>,
	count: bool,
	get: UserstyleKey,
) -> Result<()> {
	let raw: String = reqwest::blocking::get(
		"https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml",
	)?
	.text()?;
	let data: Root = serde_yaml::from_str(&raw).unwrap();

	match command {
		Some(UserstylesQuery::Maintained { by, options }) => {
			let result = data
				.userstyles
				.into_iter()
				.filter(|userstyle| {
					let current_maintainers = &userstyle.1.current_maintainers;
					let matches = matches_current_maintainer(current_maintainers, &by);

					if options.not {
						!matches
					} else {
						matches
					}
				})
				.map(|userstyle| get_userstyle_key(userstyle, options.get))
				.collect::<Vec<_>>();

			display_json_or_count(&result, options.count)?;
		}
		Some(UserstylesQuery::Has {
			name,
			categories,
			icon,
			color,
			app_link,
			options,
		}) => {
			let result = data
				.userstyles
				.into_iter()
				.filter(|userstyle| {
					let matches: bool = {
						if let Some(name) = &name {
							*name == userstyle.0
								|| match &userstyle.1.name {
									StringOrStrings::Multiple(n) => n.contains(name),
									StringOrStrings::Single(n) => *n == *name,
								}
						} else {
							true
						}
					} && {
						if let Some(categories) = &categories {
							categories
								.iter()
								.all(|c| userstyle.1.categories.contains(c))
						} else {
							true
						}
					} && {
						if let Some(icon) = &icon {
							let value = &userstyle.1.icon;
							is_booleanish_match(value.clone(), icon)
						} else {
							true
						}
					} && {
						if let Some(color) = &color {
							color
								.parse()
								.unwrap_or_else(|_| *color == userstyle.1.color)
						} else {
							true
						}
					} && {
						if let Some(app_link) = &app_link {
							app_link.parse().unwrap_or_else(|_| {
								match &userstyle.1.readme.app_link {
									StringOrStrings::Multiple(l) => l.contains(app_link),
									StringOrStrings::Single(l) => *l == *app_link,
								}
							})
						} else {
							true
						}
					};

					if options.not {
						!matches
					} else {
						matches
					}
				})
				.map(|userstyle| get_userstyle_key(userstyle, options.get))
				.collect::<Vec<_>>();

			display_json_or_count(&result, options.count)?;
		}
		None => {
			if let Some(r#for) = r#for {
				println!(
					"{}",
					serde_json::to_string_pretty(
						data.userstyles
							.into_iter()
							.filter(|port| port.0 == r#for)
							.map(|port| get_userstyle_key(port, get))
							.collect::<Vec<_>>()
							.first()
							.unwrap()
					)
					.context("Failed to serialize results")?
				);
			} else {
				display_json_or_count(
					&data
						.userstyles
						.into_iter()
						.map(|port| get_userstyle_key(port, get))
						.collect::<Vec<_>>(),
					count,
				)?;
			}
		}
	}

	Ok(())
}

pub fn init(
	name: Option<String>,
	categories: Option<Vec<String>>,
	icon: Option<String>,
	color: Option<String>,
	url: Option<String>,
) -> Result<()> {
	let cwd = env::current_dir()?;
	if !cwd
		.join(path::PathBuf::from("scripts/userstyles.yml"))
		.exists()
	{
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
			CATEGORIES.to_vec(),
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
				.map(catppuccin::Color::identifier)
				.collect(),
		)
		.prompt()
		.unwrap()
		.to_string()
	});

	let url = url.unwrap_or_else(|| {
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

	let target = cwd.join(path::PathBuf::from("styles/".to_string() + &name_kebab));
	if target.exists() {
		bail!("Userstyle already exists",)
	}
	fs::create_dir(&target)?;

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
		Url::parse(&url)?
			.host_str()
			.expect("App link should be a valid URL"),
	);

	let comment_re =
		fancy_regex::Regex::new(r"\/\*(?:(?!\*\/|==UserStyle==|prettier-ignore)[\s\S])*?\*\/")?;
	template = comment_re.replace_all(&template, "").to_string();

	fs::write(
		target.join(path::PathBuf::from("catppuccin.user.css")),
		&template,
	)?;

	let metadata = Userstyle {
		name: StringOrStrings::Single(name),
		categories,
		icon,
		color,
		readme: Readme {
			app_link: StringOrStrings::Single(url),
		},
		current_maintainers: vec![],
		past_maintainers: None,
	};
	let mut bare = HashMap::new();
	bare.insert(name_kebab, metadata);
	println!(
		"{}",
		serde_yaml::to_string(&UserstylesRoot { userstyles: bare }).unwrap()
	);

	Ok(())
}
