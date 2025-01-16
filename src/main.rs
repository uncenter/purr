use catppuccin_purr::{
	cache::Cache,
	cli::{Cli, Commands, Template},
	init, query, whiskerify,
};
use clap::Parser;
use color_eyre::eyre::Result;
use etcetera::{choose_base_strategy, BaseStrategy};

static ONE_DAY_IN_SECONDS: u64 = 24 * 60 * 60;

fn main() -> Result<()> {
	color_eyre::install()?;
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Warn)
		.init();

	let args: Cli = Cli::parse();

	let mut cache = Cache::new(
		choose_base_strategy()
			.unwrap()
			.cache_dir()
			.join("purr/store.json"),
		args.refresh,
		ONE_DAY_IN_SECONDS,
	);

	match args.command {
		Commands::Query {
			command,
			r#for,
			count,
			get,
			_no_userstyles,
			userstyles,
			only_userstyles,
		} => query::query(
			&mut cache,
			command,
			r#for,
			count,
			get,
			userstyles,
			only_userstyles,
		)?,
		Commands::Init { command } => match command {
			Template::Port {
				name,
				url,
				whiskers,
			} => init::port(name, url, whiskers)?,
			Template::Userstyle {
				name,
				categories,
				icon,
				color,
				url,
				clear_comments,
			} => init::userstyle(
				&mut cache,
				name,
				categories,
				icon,
				color,
				url,
				clear_comments,
			)?,
		},
		Commands::Whiskerify { input, output } => whiskerify::handle(input, output)?,
	}

	Ok(())
}
