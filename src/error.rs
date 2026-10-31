use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

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

	#[error("failed to serialize results")]
	Serialize(#[from] serde_json::Error),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Yaml(#[from] serde_yaml::Error),

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
