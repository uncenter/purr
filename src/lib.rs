use cli::{CountOrList, OutputFormat};
use color_eyre::eyre::{Context, Ok, Result};
use models::shared::Maintainer;

pub mod cli;
pub mod models;
pub mod ports;
pub mod userstyles;

fn matches_current_maintainer(current_maintainers: &Vec<Maintainer>, by: Option<String>) -> bool {
	match &by {
		Some(by) => current_maintainers.iter().any(|maintainer| {
			maintainer
				.url
				.replace("https://github.com/", "")
				.to_lowercase()
				.contains(&by.to_lowercase())
				|| match &maintainer.name {
					Some(name) => name.to_lowercase().contains(&by.to_lowercase()),
					None => false,
				}
		}),
		None => !current_maintainers.is_empty(),
	}
}

fn display_list_or_count(result: Vec<String>, count: bool, output: OutputFormat) -> Result<()> {
	println!(
		"{}",
		match count {
			true => result.len().to_string(),
			false => match output {
				OutputFormat::Json =>
					serde_json::to_string_pretty(&result).context("Failed to serialize results")?,
				OutputFormat::Plain => result.join("\n"),
			},
		}
	);

	Ok(())
}

fn display_has_or_list_or_count(
	result: Vec<String>,
	maybe_has: Option<CountOrList>,
	output: OutputFormat,
) -> Result<()> {
	match maybe_has {
		Some(opt) => {
			display_list_or_count(result, opt.count, output)?;
		}
		None => println!("{}", !result.is_empty()),
	}

	Ok(())
}

fn booleanish_match(value: Option<String>, expected: String) -> bool {
	(expected == "true" && value.is_some())
		|| (expected == "false" && value.is_none())
		|| (if let Some(value) = value {
			value == expected
		} else {
			false
		})
}
