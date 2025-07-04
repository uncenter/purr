use std::collections::HashMap;

use color_eyre::eyre::{eyre, Context, Result};
use serde_json::Value;

use crate::cache::Cache;
use crate::cli::{Key, Query, WhiskersCustomProperty};
use crate::github::{self, fetch_all_repositories, fetch_whiskers_status, RepositoryResponse};
use crate::models::{self, ports::Port};
use crate::utils::fetch_yaml;

mod utils;
use utils::{display_json_or_count, get_key, is_booleanish_match, matches_current_maintainer};

pub fn query(
	cache: &mut Cache,
	command: Option<Query>,
	r#for: Option<String>,
	count: bool,
	get: Vec<Key>,
	include_userstyles: bool,
	only_userstyles: bool,
) -> Result<()> {
	let ports = cache
		.get_or("ports-yml", || {
			fetch_yaml::<models::ports::Root>(
				"https://github.com/catppuccin/catppuccin/raw/main/resources/ports.yml",
			)
		})?
		.ports
		.into_iter()
		.collect::<Vec<_>>();

	let userstyles = cache
		.get_or("userstyles-yml", || {
			fetch_yaml::<models::userstyles::Root>(
				"https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml",
			)
		})?
		.userstyles
		.into_iter()
		.map(|(key, userstyle)| (key, Port::from(userstyle)))
		.collect::<Vec<_>>();

	let data = if only_userstyles {
		userstyles
	} else if include_userstyles {
		[ports, userstyles].concat()
	} else {
		ports
	}
	.into_iter();

	fn extract_port_data(port: (String, Port), keys: &[Key]) -> HashMap<Key, Value> {
		keys.iter()
			.map(|&key| (key, get_key(port.clone(), key)))
			.collect()
	}

	match command {
		Some(Query::Maintained { by, options }) => {
			let result = data
				.filter(|port| {
					let current_maintainers = &port.1.current_maintainers;
					let matches = matches_current_maintainer(current_maintainers, &by);

					if options.not {
						!matches
					} else {
						matches
					}
				})
				.map(|port| extract_port_data(port, &options.get))
				.collect::<Vec<_>>();

			display_json_or_count(result, options.count)?;
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
							platform.iter().all(|p| port.1.platform.contains(p))
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
				.map(|port| extract_port_data(port, &options.get))
				.collect::<Vec<_>>();

			display_json_or_count(result, options.count)?;
		}
		Some(Query::Stars {
			r#for,
			archived,
			token,
		}) => {
			if let Some(repository) = r#for {
				let res = github::rest(&format!("repos/catppuccin/{repository}"), Some(token))?
					.json::<RepositoryResponse>()?;

				println!("{}", res.stargazers_count);
			} else {
				let repositories = fetch_all_repositories(cache, &token)?;

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
				let status = fetch_whiskers_status(cache, &repository, token)?;

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

				let repositories = fetch_all_repositories(cache, &token)?;
				let result = repositories
					.iter()
					.flatten()
					.filter(|repo| !repo.is_archived)
					.filter_map(|repository| {
						let status =
							fetch_whiskers_status(cache, &repository.name, token.clone()).unwrap();

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
					// TODO: Improve Whiskers status output formatting.
					println!(
						"true: {}, false: {}, n/a: {} ({:.2}%)",
						found_true,
						found_false,
						found_na,
						(found_true as f32 / (found_true + found_false) as f32) * 100.0
					);
				} else {
					display_json_or_count(result, count)?;
				}
			}
		}
		None => {
			if let Some(r#for) = r#for {
				println!(
					"{}",
					serde_json::to_string_pretty(
						data.filter(|port| port.0.to_lowercase() == r#for.to_lowercase())
							.map(|port| extract_port_data(port, &get))
							.collect::<Vec<_>>()
							.first()
							.ok_or_else(|| eyre!("no port with the name '{}'", r#for))?
					)
					.context("Failed to serialize results")?
				);
			} else {
				display_json_or_count(
					data.map(|port| extract_port_data(port, &get))
						.collect::<Vec<_>>(),
					count,
				)?;
			}
		}
	}

	Ok(())
}
