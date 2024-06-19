use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	fs,
	io::{self, Write},
	path::PathBuf,
	time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
	timestamp: SystemTime,
	data: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
	path: PathBuf,
	entries: HashMap<String, Entry>,
}

impl Cache {
	pub fn new(path: PathBuf) -> Self {
		let entries = match fs::read_to_string(&path) {
			Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
			Err(_) => {
				fs::create_dir_all(path.parent().unwrap()).unwrap();
				HashMap::new()
			}
		};

		Cache { path, entries }
	}

	pub fn get(&self, key: &str) -> Option<&String> {
		match self.entries.get(key) {
			Some(entry) => {
				let diff = SystemTime::now()
					.duration_since(entry.timestamp)
					.unwrap()
					.as_secs();
				println!("diff: {diff}");
				if diff < (24 * 60 * 60) {
					println!("using cached");
					Some(&entry.data)
				} else {
					println!("invalidating cache");
					None
				}
			}
			None => None,
		}
	}

	pub fn save(&mut self, key: &str, value: String) -> Result<String> {
		self.entries.insert(
			key.to_string(),
			Entry {
				timestamp: SystemTime::now(),
				data: value.clone(),
			},
		);
		self.save_to_file()?;
		Ok(value)
	}

	fn save_to_file(&self) -> io::Result<()> {
		let mut file = fs::File::create(&self.path)?;
		file.write_all(serde_json::to_string(&self.entries)?.as_bytes())
	}
}
