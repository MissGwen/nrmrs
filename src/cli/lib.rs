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

    /// Add a new registry [use --name <registry_name> --url <registry_url>]
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        url: String,
    },

    /// Remove a registry [use --name <registry_name>]
    Delete {
        #[arg(short, long)]
        name: String,
    },
}
