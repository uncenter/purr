use clap::Parser;
use color_eyre::eyre::Result;
use etcetera::{choose_base_strategy, BaseStrategy};

use catppuccin_purr::{cache, cli, cmd};

static ONE_DAY_IN_SECONDS: u64 = 24 * 60 * 60;

fn main() -> Result<()> {
	color_eyre::install()?;
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Warn)
		.init();

	let args = cli::Cli::parse();

	let mut cache = cache::Cache::new(
		choose_base_strategy()
			.unwrap()
			.cache_dir()
			.join("purr/store.json"),
		args.refresh,
		ONE_DAY_IN_SECONDS,
	);

	match args.command {
		cli::Commands::Query {
			command,
			r#for,
			count,
			get,
			_no_userstyles,
			userstyles,
			only_userstyles,
		} => cmd::query::query(
			&mut cache,
			command,
			r#for,
			count,
			get,
			userstyles,
			only_userstyles,
		)?,
		cli::Commands::Init { command } => match command {
			cli::Template::Port {
				name,
				url,
				whiskers,
			} => cmd::init::port(name, url, whiskers)?,
			cli::Template::Userstyle {
				name,
				categories,
				icon,
				color,
				url,
				clear_comments,
			} => cmd::init::userstyle(
				&mut cache,
				name,
				categories,
				icon,
				color,
				url,
				clear_comments,
			)?,
		},
		cli::Commands::Whiskerify { input, output } => cmd::whiskerify::handle(input, output)?,
	}

	Ok(())
}
