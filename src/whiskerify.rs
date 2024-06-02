use std::{fs, path::PathBuf};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use log::warn;

use fancy_regex::Regex;

pub fn convert(path: PathBuf, dry_run: bool) -> Result<()> {
	let mut contents = fs::read_to_string(&path)?;

	let mut color_matches = Regex::new("rgba?\\(.*\\)")
		.unwrap()
		.captures_iter(&contents.clone())
		.map(|m| {
			let text = m.unwrap().get(0).unwrap().as_str();
			let color = csscolorparser::parse(text).unwrap();
			(text.to_string(), color.to_rgba8())
		})
		.collect::<Vec<_>>();

	for flavor in catppuccin::PALETTE.all_flavors() {
		contents = contents
			.replace(&flavor.name.to_string(), &as_tera_expr("flavor.name"))
			.replace(
				&flavor.identifier().to_string(),
				&as_tera_expr("flavor.identifier"),
			);

		for color in &flavor.colors {
			contents = contents.replace(
				&color.hex.to_string(),
				&format!(
					"#{}",
					as_tera_expr(&(color.identifier().to_string() + ".hex"))
				),
			);

			for (text, values) in color_matches.clone() {
				if color.rgb.r == values[0] && color.rgb.g == values[1] && color.rgb.b == values[2]
				{
					let opacity = values[3];

					let filters = if opacity == 255 {
						" | css_rgb".to_string()
					} else {
						format!(
							" | mod(opacity={:.2}) | css_rgba",
							f32::from(opacity) / 255_f32
						)
					};

					contents = contents.replace(
						&text,
						&as_tera_expr(&(color.identifier().to_owned() + &filters)),
					);
					color_matches.retain(|x| *x.0 != *text);
				}
			}
		}
	}

	for (text, _) in color_matches {
		warn!(
			"could not replace non-Catppuccin color '{}' at {}:{}",
			text.yellow(),
			path.to_string_lossy(),
			&get_location_in_text(&text, &contents)
		);
	}

	if dry_run {
		println!("{contents}");
	} else {
		fs::write(path, contents)?;
	}

	Ok(())
}

fn as_tera_expr(value: &str) -> String {
	format!("{} {} {}", "{{", value, "}}")
}

fn get_location_in_text(search: &str, text: &str) -> String {
	let (line_number, line_content) = text
		.lines()
		.enumerate()
		.find(|(_i, line)| line.contains(search))
		.unwrap();

	format!(
		"{}:{}",
		line_number + 1,
		line_content.find(search).unwrap() + 1
	)
}
