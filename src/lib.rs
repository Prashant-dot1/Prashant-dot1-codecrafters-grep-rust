use std::{borrow::Borrow, cell::RefCell, collections::HashSet, hash::Hash, rc::Rc, str::FromStr};

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
        for i in 0..self.len() {
            let inp = &self[i..];

            if !p.match_string(inp).is_empty() {
                return Some(inp);
            }
        }
        None
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