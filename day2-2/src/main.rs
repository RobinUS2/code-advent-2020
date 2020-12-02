use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
    	let mut numValid = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
        	// format 4-10 h: wmfqghskgjtt
            if let Ok(lineStr) = line {
            	let lineTokens = lineStr.split(" ");
            	let lineTokenStr: Vec<&str> = lineTokens.collect();
            	let minMaxTokens: Vec<&str> = lineTokenStr[0].split("-").collect();
            	let posA = minMaxTokens[0].parse::<i64>().unwrap_or(0) as usize;
            	let posB = minMaxTokens[1].parse::<i64>().unwrap_or(0) as usize;
            	if posA == 0 || posB == 0 {
            		continue
            	}
            	let charStrDirty = lineTokenStr[1];
            	let charStr : String = charStrDirty.chars().into_iter().take(1).collect();
            	let charStrChar = charStr.as_bytes()[0] as char;

            	let pwd = lineTokenStr[2].to_string();
            	let pwdLen = pwd.chars().count();
            	let pwdBytes = pwd.as_bytes();
            	let mut foundA = false;
            	let mut foundB = false;
            	if posA < pwdLen {
	            	let pwdA = pwdBytes[posA-1];
	            	if pwdA == charStrChar {
	            		foundA = true;
	            	}
	            }
	            if posB < pwdLen {
	            	let pwdB = pwdBytes[posB-1];
	            	if pwdB == charStrChar {
	            		foundB = true;
	            	}
	            }

            	println!("{} {} {} {} {} {}", posA, posB, charStr, pwd, pwdA, pwdB);
            }
        }
        println!("numValid {}", numValid)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}