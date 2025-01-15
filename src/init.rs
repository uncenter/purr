use std::collections::HashMap;
use std::{env, fs, io, path};

use color_eyre::eyre::{bail, Result};
use convert_case::Casing;
use inquire::validator::Validation;
use inquire::{MultiSelect, Select, Text};
use url::Url;

use crate::cache::Cache;
use crate::models::shared::{StringOrStrings, CATEGORIES};
use crate::models::userstyles::{Readme, Userstyle, UserstylesRoot};
use crate::{fetch_text, github};

pub fn port(name: Option<String>, url: Option<String>) -> Result<()> {
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

pub fn userstyle(
	cache: &mut Cache,
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

	let mut template = cache
		.get_or("userstyles-template", || {
			fetch_text(
				"https://github.com/catppuccin/userstyles/raw/main/template/catppuccin.user.css",
			)
		})?
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
