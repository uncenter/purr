//! Checks that `models/` still match the formats of the latest upstream data sources.
//! Ignored by default as tests rely on the network.
//! Run with `cargo test --test upstream_models -- --ignored`.

use std::collections::BTreeSet;

use catppuccin_purr::{
	models::{self, categories::CATEGORIES},
	utils::fetch_text,
};
use serde::de::DeserializeOwned;

const PORTS_YML: &str = "https://github.com/catppuccin/catppuccin/raw/main/resources/ports.yml";
const USERSTYLES_YML: &str =
	"https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml";
const CATEGORIES_YML: &str =
	"https://github.com/catppuccin/catppuccin/raw/main/resources/categories.yml";

/// Attempts to deserialize the content from `url`, failing on missing/mistype/unknown fields relative to the model struct.
/// Paths in `unused` are upstream fields deliberately not modelled.
fn assert_parses_strictly<T: DeserializeOwned>(url: &str, unused: &[&str]) {
	let raw = fetch_text(url).unwrap();
	let mut unknown = Vec::new();
	let result: Result<T, _> =
		serde_ignored::deserialize(serde_yaml::Deserializer::from_str(&raw), |path| {
			let path = path.to_string();
			if !unused.contains(&path.as_str()) {
				unknown.push(path);
			}
		});

	if let Err(e) = result {
		panic!("{url} does not match the model: {e}");
	}
	assert!(
		unknown.is_empty(),
		"{url} has fields missing from the model:\n  {}",
		unknown.join("\n  ")
	);
}

#[test]
#[ignore = "requires network"]
fn ports() {
	assert_parses_strictly::<models::ports::Root>(PORTS_YML, &["archived", "showcases"]);
}

#[test]
#[ignore = "requires network"]
fn userstyles() {
	assert_parses_strictly::<models::userstyles::Root>(USERSTYLES_YML, &[]);
}

#[test]
#[ignore = "requires network"]
fn categories() {
	#[derive(serde::Deserialize)]
	struct Category {
		key: String,
	}

	let raw = fetch_text(CATEGORIES_YML).unwrap();
	let upstream = serde_yaml::from_str::<Vec<Category>>(&raw)
		.unwrap()
		.into_iter()
		.map(|c| c.key)
		.collect::<BTreeSet<_>>();
	let ours = CATEGORIES
		.iter()
		.map(|&c| c.to_string())
		.collect::<BTreeSet<_>>();

	let missing = upstream.difference(&ours).collect::<Vec<_>>();
	let extra = ours.difference(&upstream).collect::<Vec<_>>();
	assert!(
		missing.is_empty() && extra.is_empty(),
		"models::categories::CATEGORIES is out of date with {CATEGORIES_YML}\n  missing: {missing:?}\n  extra: {extra:?}"
	);
}
