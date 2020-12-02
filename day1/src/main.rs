use std::io::{stdout, Write};
use std::env;

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    let session = env::var("ADVENT_SESSION").unwrap_or("".to_string());
//    println!("{}", session);
    easy.url("https://adventofcode.com/2020/day/1/input").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    
    // cookie
    let mut sess: String = "session=".to_owned();
    sess.push_str(&session);
//    println!("{}", sess);
    easy.cookie(&sess).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
