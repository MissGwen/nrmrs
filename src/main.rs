use clap::{Parser, Subcommand};
use std::{error, result};
mod database;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all the registries
    Ls {},
    // /// 删除指定ID的TODO任务
    // Delete {
    //     /// 任务ID（必选）
    //     #[arg(short, long)]
    //     id: u32,
    // },
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let connection = database::registry::DatabaseManager::init()?;
    // let args = Args::parse();
    let cli = Cli::parse();
    match cli.command {
        Commands::Ls {} => {
            let packages = connection.find_all()?;
            for (name, url) in packages {
                println!("* {:<15} {}", name, url);
            }
        } // Commands::Delete { id } => {
          //     println!("删除任务ID：{}", id);
          // }
    }
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }

    Ok(())

    // // 接受命令行参数
    // let args: Vec<String> = env::args().collect();
    // // 判断参数是否为ls
    // if args.len() > 1 && args[1] == "ls" {
    //     let packages = connection.find_all()?;
    //     for (name, url) in packages {
    //         println!("* {:<15} {}", name, url);
    //     }
    // } else {
    //     println!("npm registry manager! you can use ls to list all registries!")
    // }
    // Ok(())
}
