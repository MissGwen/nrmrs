use std::{env, error, result};
mod database;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let connection = database::init::DatabaseManager::new()?;
    // 接受命令行参数
    let args: Vec<String> = env::args().collect();
    // 判断参数是否为ls
    if args.len() > 1 && args[1] == "ls" {
        // 查询所有包
        let packages = connection.find_all()?;
        // 打印所有包
        for (name, url) in packages {
            println!("{}: {}", name, url);
        }
    }
    // connection.create_registry("taobao", "https://registry.npmmirror.com/")?;
    Ok(())
}
