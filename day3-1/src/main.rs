use std::io::{stdout, Write};
use std::env;

extern crate curl;

fn main() {
	let body = from_assignment(3);

	let tree = '#';
	let grass = '.';
	let slopeX = 3;
	let slopeY = 1;

	// to multi dimensional grid
	let lines = body.split("\n");
	let numLines = body.split("\n").count();
	// println!("{}", numLines);
	let mut dimension = 0;
	let mut desiredWidth = 0;
	let mut grid = vec![vec!['#'; 0]; 0];
	let mut y = 0;
    for line in lines {
    	let chars = line.chars();
    	if dimension == 0 {
	    	dimension = line.chars().count();
	    	desiredWidth = numLines * slopeX;
	    	grid = vec![vec!['#'; desiredWidth]; numLines];
	    	println!("{}x{}", desiredWidth, numLines)
	    }

	    let mut x = 0;
	    'outer: loop {
		    for (j, s) in line.chars().into_iter().enumerate() {
		    	// println!("x {} y {} = {} ({})", x, y, s, j);
		    	if x >= desiredWidth {
		    		break 'outer;
				}
				// print!("{}", s);
		    	grid[y][x] = s;
		    	x += 1;

		    	// stop
		    	// println!("a");
		    }
		    // println!("b {} {}", x, desiredWidth);
		    if x == 0 {
		    	// empty line
		    	break;
		    }
		}
		// print!("\n");
        y += 1;
    }

    // start
    println!("start");
    let mut posX = 0;
    let mut posY = 0;
    let mut numTrees = 0;
    loop {
    	let val = grid[posY][posX];
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

    println!("numTrees {}", numTrees)

}

pub fn from_assignment(day: i32) -> String {
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
