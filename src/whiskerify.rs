use std::{fs, path::PathBuf};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use log::warn;

use fancy_regex::Regex;

pub fn convert(path: PathBuf, dry_run: bool) -> Result<()> {
	let mut contents = fs::read_to_string(&path)?;

	let mut possible_rgbs: Vec<(&str, [u8; 4])> = vec![];

	let temp = contents.clone();
	for m in Regex::new("rgba?\\(.*\\)").unwrap().captures_iter(&temp) {
		let text = m.unwrap().get(0).unwrap().as_str();
		let color = csscolorparser::parse(text).unwrap();

		possible_rgbs.push((text, color.to_rgba8()));
	}

	for flavor in catppuccin::PALETTE.all_flavors() {
		contents = contents.replace(
			&flavor.name.to_string(),
			&format!("{} {} {}", "{{", "flavor.name", "}}"),
		);
		contents = contents.replace(
			&flavor.identifier().to_string(),
			&format!("{} {} {}", "{{", "flavor.identifier", "}}"),
		);

		for color in &flavor.colors {
			contents = contents.replace(
				&color.hex.to_string(),
				&format!("#{} {}.{} {}", "{{", color.identifier(), "hex", "}}"),
			);

			for (text, values) in possible_rgbs.clone() {
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
						text,
						format!("{} {}{} {}", "{{", color.identifier(), filters, "}}").as_str(),
					);
					possible_rgbs.retain(|x| *x.0 != *text);
				}
			}
		}
	}

	for (text, _) in possible_rgbs {
		let (line_number, line_content) = contents
			.lines()
			.enumerate()
			.find(|(_i, line)| line.contains(text))
			.unwrap();

		warn!(
			"could not replace non-Catppuccin color '{}' at {}:{}:{}",
			text.yellow(),
			path.to_string_lossy(),
			line_number + 1,
			line_content.find(text).unwrap() + 1
		);
	}

	if dry_run {
		println!("{contents}");
	} else {
		fs::write(path, contents)?;
	}

	Ok(())
}
