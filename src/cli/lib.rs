use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all the registries
    Ls {},

    /// Change current registry [use --name <registry_name>]
    Use {
        #[arg(short, long)]
        name: String,
    },
}
