use std::{env, error, result};
mod database;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let connection = database::registry::DatabaseManager::init()?;
    // 接受命令行参数
    let args: Vec<String> = env::args().collect();
    // 判断参数是否为ls
    if args.len() > 1 && args[1] == "ls" {
        let packages = connection.find_all()?;
        for (name, url) in packages {
            println!("* {:<15} {}", name, url);
        }
    }
    // connection.create_registry("taobao", "https://registry.npmmirror.com/")?;
    Ok(())
}
