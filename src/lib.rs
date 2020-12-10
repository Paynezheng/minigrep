use std::error::Error;
use std::fs;
//读取环境变量
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    //因为迭代器返回了数据的所有权，就无需复制内存和数据
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        //env::var检查环境变量
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
        /*
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        //env::var检查环境变量
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
        */
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(_query: &str,_contents: &'a str) -> Vec<&'a str> {
    // 迭代器 + 闭包更适合用来做过滤数据
    _contents.lines()
    .filter(|line| line.contains(_query))
    .collect()
    /*
    let mut results = Vec::new();
    for line in _contents.lines() {
        if line.contains(_query){
            results.push(line);
        }
    }
    results
    */
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
    /*
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
    */
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "\
Rust:
safe,fast,productive.
苟利国家生死以，
岂因祸福避趋之？";

        assert_eq!(
            vec!["safe,fast,productive."],
            search(query,contents)
            );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust是最好的语言！
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust是最好的语言！", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

