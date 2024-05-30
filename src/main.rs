use clap::Parser;
use color_eyre::eyre::Result;
use purr::{
	cli::{Cli, Commands, Userstyles},
	ports, userstyles,
};

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = Cli::parse();

	match args.command {
		Commands::Query {
			command,
			count,
			output,
		} => ports::query(command, count, output)?,
		Commands::Init { name, url } => ports::init(name, url)?,
		Commands::Userstyles { command } => match command {
			Userstyles::Query {
				command,
				count,
				output,
			} => userstyles::query(command, count, output)?,
			Userstyles::Init {
				name,
				categories,
				icon,
				color,
				url,
			} => userstyles::init(name, categories, icon, color, url)?,
		},
	}

	Ok(())
}
