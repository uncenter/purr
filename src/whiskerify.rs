use std::{fs, path::PathBuf};

use catppuccin::Hsl;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use log::warn;

use fancy_regex::Regex;

pub fn handle(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
	let original: String = fs::read_to_string(&input)?;
	let new = convert(original.clone(), Some(input));

	if original == new {
		warn!("no changes made to original file");
	}

	if let Some(path) = output {
		fs::write(path, new)?;
	} else {
		println!("{new}");
	}

	Ok(())
}

pub fn convert(mut contents: String, input_path: Option<PathBuf>) -> String {
	let mut color_matches: Vec<(String, csscolorparser::Color)> = Regex::new("(rgb|hsl)a?\\(.*\\)")
		.unwrap()
		.captures_iter(&contents.clone())
		.filter_map(|m| {
			let text = m.unwrap().get(0).unwrap().as_str();
			let color = match csscolorparser::parse(text) {
				Ok(c) => c,
				Err(_) => {
					warn!("invalid color '{}'", text);
					return None;
				}
			};

			Some((text.to_string(), color))
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
			let pat = "(?i)".to_string() + &color.hex.to_string()[1..];
			let search = Regex::new(&pat).unwrap();

			for result in search.find_iter(&contents.clone()).flatten() {
				contents = contents.replace(
					result.as_str(),
					&as_tera_expr(&(color.identifier().to_string() + ".hex")),
				);
			}

			for (text, color_match) in color_matches.clone() {
				let res = if text.contains("hsl") {
					let expected = hsl_to_vec(&color.hsl)
						.into_iter()
						.map(round_to_two_decimal_places)
						.collect::<Vec<_>>();

					let mut values =
						<(f64, f64, f64, f64) as Into<[f64; 4]>>::into(color_match.to_hsla())
							.into_iter()
							.map(round_to_two_decimal_places)
							.collect::<Vec<_>>();

					let opacity = (values.pop().unwrap() * 255.0).round() as u8;

					let colors_match =
						expected
							.iter()
							.zip(values.iter())
							.all(|(&hsl_val, &hsla_val)| {
								let tolerance = if hsl_val < 1.0 && hsla_val < 1.0 {
									0.02
								} else {
									1.0
								};
								(hsl_val - hsla_val).abs() < tolerance
							});

					(colors_match, opacity, "hsl")
				} else {
					let values = color_match.to_rgba8();
					let colors_match = color.rgb.r == values[0]
						&& color.rgb.g == values[1]
						&& color.rgb.b == values[2];
					(colors_match, values[3], "rgb")
				};

				if res.0 {
					let opacity = res.1;

					let filters = if opacity == 255 {
						format!(" | css_{}", res.2)
					} else {
						format!(
							" | mod(opacity={:.2}) | css_{}a",
							f32::from(opacity) / 255_f32,
							res.2
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
			"could not replace non-Catppuccin color '{}'{}",
			text.yellow(),
			match input_path {
				Some(ref p) => format!(
					" at {}:{}",
					p.to_string_lossy(),
					&get_location_in_text(&text, &contents),
				),
				None => String::new(),
			}
		);
	}

	contents
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

fn round_to_two_decimal_places(value: f64) -> f64 {
	(value * 100.0).round() / 100.0
}

fn hsl_to_vec(hsl: &Hsl) -> Vec<f64> {
	vec![hsl.h, hsl.s, hsl.l]
}
