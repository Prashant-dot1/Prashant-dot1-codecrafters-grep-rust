use std::{collections::HashSet, str::FromStr};

use map_macro::hash_set;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Pattern {
    Numeric,
    AlphaNumeric,
    ExactChar(char),
    Sequence(Vec<Pattern>),
    CharacterSet {
        chars : String,
        negated : bool
    },
    // StartStringAnchor(Box<Pattern>)
    StartStringAnchor(String),
    EndStringAnchor(String),
    OneOrMore(char),
    Optional(char)
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters = s.chars().peekable();
        let mut items = Vec::new();

        if s.ends_with('$') {
            return Ok(Pattern::EndStringAnchor(s[..(s.len()-1)].to_string()));
        }
        while let Some(c) = characters.next() {
            let element = match c {
                '\\' => match characters.next() {
                    Some('d') => Pattern::Numeric,
                    Some('w') => Pattern::AlphaNumeric,
                    Some(c) => Pattern::ExactChar(c),
                    None => panic!("somethign wrong")
                },
                '[' => {
                    let mut chars = String::new();
                    let mut negated = false;
                    let mut end = false;

                    while let Some(c) = characters.next() {
                        match c {
                            '^' => {
                                negated = true;
                            },
                            ']' => {
                                end = true;
                                break;
                            },
                            other => {
                                chars.push(c);
                            }
                        }
                    }

                    if !end {
                        return Err("Unterminated pattern '['".to_string());
                    }

                    Pattern::CharacterSet { chars: chars, negated }
                },
                // '^' => {
                //     let mut newStr = String::new();
                //     while let Some(c) = characters.next() {
                //         newStr.push(c)
                //     }
                //     let newP = Pattern::from_str(&newStr).unwrap();

                //     Pattern::StartStringAnchor(Box::new(newP))

                // },
                '^' => {
                    let mut newStr = String::new();
                    while let Some(c) = characters.next() {
                        newStr.push(c)
                    }

                    Pattern::StartStringAnchor(newStr)

                },
                e => { 
                    if characters.next_if(|&c| c == '+').is_some() {
                        Pattern::OneOrMore(e)
                    }
                    else if characters.next_if(|&c| c == '?').is_some() {
                        Pattern::Optional(e)
                    }
                    else{
                        Pattern::ExactChar(e)
                    }
                }
            };

            items.push(element);
        }

        if items.len() == 1 {
            return Ok(items.pop().expect("Has one item"));
        }

        Ok(Pattern::Sequence(items))
    }
}


trait CharOps {
    fn first_char(&self) -> Option<char>;
    fn first_char_in(&self, options: &str) -> bool;
    fn skip_first_char(&self) -> &str;
    fn get_starting_string<'a>(&'a self, p: &Pattern) -> Option<&'a str>;
}

impl CharOps for str {
    fn first_char(&self) -> Option<char> {
        self.chars().next()
    }

    fn first_char_in(&self, options: &str) -> bool {
        match self.chars().next() {
            Some(c) => options.contains(c),
            None => false,
        }
    }

    fn skip_first_char(&self) -> &str {
        &self[1..]
    }

    fn get_starting_string<'a>(&'a self, p: &Pattern) -> Option<&'a str> {

        match *p {
            Pattern::Numeric | Pattern::AlphaNumeric | Pattern::ExactChar(_) => {
                for i in 0..self.len() {
                    let inp = &self[i..];
        
                    if !p.match_string(inp).is_empty() {
                        return Some(inp);
                    }
                }
                None
            },
            _ => Some(self)
        }
    }
}

impl Pattern {
    pub fn match_string(&self, input: &str) -> HashSet<String> {
        match self {
            Pattern::ExactChar(c) if input.first_char().unwrap() == *c => {
                hash_set! { input.skip_first_char().to_string() }
            },
            Pattern::Numeric if input.first_char_in("0123456789") => {
                hash_set! { input.skip_first_char().to_string() }
            },
            Pattern::AlphaNumeric if input.first_char_in("_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") => {
                hash_set! { input.skip_first_char().to_string() }
            },
            Pattern::Sequence(sub_patterns) => {
                let mut currect_input = hash_set! { input.to_string() };
                for subpattern in sub_patterns {
                    let mut remaining_input = HashSet::new();

                    for inp in currect_input.iter() {
                        if let Some(starting_inp) = inp.as_str().get_starting_string(subpattern) {
                            let res = subpattern.match_string(starting_inp);
                            remaining_input.extend(res);
                        }
                    }

                    if remaining_input.is_empty() {
                        return HashSet::new();
                    }

                    currect_input = remaining_input;
                }
                currect_input
            },
            Pattern::CharacterSet { chars, negated } => {

                if !input.is_empty() && input.first_char_in(chars) != *negated {
                    hash_set! {input.skip_first_char().to_string()}
                }
                else {
                    HashSet::new()
                }
            },
            Pattern::StartStringAnchor(newPattern) => {
                if !input.is_empty() {
                    if input.starts_with(newPattern) {
                        if input == newPattern { return hash_set! {"".to_string()}};
                    }
                    return HashSet::new()
                }
                else{
                    HashSet::new()
                }

            },
            Pattern::EndStringAnchor(newPattern) => {
                if !input.is_empty() {
                    if input.ends_with(newPattern) {
                        return hash_set! {"".to_string()};
                    }
                    return HashSet::new()
                }
                return HashSet::new();
            },
            Pattern::OneOrMore(repeat_c) => {
                
                println!("input got : {input}");
                let mut inp = input.chars().peekable();
                let mut i = 0;
                
                if inp.next_if(|&c| c == *repeat_c).is_some() {
                    i = i+1;
                    while inp.next_if(|&c| c == *repeat_c).is_some() {
                        i = i+1;
                    } 
                }

                if i >= 1 {
                    println!("after one or more : {}",&input[i..]);
                    return hash_set! {input[i..].to_string()};
                }

                return HashSet::new();

            },
            Pattern::Optional(option_c) => {

                return hash_set! {input[1..].to_string()};
            }
            _ => HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn sally_3() {
        let input = "this 3 apples";
        let pattern  = Pattern::from_str("\\d apple").unwrap();

        assert_eq!(pattern.match_string(input),hash_set! {"s".to_string()});
        
    }
}