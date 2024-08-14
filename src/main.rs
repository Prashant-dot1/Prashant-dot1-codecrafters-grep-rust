use std::env;
use std::io;
use std::ops::Index;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } 
    else if pattern == "\\d" {
        // for i in 0..10 {
        //     let pattern = ('0' as u8 + i) as char;
        //     println!("printing pattern : {pattern}");
            
        //     if input_line.contains(pattern) {
        //         println!("the parttern matched for {pattern}");
        //         return true;
        //     }
        // }
        // return false;
        return input_line.contains(|c: char| {c.is_digit(10)});
    }
    else if pattern == "\\w" {
        return input_line.contains(|c : char| {c.is_alphanumeric()});
    }
    else if pattern.starts_with("[") && pattern.ends_with("]"){
        let len = pattern.len();

        match &pattern.chars().nth(1).unwrap() {
            '^' => { 
                let pat = &pattern[2..len-1];
                println!("pat : {pat}");
                let list_of_chars : Vec<char> = pat.chars().collect();
                return !input_line.contains(&list_of_chars[..])
            },
            _ => {
                let pat = &pattern[1..len-1];
                println!("pat : {pat}");
                let list_of_chars : Vec<char> = pat.chars().collect();
                return input_line.contains(&list_of_chars[..])
            }
        };
    }
    else {
        panic!("Unhandled pattern: {}", pattern)
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
