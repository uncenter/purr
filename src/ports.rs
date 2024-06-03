use std::{env, fs, io, path};

use color_eyre::eyre::{bail, Context, Result};
use convert_case::Casing;
use inquire::validator::Validation;
use inquire::Text;
use serde_json::Value;
use url::Url;

use crate::cli::{Key, Query, WhiskersCustomProperty};
use crate::github::{
	self, fetch_all_repositories, fetch_whiskers_custom_property, RepositoryResponse,
};
use crate::models::ports::Root;
use crate::models::shared::StringOrStrings;
use crate::{display_json_or_count, get_key, is_booleanish_match, matches_current_maintainer};

pub fn query(command: Option<Query>, r#for: Option<String>, count: bool, get: Key) -> Result<()> {
	let raw: String = reqwest::blocking::get(
		"https://github.com/catppuccin/catppuccin/raw/main/resources/ports.yml",
	)?
	.text()?;
	let data: Root = serde_yaml::from_str(&raw).unwrap();

	match command {
		Some(Query::Maintained { by, options }) => {
			let result = data
				.ports
				.into_iter()
				.filter(|port| {
					let current_maintainers = &port.1.current_maintainers;
					let matches = matches_current_maintainer(current_maintainers, &by);

					if options.not {
						!matches
					} else {
						matches
					}
				})
				.map(|port| get_key(port, options.get))
				.collect::<Vec<_>>();

			display_json_or_count(&result, options.count)?;
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
			options,
		}) => {
			let result = data
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
							*upstreamed == port.1.upstreamed.unwrap_or(false)
						} else {
							true
						}
					} && {
						if let Some(platform) = &platform {
							platform.iter().all(|p| match &port.1.platform {
								StringOrStrings::Single(platform) => *platform == *p,
								StringOrStrings::Multiple(platforms) => platforms.contains(p),
							})
						} else {
							true
						}
					} && {
						if let Some(categories) = &categories {
							categories.iter().all(|c| port.1.categories.contains(c))
						} else {
							true
						}
					} && {
						if let Some(icon) = &icon {
							let value = &port.1.icon;
							is_booleanish_match(value.clone(), icon)
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
							is_booleanish_match(value.to_owned(), alias)
						} else {
							true
						}
					} && {
						if let Some(url) = &url {
							is_booleanish_match(port.1.url.clone(), url)
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
				.map(|port| get_key(port, options.get))
				.collect::<Vec<_>>();

			display_json_or_count(&result, options.count)?;
		}
		Some(Query::Stars {
			r#for,
			archived,
			token,
		}) => {
			if let Some(repository) = r#for {
				let data = github::rest(&format!("repos/catppuccin/{repository}"), Some(token))?
					.json::<RepositoryResponse>()?;

				println!("{}", data.stargazers_count);
			} else {
				let repositories = fetch_all_repositories(&token)?;

				let stars: i64 = repositories
					.iter()
					.flatten()
					.filter_map(|r| {
						let count = r.stargazer_count;
						let matches = archived == r.is_archived;

						if matches {
							Some(count)
						} else {
							None
						}
					})
					.sum();

				println!("{stars}");
			}
		}
		Some(Query::Whiskers {
			r#for,
			is,
			not,
			count,
			token,
		}) => {
			if let Some(repository) = r#for {
				let status = fetch_whiskers_custom_property(&repository, token)?;
				println!(
					"{}",
					if let Some(is) = is {
						let matches = status == is.to_string();
						if not { !matches } else { matches }.to_string()
					} else {
						status
					}
				);
			} else {
				let mut found_true = 0;
				let mut found_false = 0;
				let mut found_na = 0;

				let repositories = fetch_all_repositories(&token)?;
				let result = repositories
					.iter()
					.flatten()
					.filter(|repo| !repo.is_archived)
					.filter_map(|repository| {
						let status =
							fetch_whiskers_custom_property(&repository.name, token.clone())
								.unwrap();

						if status == WhiskersCustomProperty::True.to_string() {
							found_true += 1;
						} else if status == WhiskersCustomProperty::False.to_string() {
							found_false += 1;
						} else {
							found_na += 1;
						}

						if let Some(is) = is {
							if status == is.to_string() {
								Some(Value::String(repository.name.to_string()))
							} else {
								None
							}
						} else {
							None
						}
					})
					.collect::<Vec<_>>();

				if is.is_none() {
					println!(
						"true: {}, false: {}, n/a: {} ({:.2}%)",
						found_true,
						found_false,
						found_na,
						(found_true as f32 / (found_true + found_false) as f32) * 100.0
					);
				} else {
					display_json_or_count(&result, count)?;
				}
			}
		}
		None => {
			if let Some(r#for) = r#for {
				println!(
					"{}",
					serde_json::to_string_pretty(
						data.ports
							.into_iter()
							.filter(|port| port.0 == r#for)
							.map(|port| get_key(port, get))
							.collect::<Vec<_>>()
							.first()
							.unwrap()
					)
					.context("Failed to serialize results")?
				);
			} else {
				display_json_or_count(
					&data
						.ports
						.into_iter()
						.map(|port| get_key(port, get))
						.collect::<Vec<_>>(),
					count,
				)?;
			}
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

	let url = url.unwrap_or_else(|| {
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

	let target = env::current_dir()?.join(path::PathBuf::from(&name_kebab));
	if target.exists() {
		bail!("Directory already exists",)
	}
	let response = github::rest("repos/catppuccin/template/tarball", None)?;

	let temp = env::temp_dir();
	let tarball = temp.join("repo.tar.gz");
	let mut tarball_file = fs::File::create(&tarball)?;
	io::copy(&mut response.bytes()?.as_ref(), &mut tarball_file)?;
	let tar_gz = fs::File::open(tarball)?;
	let tar = flate2::read::GzDecoder::new(tar_gz);
	let mut archive = tar::Archive::new(tar);
	let temp_unpacked = temp.join("unpacked");
	archive.unpack(&temp_unpacked)?;

	for entry in fs::read_dir(&temp_unpacked)? {
		let entry = entry?;
		let path = entry.path();
		fs::rename(path, &target)?;
	}

	let readme = &target.join("README.md");
	let contents = fs::read_to_string(readme)?
		.replace(
			"<a href=\"https://github.com/catppuccin/template\">App</a>",
			&format!("<a href=\"{url}\">{name}</a>"),
		)
		.replace(
			"catppuccin/template",
			&format!("catppuccin/{}", &name_kebab),
		)
		.replace(
			"https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/previews/",
			"assets/",
		);
	fs::write(readme, contents)?;

	Ok(())
}
