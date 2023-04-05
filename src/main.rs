/*
 * cargo run -- searchstring example-filename.txt
 */

mod globals;
use globals::{SEARCH_STR, QUERY_STR};

use std::{env, fs, process};

fn main() {
    let args: Vec<String>  = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    println!("\n{SEARCH_STR:20} : {:20}", config.query);
    println!("{QUERY_STR:20} : {:20}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

