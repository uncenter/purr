use color_eyre::Result;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use repositories::{
	RepositoriesOrganizationRepositories, RepositoriesOrganizationRepositoriesNodes,
};
use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/schema.graphql",
	query_path = "src/repositories.graphql",
	response_derives = "Debug"
)]
struct Repositories;

pub fn repositories(
	client: Client,
	cursor: Option<std::string::String>,
) -> RepositoriesOrganizationRepositories {
	let variables = repositories::Variables { cursor };

	let response_body =
		post_graphql::<Repositories, _>(&client, "https://api.github.com/graphql", variables)
			.unwrap();

	let response_data: repositories::ResponseData =
		response_body.data.expect("missing response data");

	return response_data
		.organization
		.expect("missing organization")
		.repositories;
}

pub fn paginate_repositories(
	token: String,
) -> Result<Vec<Option<RepositoriesOrganizationRepositoriesNodes>>, Error> {
	let client = Client::builder()
		.user_agent("graphql-rust/0.10.0")
		.default_headers(
			std::iter::once((
				reqwest::header::AUTHORIZATION,
				reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
			))
			.collect(),
		)
		.build()?;

	let mut cursor = None;
	let mut repos: Vec<Option<RepositoriesOrganizationRepositoriesNodes>> = vec![];

	loop {
		let data = repositories(client.clone(), cursor);

		repos.extend(data.nodes.expect("repositories nodes is null"));

		if !data.page_info.has_next_page {
			break;
		}
		cursor = data.page_info.end_cursor;
	}

	return Ok(repos);
}

pub fn rest(path: &str, token: Option<String>) -> Result<reqwest::blocking::Response> {
	let client = reqwest::blocking::Client::new();
	let request = client
		.get(format!("https://api.github.com/{}", path))
		.header(reqwest::header::USER_AGENT, "catppuccin-purr");
	if let Some(token) = token {
		Ok(request
			.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
			.send()?)
	} else {
		Ok(request.send()?)
	}
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
