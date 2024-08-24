use std::env;
use std::io;
use std::process;

use grep_starter_rust::*;


fn match_pattern(input_line: &str, pattern: &str) -> bool {

    let mut start_anchor = false;
    let mut end_anchor = false;

    let mut pattern = pattern;

    if pattern.starts_with('^') {
        start_anchor = true;
        pattern = &pattern[1..];
    }

    if pattern.ends_with('$') {
        end_anchor = true;
        pattern = &pattern[..pattern.len() - 1];
    }
    
    let parsed_pattern = Pattern::parse_pattern(pattern);
    
    let mut temp = parsed_pattern.iter();
    while let Some(p) = temp.next() {
        println!("the pattern : {:?}", p);
    }
    let mut input = input_line;

    loop {
        'inner: loop {
            for idx in 0..parsed_pattern.len() {
                let subpattern = parsed_pattern.get(idx).unwrap();
                // println!("subpattern : {:?}",subpattern);
                // println!("next input :{}", input);
                match match_character(input, subpattern.clone()) {
                    Ok(res) => {
                        if res.is_empty() {
                            // End of the pattern, match is succesful
                            // println!("res is now empty : idx :{} and parsed_pattern len idx:{}" , idx , parsed_pattern.len() -1);
                            return idx == parsed_pattern.len() - 1;
                        }
                        input = res;
                    }
                    Err(res) => {
                        if start_anchor {
                            // Needed to match from the start
                            return false;
                        }

                        if res.is_empty() {
                            // End of the input, but didn't get the match
                            return false;
                        }

                        input = res;

                        // Reset the pattern
                        break 'inner;
                    }
                }
            }

            // Whole pattern was matched and there's still more input left
            // Match will fail if end anchor was set
            // println!("end anchor : {}", end_anchor);
            return !end_anchor;
        }
    }
}


// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = dbg!(env::args().nth(2).unwrap());
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
