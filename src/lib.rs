use std::{collections::HashSet, hash::Hash, str::FromStr};

use map_macro::hash_set;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Pattern {
    Numeric,
    AlphaNumeric,
    ExactChar(char),
    Sequence(Vec<Pattern>)
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters = s.chars();
        let mut items = Vec::new();
        while let Some(c) = characters.next() {
            let element = match c {
                '\\' => match characters.next() {
                    Some('d') => Pattern::Numeric,
                    Some('w') => Pattern::AlphaNumeric,
                    Some(c) => Pattern::ExactChar(c),
                    None => panic!("somethign wrong")
                },
                e => Pattern::ExactChar(e)
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
    fn first_char_in(&self, options : &str) -> bool;
    fn skip_first_char(& self) -> Self;
    fn char_contains(&self , options : &str) -> bool;
}

impl CharOps for &str {
    fn first_char(&self) -> Option<char> {
        self.chars().next()
    }

    fn first_char_in(&self, options : &str) -> bool {
        match self.chars().next() {
            Some(c) => options.contains(c),
            None => false
        }
    }

    fn skip_first_char(&self) -> Self {
        &self[1..]
    }

    fn char_contains(&self , options : &str) -> bool {
        for c in options.chars() {
            if self.contains(c) {
                return true
            }
        }
        return false
    }
}

impl Pattern {
    pub fn match_string<'a>(& self , input : &'a str) -> HashSet<&'a str> {

        match self {
            Pattern::ExactChar(c) if input.first_char().unwrap() == *c => {
                hash_set!{input.skip_first_char()}
            },
            Pattern::Numeric if input.first_char_in("0123456789") => {
                hash_set!{input.skip_first_char()}
            },
            Pattern::AlphaNumeric if input.first_char_in("_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") => {
                hash_set!{input.skip_first_char()}
            },
            Pattern::Sequence(sub_patterns) => {
                let mut currect_input = hash_set! {input};
                for subpattern in sub_patterns {
                    
                    let mut remaining_input = HashSet::new();
                    for inp in currect_input.iter() {
                        let r = subpattern.match_string(inp);

                        remaining_input.extend(r);
                    }
                    currect_input = remaining_input;

                }
                currect_input
            }
            _ => HashSet::new()
        }
    }
}