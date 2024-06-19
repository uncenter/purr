use catppuccin_purr::{
	cache::Cache,
	cli::{Cli, Commands, Userstyles},
	ports, userstyles, whiskerify,
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
		} => ports::query(cache, command, r#for, count, get)?,
		Commands::Init { name, url } => ports::init(name, url)?,
		Commands::Userstyles { command } => match command {
			Userstyles::Query {
				command,
				r#for,
				count,
				get,
			} => userstyles::query(cache, command, r#for, count, get)?,
			Userstyles::Init {
				name,
				categories,
				icon,
				color,
				url,
			} => userstyles::init(cache, name, categories, icon, color, url)?,
		},
		Commands::Whiskerify { input, output } => whiskerify::handle(input, output)?,
	}

	Ok(())
}
