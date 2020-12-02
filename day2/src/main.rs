use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
        	// format 4-10 h: wmfqghskgjtt
            if let Ok(lineStr) = line {
            	let lineTokens = lineStr.split(" ");
            	let lineTokenStr: Vec<&str> = lineTokens.collect();
            	let minMaxTokens: Vec<&str> = lineTokenStr[0].split("-").collect();
            	let min = minMaxTokens[0].parse::<i64>().unwrap_or(-1);
            	let max = minMaxTokens[1].parse::<i64>().unwrap_or(-1);
            	if min == -1 || max == -1 {
            		continue
            	}
            	let charStrDirty = lineTokenStr[1];
            	let charStr : String = charStrDirty.chars().into_iter().take(1).collect();
            	let pwd = lineTokenStr[2];
            	println!("{} {} {} {}", min, max, charStr, pwd);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}