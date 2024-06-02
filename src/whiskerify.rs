use std::{fs, path::PathBuf};

use color_eyre::eyre::Result;

pub fn convert(path: PathBuf, dry_run: bool) -> Result<()> {
	let mut contents = fs::read_to_string(&path)?;

	for flavor in catppuccin::PALETTE.all_flavors() {
		for color in &flavor.colors {
			contents = contents.replace(
				&color.hex.to_string(),
				&format!("{}{}{}", "{{ ", color.identifier(), ".hex }}"),
			);
		}
	}

	if dry_run {
		println!("{}", contents);
	} else {
		fs::write(path, contents)?;
	}

	Ok(())
}
