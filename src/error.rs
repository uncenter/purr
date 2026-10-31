use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;
use url::Url;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
	#[error("directory '{}' already exists", .0.display())]
	#[diagnostic(
		code(purr::init::directory_exists),
		help("choose a different port name, or remove the existing directory")
	)]
	DirectoryExists(PathBuf),

	#[error("not in userstyles repository")]
	#[diagnostic(
		code(purr::init::not_in_userstyles_repository),
		help("run this from the root of a clone of https://github.com/catppuccin/userstyles")
	)]
	NotInUserstylesRepository,

	#[error("userstyle '{0}' already exists")]
	#[diagnostic(
		code(purr::init::userstyle_exists),
		help("see styles/{0}/catppuccin.user.less, or choose a different name")
	)]
	UserstyleExists(String),

	#[error("no port with the name '{0}'")]
	#[diagnostic(
		code(purr::query::port_not_found),
		help(
			"`--for` expects a port identifier (the key in ports.yml or userstyles.yml), e.g. `nvim`"
		)
	)]
	PortNotFound(String),

	#[error(transparent)]
	#[diagnostic(transparent)]
	OutdatedModel(#[from] OutdatedModel),

	#[error("failed to serialize results")]
	Serialize(#[from] serde_json::Error),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Http(#[from] reqwest::Error),

	#[error(transparent)]
	InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

	#[error(transparent)]
	Url(#[from] url::ParseError),

	#[error(transparent)]
	Utf8(#[from] std::string::FromUtf8Error),

	#[error(transparent)]
	Cache(#[from] cachecow::Error),

	#[error(transparent)]
	HomeDir(#[from] user_dirs::HomeDirError),
}

/// Upstream data that doesn't match struct models is a bug, and not a user-facing issue.
/// Ask users to report it instead, with a prefilled GitHub issue URL.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse {url}")]
#[diagnostic(
	code(purr::bug::outdated_model),
	url("{}", self.issue),
	help("this is a bug in purr, as its models are out of date with the upstream data; please report it using the link above")
)]
pub struct OutdatedModel {
	url: String,
	#[source]
	source: serde_yaml::Error,
	issue: String,
}

impl OutdatedModel {
	#[must_use]
	pub fn new(url: &str, source: serde_yaml::Error) -> Self {
		let title = format!("Failed to parse {}", url.rsplit('/').next().unwrap_or(url));
		let body = format!(
			"purr v{} could not parse <{url}>:\n\n```\n{source}\n```",
			env!("CARGO_PKG_VERSION"),
		);
		let issue = Url::parse_with_params(
			concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"),
			[("title", title.as_str()), ("body", body.as_str())],
		)
		.expect("issue URL should be valid");

		Self {
			url: url.to_string(),
			source,
			issue: issue.into(),
		}
	}
}
