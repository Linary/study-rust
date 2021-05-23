use std::env;
use std::process;

use study_rust::Config;

fn main() {
    // 获取到命令行参数
    // ["target/debug/study-rust", "1234", "abcd"]
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem paring arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = study_rust::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
