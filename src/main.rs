use std::env;
use std::io;
use std::process;
use std::str::FromStr;

use grep_starter_rust::Pattern;
use map_macro::hash_set;


fn match_pattern(input_line: &str, pattern: &str) -> bool {

    let regexp = Pattern::from_str(pattern).unwrap();

    println!("pattern metadata : {:?}", regexp);
    let res = regexp.match_string(input_line);
    println!("the final result : {:?}" , res);
    if res.is_empty() {
       return false;
    }
    else if res.len() == 1 && res.contains("") {
        return true;
    }
    return false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
