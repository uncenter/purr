use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
	path: PathBuf,
	data: HashMap<String, String>,
}

impl Cache {
	pub fn new(path: PathBuf) -> Self {
		let data = match fs::read_to_string(&path) {
			Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
			Err(_) => {
				fs::create_dir_all(path.parent().unwrap()).unwrap();
				HashMap::new()
			}
		};

		Cache { path, data }
	}

	pub fn get(&self, key: &str) -> Option<&String> {
		self.data.get(key)
	}

	pub fn save(&mut self, key: &str, value: String) -> Result<String> {
		self.data.insert(key.to_string(), value.clone());
		self.save_to_file()?;
		Ok(value)
	}

	fn save_to_file(&self) -> io::Result<()> {
		let mut file = fs::File::create(&self.path)?;
		file.write_all(serde_json::to_string(&self.data)?.as_bytes())
	}
}
