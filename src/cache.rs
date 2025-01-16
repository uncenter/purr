use color_eyre::eyre::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
	collections::HashMap,
	fs,
	io::{self, Write},
	path::PathBuf,
	time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
	// TODO: Simplify timestamp; storing full SystemTime struct/object is inefficient compared to just the (nano)?seconds as integer.
	timestamp: SystemTime,
	data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
	path: PathBuf,
	entries: HashMap<String, Entry>,
	refresh: bool,
	entry_duration_seconds: u64,
}

impl Cache {
	/// Initializes a new cache, with timestamped data entries saved to the specified path as JSON.
	pub fn new(path: PathBuf, refresh: bool, entry_duration_seconds: u64) -> Self {
		let entries = match fs::read_to_string(&path) {
			Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
			Err(_) => {
				fs::create_dir_all(path.parent().unwrap()).unwrap();
				HashMap::new()
			}
		};

		Cache {
			path,
			entries,
			refresh,
			entry_duration_seconds,
		}
	}

	/// Retrieve a keyed value from the cache store, returning `None` if hard refresh is enabled in the cache settings or if the entry's timestamp is older than the specified maximum duration.
	pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
		if self.refresh {
			return None;
		}
		self.entries.get(key).and_then(|entry| {
			let diff = SystemTime::now()
				.duration_since(entry.timestamp)
				.unwrap()
				.as_secs();
			if diff < self.entry_duration_seconds {
				serde_json::from_value(entry.data.clone()).ok()
			} else {
				None
			}
		})
	}

	/// Wrapper of the [`Cache::get`] function, accepting a closure for retrieving and then saving the value if the value is not present already or invalid.
	pub fn get_or<T, F>(&mut self, key: &str, fetch: F) -> Result<T>
	where
		T: Serialize + DeserializeOwned + Clone,
		F: FnOnce() -> Result<T>,
	{
		if let Some(data) = self.get::<T>(key) {
			return Ok(data);
		}
		let value = fetch()?;
		self.save(key, value)
	}

	/// Save a value under a key to the cache store, returning that same value.
	pub fn save<T: Serialize>(&mut self, key: &str, value: T) -> Result<T> {
		self.entries.insert(
			key.to_string(),
			Entry {
				timestamp: SystemTime::now(),
				data: serde_json::to_value(&value)?,
			},
		);
		self.save_to_file()?;
		Ok(value)
	}

	/// Save the cache to the store path (specified at cache initialization).
	fn save_to_file(&self) -> io::Result<()> {
		let mut file = fs::File::create(&self.path)?;
		file.write_all(serde_json::to_string(&self.entries)?.as_bytes())
	}
}
