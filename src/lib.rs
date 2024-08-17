use std::{str::FromStr};

#[derive(Debug)]
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
}

impl Pattern {
    pub fn match_string<'a>(& self , input : &'a str) -> Option<&'a str> {

        match self {
            Pattern::Numeric if input.first_char_in("0123456789") => {
                Some(input.skip_first_char())
            },
            Pattern::AlphaNumeric if input.first_char_in("_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") => {
                Some(input.skip_first_char())
            },
            Pattern::ExactChar(c) if input.first_char().unwrap() == *c => {
                Some(input.skip_first_char())
            },
            Pattern::Sequence(sub_patterns) => {

                let mut current_input = input;
                for subpattern in sub_patterns {
                    if let Some(remaining_input) = subpattern.match_string(current_input) {
                        current_input = remaining_input
                    }
                    else {
                        return None
                    }
                }
                Some(current_input)
            }
            _ => None
        }
    }
}