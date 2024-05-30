use clap::{arg, Args, Parser, Subcommand, ValueEnum};
use url::Url;

#[derive(Parser)]
#[command(version, arg_required_else_help(true))]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Query {
		#[command(subcommand)]
		command: Option<Query>,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
	Init {
		#[arg(long)]
		name: Option<String>,

		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
	Userstyles {
		#[command(subcommand)]
		command: Userstyles,
	},
}

#[derive(Subcommand)]
pub enum Userstyles {
	Query {
		#[command(subcommand)]
		command: Option<UserstylesQuery>,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
	Init {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category")]
		categories: Option<Vec<String>>,

		#[arg(long)]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, value_parser = valid_url)]
		url: Option<String>,
	},
}

#[derive(Subcommand)]
pub enum Query {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
	Has {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category")]
		categories: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		upstreamed: Option<bool>,

		#[arg(long)]
		platform: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		alias: Option<String>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true", value_parser = valid_url)]
		url: Option<String>,

		#[command(flatten)]
		result: Option<CountOrList>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
}

#[derive(Subcommand)]
pub enum UserstylesQuery {
	Maintained {
		#[arg(long, name = "NAME")]
		by: Option<String>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long)]
		count: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
	Has {
		#[arg(long)]
		name: Option<String>,

		#[arg(long = "category")]
		categories: Option<Vec<String>>,

		#[arg(long, num_args = 0..=1, default_missing_value = "true")]
		icon: Option<String>,

		#[arg(long)]
		color: Option<String>,

		#[arg(long, value_parser = valid_url)]
		app_link: Option<String>,

		#[command(flatten)]
		result: Option<CountOrList>,

		#[arg(short, long)]
		not: bool,

		#[arg(short, long, default_value = "json", name = "FORMAT")]
		output: OutputFormat,
	},
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct CountOrList {
	#[arg(short, long)]
	pub count: bool,

	#[arg(short, long)]
	pub list: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
	Json,
	Plain,
}

fn valid_url(u: &str) -> Result<String, String> {
	if Url::parse(u).is_ok() {
		Ok(String::from(u))
	} else {
		Err(format!("{} is not a valid URL", u))
	}
}
