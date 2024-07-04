use catppuccin_purr::{
	cache::Cache,
	cli::{Cli, Commands, Template},
	init, query, whiskerify,
};
use clap::Parser;
use color_eyre::eyre::Result;
use etcetera::{choose_base_strategy, BaseStrategy};

fn main() -> Result<()> {
	color_eyre::install()?;
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Warn)
		.init();

	let args: Cli = Cli::parse();

	let cache = Cache::new(
		choose_base_strategy()
			.unwrap()
			.cache_dir()
			.join("purr/store.json"),
		args.refresh,
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
			cache,
			command,
			r#for,
			count,
			get,
			userstyles,
			only_userstyles,
		)?,
		Commands::Init { command } => match command {
			Template::Port { name, url } => init::port(name, url)?,
			Template::Userstyle {
				name,
				categories,
				icon,
				color,
				url,
			} => init::userstyle(cache, name, categories, icon, color, url)?,
		},
		Commands::Whiskerify { input, output } => whiskerify::handle(input, output)?,
	}

	Ok(())
}
