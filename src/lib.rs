// 将业务逻辑放到lib.rs
use std::error::Error;
// collect
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        println!("Search for {} in file {}", query, filename);
        // 这里会获取所有权
        Ok(Config { query, filename })
    }
}
