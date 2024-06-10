use catppuccin_purr::{
	cli::{Cli, Commands, Userstyles},
	ports, userstyles, whiskerify,
};
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
	color_eyre::install()?;
	pretty_env_logger::formatted_builder()
		.filter_level(log::LevelFilter::Warn)
		.init();

	let args = Cli::parse();

	match args.command {
		Commands::Query {
			command,
			r#for,
			count,
			get,
		} => ports::query(command, r#for, count, get)?,
		Commands::Init { name, url } => ports::init(name, url)?,
		Commands::Userstyles { command } => match command {
			Userstyles::Query {
				command,
				r#for,
				count,
				get,
			} => userstyles::query(command, r#for, count, get)?,
			Userstyles::Init {
				name,
				categories,
				icon,
				color,
				url,
			} => userstyles::init(name, categories, icon, color, url)?,
		},
		Commands::Whiskerify { input, output } => whiskerify::handle(input, output)?,
	}

	Ok(())
}
