use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //将迭代器作为参数
    let config = Config::new(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}",err);
            process::exit(1);
            });

    println!("This is query: {}!😈🚀",config.query);
    println!("This is filename: {}!👾",config.filename);

    if let Err(e) =minigrep::run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}