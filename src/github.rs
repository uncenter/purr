use color_eyre::{eyre::bail, Result};

use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};

use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use repositories::{
	RepositoriesOrganizationRepositories, RepositoriesOrganizationRepositoriesNodes,
};

use crate::cache::Cache;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/schema.graphql",
	query_path = "src/repositories.graphql",
	response_derives = "Debug"
)]
struct Repositories;

#[must_use]
pub fn fetch_repositories(
	client: &Client,
	cursor: Option<std::string::String>,
) -> RepositoriesOrganizationRepositories {
	let variables = repositories::Variables { cursor };

	let reponse =
		post_graphql::<Repositories, _>(client, "https://api.github.com/graphql", variables)
			.unwrap();

	let data: repositories::ResponseData = reponse.data.expect("missing response data");

	data.organization
		.expect("missing organization")
		.repositories
}

pub fn fetch_all_repositories(
	token: &str,
) -> Result<Vec<Option<RepositoriesOrganizationRepositoriesNodes>>, Error> {
	let client = Client::builder()
		.user_agent("graphql-rust/0.10.0")
		.default_headers(
			std::iter::once((
				reqwest::header::AUTHORIZATION,
				reqwest::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
			))
			.collect(),
		)
		.build()?;

	let mut cursor = None;
	let mut repositories: Vec<Option<RepositoriesOrganizationRepositoriesNodes>> = vec![];

	loop {
		let data = fetch_repositories(&client, cursor);

		repositories.extend(data.nodes.expect("repositories nodes is null"));

		if !data.page_info.has_next_page {
			break;
		}
		cursor = data.page_info.end_cursor;
	}

	Ok(repositories)
}

pub fn rest(path: &str, token: Option<String>) -> Result<reqwest::blocking::Response> {
	let client = Client::new();
	let request = client
		.get(format!("https://api.github.com/{path}"))
		.header(reqwest::header::USER_AGENT, "catppuccin-purr");
	Ok(if let Some(token) = token {
		request
			.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
			.send()?
	} else {
		request.send()?
	}
	.error_for_status()?)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryResponse {
	pub stargazers_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomProperty {
	pub property_name: String,
	pub value: String,
}

pub fn fetch_whiskers_custom_property(
	mut cache: Cache,
	repository: &str,
	token: String,
) -> Result<String> {
	let cache_id = &format!("whiskers-{repository}");
	if let Some(cached) = cache.get(&cache_id) {
		println!("using cached status");
		return Ok(cached.to_string());
	}

	let props = rest(
		&format!("repos/catppuccin/{repository}/properties/values"),
		Some(token),
	)?
	.json::<Vec<CustomProperty>>()?;

	for prop in props {
		if prop.property_name == "whiskers" {
			return cache.save(&cache_id, prop.value);
		}
	}

	bail!("whiskers custom property should exist on all repositories")
}
