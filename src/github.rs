use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use repositories::{
	RepositoriesOrganizationRepositories, RepositoriesOrganizationRepositoriesNodes,
};

use crate::cache::{Cache, CacheError};

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/schema.graphql",
	query_path = "src/repositories.graphql",
	response_derives = "Debug,Serialize,Clone"
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
	cache: &mut Cache,
	token: &str,
) -> Result<Vec<Option<RepositoriesOrganizationRepositoriesNodes>>, GitHubError> {
	cache.get_or("all-repositories", || {
		let client = Client::builder()
			.user_agent("catppuccin-purr")
			.default_headers(
				std::iter::once((
					reqwest::header::AUTHORIZATION,
					reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))?,
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
	})
}

pub fn rest(path: &str, token: Option<String>) -> Result<reqwest::blocking::Response, GitHubError> {
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

pub fn fetch_whiskers_status(
	cache: &mut Cache,
	repository: &str,
	token: String,
) -> Result<String, GitHubError> {
	let cache_key = format!("whiskers-{repository}");
	if let Some(cached) = cache.get::<String>(&cache_key) {
		return Ok(cached.to_string());
	}

	let props = rest(
		&format!("repos/catppuccin/{repository}/properties/values"),
		Some(token),
	)?
	.json::<Vec<CustomProperty>>()?;

	let property = props
		.iter()
		.find(|prop| prop.property_name == "whiskers")
		.expect("whiskers custom property should exist on all repositories");

	cache.save(&cache_key, property.value.clone()).into()
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum GitHubError {
	#[error("Failed to retrieve data from filesystem cache.")]
	Cache(#[from] CacheError),
	#[error("Request to GitHub API failed.")]
	RequestFailed(#[from] reqwest::Error),
}
