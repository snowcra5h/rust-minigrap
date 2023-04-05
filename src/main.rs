/*
 * cargo run -- searchstring example-filename.txt
 */

mod globals;
use globals::{SEARCH_STR, QUERY_STR};

use minigrep::Config;

use std::{env, process};

fn main() {
    let args: Vec<String>  = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    println!("\n{SEARCH_STR:20} : {:20}", config.query);
    println!("{QUERY_STR:20} : {:20}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


