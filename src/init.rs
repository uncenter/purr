use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

use color_eyre::eyre::{bail, Result};
use convert_case::Casing;
use fancy_regex::Regex;
use inquire::validator::Validation;
use inquire::{Confirm, MultiSelect, Select, Text};
use url::Url;

use crate::cache::Cache;
use crate::models::shared::{StringOrStrings, CATEGORIES};
use crate::models::userstyles::{Readme, Userstyle, UserstylesRoot};
use crate::{fetch_text, github};

pub fn port(name: Option<String>, url: Option<String>, whiskers: Option<bool>) -> Result<()> {
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

	let whiskers = whiskers.unwrap_or_else(|| Confirm::new("Use Whiskers?").prompt().unwrap());

	let target = env::current_dir()?.join(PathBuf::from(&name_kebab));
	if target.exists() {
		bail!("Directory already exists",)
	}
	let response = github::rest(
		&format!(
			"repos/{}/tarball",
			if whiskers {
				"uncenter/ctp-template-whiskers"
			} else {
				"catppuccin/template"
			}
		),
		None,
	)?;

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

	let git_user_name = String::from_utf8(
		Command::new("git")
			.args(["config", "user.name"])
			.output()
			.expect("failed to execute git process")
			.stdout,
	)?;
	let username = git_user_name.trim();

	let readme = &target.join("README.md");
	let mut readme_contents = fs::read_to_string(readme)?
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
		)
		.replace(
			"[Human](https://github.com/catppuccin)",
			&format!("[{}](https://github.com/{})", username, username),
		);

	if whiskers {
		let template_path = format!("{}.tera", name_kebab);
		fs::rename(&target.join("app.tera"), &target.join(&template_path))?;

		let justfile = &target.join("justfile");
		fs::write(
			justfile,
			fs::read_to_string(justfile)?.replace("app.tera", &template_path),
		)?;

		readme_contents = readme_contents.replace("app.tera", &template_path);
	}

	fs::write(readme, readme_contents)?;
	fs::remove_file(&target.join("assets/.gitkeep"))?;

	Ok(())
}

pub fn userstyle(
	cache: &mut Cache,
	name: Option<String>,
	categories: Option<Vec<String>>,
	icon: Option<String>,
	color: Option<String>,
	url: Option<String>,
	clear_comments: bool,
) -> Result<()> {
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

	let target = cwd.join(PathBuf::from("styles/".to_string() + &name_kebab));
	if target.exists() {
		bail!("Userstyle already exists",)
	}
	fs::create_dir(&target)?;

	let mut template = cache
		.get_or("userstyles-template", || {
			fetch_text(
				"https://github.com/catppuccin/userstyles/raw/main/template/catppuccin.user.less",
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

	if clear_comments {
		let comment_re =
			Regex::new(r"(?m)^ +\/\*(?:(?!\*\/|==UserStyle==|deno-fmt-ignore)[\s\S])*?\*\/\n")?;
		template = comment_re.replace_all(&template, "").to_string();
	}

	fs::write(
		target.join(PathBuf::from("catppuccin.user.less")),
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
