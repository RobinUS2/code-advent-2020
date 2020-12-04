use std::io::{stdout, Write};
use std::env;
use std::collections::HashMap;

extern crate curl;

fn main() {
	let body = get_data(4);

	let mut numValid = 0;
	let mut keyValues = HashMap::new();
	const REQUIRED_KEYS: [& str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let mut rows = Vec::new();
	for line in body.lines() {
		rows.push(line);
	}
	rows.push(""); // final newline
	for line in rows {
		let tokens = line.split(" ");
		for token in tokens {
			println!("{:?}", token);
			if token == "" {
				// newline
				println!("{}", keyValues.len());
				let mut allFound = true;
				for requiredKey in &REQUIRED_KEYS {
					let val = keyValues.get(&requiredKey.to_string());
					if val == None {
						allFound = false;
						break;
					}
					println!("{:?} {:?}", requiredKey, val);
				}
				for (k, v) in &keyValues {
			        println!("{:?}: {:?}", k, v);
			    }
				if !allFound {
					println!("invalid");
				} else {
					println!("valid");
					numValid += 1;
				}
				println!("--\n");
				keyValues.clear();
			} else {
				// add
				let keyValueSplit: Vec<&str> = token.split(":").collect();
				let key: String = keyValueSplit[0].chars().collect();
				let value: String = keyValueSplit[1].chars().collect();
				keyValues.insert(key.to_string(), value.to_string());
			}
		}
	}

	println!("numValid {:?}", numValid);
}

pub fn get_data(day: i32) -> String {
    let mut data = Vec::new();
    let mut easy = curl::easy::Easy::new();
    let session = env::var("ADVENT_SESSION").unwrap_or("".to_string());
    easy.url(&format!("https://adventofcode.com/2020/day/{}/input", day)).unwrap();

    // cookie
    let mut sess: String = "session=".to_owned();
    sess.push_str(&session);
    easy.cookie(&sess).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    return body;
}
