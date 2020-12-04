use std::io::{stdout, Write};
use std::env;

extern crate curl;

fn main() {
	let body = get_data(3);

    let mut lines: Vec<Vec<char>> = vec!();
    for line in body.trim().lines() {
        let chars: Vec<char> = line.chars().collect();
        lines.push(chars);
    }


	let mut result = count(&lines, 1, 1);
	result *= count(&lines, 3, 1);
	result *= count(&lines, 5, 1);
	result *= count(&lines, 7, 1);
	result *= count(&lines, 1, 2);
	println!("{}", result)
}

fn count(lines: &Vec<Vec<char>>, slopeX: usize, slopeY: usize) -> i64 {

	let tree = '#';
	let grass = '.';

	let dimension = lines[0].len();
	let numLines = lines.len();

    // start
    println!("start");
    let mut posX = 0;
    let mut posY = 0;
    let mut numTrees = 0;
    loop {
    	let val = lines[posY][posX % dimension];
    	let isTree = val == tree;
    	let isGrass = val == grass;
    	println!("x{} y{} {} tree={} open={}", posX, posY, val, isTree, isGrass);

    	if isTree {
    		numTrees += 1;
    	}

    	// move
    	posX += slopeX;
    	posY += slopeY;

    	// done?
    	if posY >= numLines {
    		break;
    	}
    }

    println!("numTrees {}", numTrees);
    return numTrees;
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
