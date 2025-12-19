use clap::Parser;
use std::{error, result};

mod cli;
mod database;
mod error_handling;
mod npm;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let connection = database::registry::DatabaseManager::init()?;
    let cli = cli::lib::Args::parse();
    match cli.command {
        cli::lib::Commands::Ls {} => {
            let registry_list = connection.find_all()?;
            for registry in registry_list {
                let prefix = if registry.is_current { "*" } else { " " };
                println!("{} {:<15} {}", prefix, registry.name, registry.url);
            }
        }
        cli::lib::Commands::Use { name } => {
            let url = connection.find_url_by_name(&name)?;
            let success_msg = npm::config::set_registry(&url, &name)?;
            println!("{}", success_msg);
        }
    }
    Ok(())
}
