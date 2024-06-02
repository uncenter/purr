use std::{fs, path::PathBuf};

use color_eyre::eyre::Result;
use fancy_regex::Regex;

pub fn convert(path: PathBuf, dry_run: bool) -> Result<()> {
	let mut contents = fs::read_to_string(&path)?;

	let mut rgb_color_matches: Vec<(&str, [u8; 4])> = vec![];

	let temp = contents.clone();
	for m in Regex::new("rgba?\\(.*\\)").unwrap().captures_iter(&temp) {
		let text = m.unwrap().get(0).unwrap().as_str();
		let color = csscolorparser::parse(text).unwrap();

		rgb_color_matches.push((text, color.to_rgba8()));
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

			for (text, values) in &rgb_color_matches {
				if color.rgb.r == values[0] && color.rgb.g == values[1] && color.rgb.b == values[2]
				{
					let opacity = values[3];

					let filters = if opacity != 255 {
						format!(
							" | mod(opacity={:.2}) | css_rgba",
							opacity as f32 / 255 as f32
						)
					} else {
						" | css_rgb".to_string()
					};

					contents = contents.replace(
						text,
						format!("{} {}{} {}", "{{", color.identifier(), filters, "}}").as_str(),
					);
				}
			}
		}
	}

	if dry_run {
		println!("{}", contents);
	} else {
		fs::write(path, contents)?;
	}

	Ok(())
}
