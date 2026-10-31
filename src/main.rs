use cachecow::Cache;
use std::time::Duration;

use clap::Parser;

use catppuccin_purr::{cli, cmd, error::Result};

fn main() -> miette::Result<()> {
	miette::set_panic_hook();
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Warn)
		.init();

	Ok(run(cli::Cli::parse())?)
}

fn run(args: cli::Cli) -> Result<()> {
	let mut cache = Cache::new(
		user_dirs::cache_dir()?.join("purr/store.json"),
		Duration::from_secs(if args.refresh { 0 } else { 24 * 60 * 60 }),
		cachecow::FlushPolicy::Auto,
	)?;

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
