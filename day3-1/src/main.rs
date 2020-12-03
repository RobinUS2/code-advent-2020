use std::io::{stdout, Write};
use std::env;

use curl::easy::Easy;

fn main() {
	let body = from_assignment(3);
}

pub fn from_assignment(day: i32) -> String {
    let mut handle = Easy::new();
    let session = env::var("ADVENT_SESSION").unwrap_or("".to_string());
    // println!("{}", session);
    let url = &format!("https://adventofcode.com/2020/day/{}/input", day);
    handle.url(url).unwrap();
    handle.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    
    // cookie
    let mut sess: String = "session=".to_owned();
    sess.push_str(&session);
    // println!("{}", sess);
    handle.cookie(&sess).unwrap();
    handle.perform().unwrap();

    // resp
    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    println!("{}", body);
    return body;
}
