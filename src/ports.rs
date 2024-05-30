use std::{env, fs, path};

use color_eyre::eyre::{bail, Result};
use convert_case::Casing;
use inquire::validator::Validation;
use inquire::Text;
use url::Url;

use crate::cli::{OutputFormat, Query};
use crate::models::ports::Root;
use crate::models::shared::StringOrStrings;
use crate::{
	booleanish_match, display_has_or_list_or_count, display_list_or_count,
	matches_current_maintainer,
};

pub fn query(command: Option<Query>, count: bool, output: OutputFormat) -> Result<()> {
	let raw: String = reqwest::blocking::get(
		"https://github.com/catppuccin/catppuccin/raw/main/resources/ports.yml",
	)?
	.text()?;
	let data: Root = serde_yaml::from_str(&raw).unwrap();

	match command {
		Some(Query::Maintained {
			by,
			not,
			count,
			output,
		}) => {
			let result: Vec<String> = data
				.ports
				.into_iter()
				.filter(|port| {
					let current_maintainers = &port.1.current_maintainers;
					let matches = matches_current_maintainer(current_maintainers, by.to_owned());

					if not {
						!matches
					} else {
						matches
					}
				})
				.map(|port| port.0)
				.collect();

			display_list_or_count(result, count, output)?;
		}
		Some(Query::Has {
			name,
			categories,
			upstreamed,
			platform,
			icon,
			color,
			alias,
			url,
			output,
			not,
			result: res,
		}) => {
			let result: Vec<String> = data
				.ports
				.into_iter()
				.filter(|port| {
					let matches: bool = {
						if let Some(name) = &name {
							*name == port.0 || &port.1.name == name
						} else {
							true
						}
					} && {
						if let Some(upstreamed) = &upstreamed {
							*upstreamed == port.1.upstreamed.or(Some(false)).unwrap()
						} else {
							true
						}
					} && {
						if let Some(platform) = &platform {
							platform.into_iter().all(|p| match &port.1.platform {
								StringOrStrings::Single(platform) => *platform == *p,
								StringOrStrings::Multiple(platforms) => platforms.contains(&p),
							})
						} else {
							true
						}
					} && {
						if let Some(categories) = &categories {
							categories
								.into_iter()
								.all(|c| port.1.categories.contains(&c))
						} else {
							true
						}
					} && {
						if let Some(icon) = &icon {
							let value = &port.1.icon;
							booleanish_match(value.clone(), icon.to_string())
						} else {
							true
						}
					} && {
						if let Some(color) = &color {
							color.parse().unwrap_or_else(|_| *color == port.1.color)
						} else {
							true
						}
					} && {
						if let Some(alias) = &alias {
							let value = &port.1.alias;
							booleanish_match(value.to_owned(), alias.to_string())
						} else {
							true
						}
					} && {
						if let Some(url) = &url {
							booleanish_match(port.1.url.clone(), url.to_string())
						} else {
							true
						}
					};

					if not {
						!matches
					} else {
						matches
					}
				})
				.map(|port| port.0)
				.collect();

			display_has_or_list_or_count(result, res, output)?;
		}
		None => {
			let result: Vec<String> = data.ports.into_iter().map(|port| port.0).collect();

			display_list_or_count(result, count, output)?;
		}
	}

	Ok(())
}

pub fn init(name: Option<String>, url: Option<String>) -> Result<()> {
	let name = name.unwrap_or_else(|| {
		Text::new("What is the name of this port?")
			.prompt()
			.unwrap()
	});
	let name_kebab = name.to_case(convert_case::Case::Kebab);

	let _url = url.unwrap_or_else(|| {
		Text::new("What is the URL of this port?")
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

	let new_directory = env::current_dir()?.join(path::PathBuf::from(&name_kebab));
	if new_directory.exists() {
		bail!("Directory already exists",)
	} else {
		fs::create_dir(&new_directory)?;
	}

	/* Fetch template and write to directory */

	Ok(())
}
