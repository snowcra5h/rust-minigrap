/*
 * cargo run -- searchstring example-filename.txt
 */

mod globals;
use globals::{SEARCH_STR, QUERY_STR};

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query:     &String = &args[1];
    let file_path: &String = &args[2];

    println!("\n{SEARCH_STR:20} : {:20}", query);
    println!("{QUERY_STR:20} : {:20}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}

