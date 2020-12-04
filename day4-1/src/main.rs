use std::io::{stdout, Write};
use std::env;
use std::collections::HashMap;
use regex::Regex;

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
				let mut invalid = false;
				for requiredKey in &REQUIRED_KEYS {
					let val: Option<&String> = keyValues.get(&requiredKey.to_string());
					if val == None {
						allFound = false;
						break;
					}

					// validators
					match val {
						Some(realVale) => {
							match requiredKey {
					            &"byr" => {
					            	let v = realVale.parse::<i64>().unwrap_or(-1);
					            	if v < 1920 || v > 2002 {
					            		invalid = true;
					            	}
					            },
					            &"iyr" => {
					            	let v = realVale.parse::<i64>().unwrap_or(-1);
					            	if v < 2010 || v > 2020 {
					            		invalid = true;
					            	}
					            },
					            &"eyr" => {
					            	let v = realVale.parse::<i64>().unwrap_or(-1);
					            	if v < 2020 || v > 2030 {
					            		invalid = true;
					            	}
					            },
					            &"hgt" => {
					            	let v = parse_int(realVale);
			            			let isCm = realVale.ends_with("cm");
			            			let isIn = realVale.ends_with("in");
			            			if !isCm && !isIn {
			            				invalid = true;
			            			} else if isCm && (v < 150  || v > 193) {
			            				invalid = true;
			            			} else if isIn && (v < 59  || v > 76) {
			            				invalid = true;
			            			}
			            			// println!("hgt {:?} isCm {} isIn {}", v, isCm, isIn);
					            },
					            &"hcl" => {
					            	let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
					            	if !re.is_match(realVale) {
					            		invalid = true;
					            	}
					            },
					            &"ecl" => {
					            	let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
					            	if !re.is_match(realVale) {
					            		invalid = true;
					            	}
					            },
					            &"pid" => {
					            	let re = Regex::new(r"^[0-9]{9}$").unwrap();
					            	if !re.is_match(realVale) {
					            		invalid = true;
					            	}
					            },
					            _ => (),
					        }
					    },
					    None => (),
				    }

					println!("{:?} {:?}", requiredKey, val);
				}
				for (k, v) in &keyValues {
			        println!("{:?}: {:?}", k, v);
			    }
				if !allFound {
					println!("incomplete");
				} else if invalid {
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

fn parse_int(input: &str) -> i64 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let capture = re.captures(input);
    let mut v: i64 = 0;
    match capture {
    	Some(caps) => {
	    	let firstCap = caps.get(1).map_or("-2", |m| m.as_str());
	    	v = firstCap.parse::<i64>().unwrap_or(-1);
    	},
    	None => (),
    }
    return v;
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
