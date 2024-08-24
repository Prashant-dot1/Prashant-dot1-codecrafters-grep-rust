use anyhow::Result;

#[derive(Debug,Clone, PartialEq, PartialOrd)]
pub enum Pattern {
    Numeric,
    AlphaNumeric,
    ExactChar(char),
    Group(Vec<Pattern>),
    NegativeGroup(Vec<Pattern>),
    Optional(Box<Pattern>),
    Either((Vec<Pattern>,Vec<Pattern>)),
    CaptureGroup(Vec<Pattern>),
    AnyChar,
    RepeatedOptional(Box<Pattern>),
    Reference(usize)
}

enum Modifier {
    OneOrMore,
    ZeroOrOne,
    ZeroOrMore,
    Reference(usize),
}

impl Pattern {

    fn special_char(c : char) -> Pattern {
        match c { 
            'd' => Pattern::Numeric,
            'w' => Pattern::AlphaNumeric,
            '\\' => Pattern::ExactChar('\\'),
            _ => panic!("Unhandled special char")
        }
    }

    fn parse_char_by_char(input : &str) -> (&str , Option<Pattern> , Option<Modifier>){

        match input.chars().next() {
            Some('\\') => {
                let rem = &input[2..];

                match input.chars().nth(1).unwrap() {
                    index @'1'..='9' => {
                        (
                            rem,
                            None,
                            Some(Modifier::Reference(index.to_digit(10).unwrap() as usize))
                        )
                    }
                    c => {
                        (
                            rem,
                            Some(Pattern::special_char(c)),
                            None
                        )
                    }
                }
            },
            Some('[') => {
                let mut rem = &input[1..];

                let mut negated = false;
                if &input[1..2] == "^" {
                    negated = true;
                    rem = &rem[1..];
                }

                let position = rem.find(']').expect("Terminated wrongly");

                let vec_pattern = Pattern::parse_pattern(&rem[..position]);
                rem = &rem[position..];

                let res_group = if negated {
                    Pattern::NegativeGroup(vec_pattern)
                }
                else {
                    Pattern::Group(vec_pattern)
                };

                (&rem[1..], Some(res_group) , None)
            },
            Some('(') => {
                let mut rem = &input[1..];


                let left  = if let Some(pos) = rem.find('|') {
                    let pattern = &rem[..pos];
                    rem = &rem[pos+1..];

                    Some(Pattern::parse_pattern(pattern))
                }
                else {
                    None
                };

                let pos = rem.find(')').expect("Unterminated )");
                let right = Pattern::parse_pattern(&rem[..pos]);
                rem = &rem[pos+1..];

                let res = match left {
                    Some(left) => {
                        vec![Pattern::Either((left, right))]
                    },
                    _ => right
                };


                (rem, Some(Pattern::CaptureGroup(res)), None)
            },
            Some('+') => (&input[1..], None, Some(Modifier::OneOrMore)),
            Some('?') => (&input[1..], None, Some(Modifier::ZeroOrOne)),
            Some('*') => (&input[1..], None, Some(Modifier::ZeroOrMore)),
            Some('.') => (&input[1..], Some(Pattern::AnyChar), None),
            Some(c) => (&input[1..], Some(Pattern::ExactChar(c)), None),
            _ => panic!("Unhandled characted")
        }
    }


    pub fn parse_pattern(input : &str) -> Vec<Pattern> {

        let mut items : Vec<Pattern> = Vec::new();
        let mut remainder = input;

        while !remainder.is_empty() {
            let (remaining_inp , character , modifier) = Pattern::parse_char_by_char(remainder);

            match modifier {
                Some(Modifier::OneOrMore) => {
                    let prev = items.last().unwrap().clone();

                    items.push(Pattern::RepeatedOptional(Box::new(prev)))
                },
                Some(Modifier::ZeroOrMore) => {
                    let prev = items.pop().unwrap();
                    items.push(Pattern::RepeatedOptional(Box::new(prev)))
                },
                Some(Modifier::ZeroOrOne) => {
                    let prev = items.pop().unwrap();
                    items.push(Pattern::Optional(Box::new(prev)));
                },
                Some(Modifier::Reference(index)) => {
                    // let g = items
                    //                     .iter()
                    //                     .filter(|p| matches!(p , Pattern::CaptureGroup(_)))
                    //                     .nth(index-1)
                    //                     .unwrap();

                    // if let Pattern::CaptureGroup(res) = g {
                    //     items.extend(res.clone())
                    // }
                    items.push(Pattern::Reference(index));
                },
                None => {
                    items.push(character.expect("Should have a character without modifier"));
                }
            }
            remainder = remaining_inp;
        }

        items
    }
}


pub fn match_character<'a>(input : &'a str , subpattern : Pattern, captured: &mut Vec<String>) -> Result<&'a str , & 'a str> {

    if input.is_empty() {
        return Ok("");
    }
    let mut input = input;
    let ch = input.chars().next().unwrap();
    
    match subpattern {
        Pattern::AnyChar => to_match_result(input, true),
        Pattern::ExactChar(c) => to_match_result(input, c == ch),
        Pattern::Numeric => to_match_result(input, ch.is_ascii_digit()),
        Pattern::AlphaNumeric => to_match_result(input, ch == '_' || ch.is_ascii_alphanumeric()),
        Pattern::Group(items) => to_match_result(
            input,
            items
                .iter()
                .any(|i| match_character(input, i.clone(),captured).is_ok()),
        ),
        Pattern::NegativeGroup(items) => to_match_result(
            input,
            !items
                .iter()
                .any(|i| match_character(input, i.clone(),captured).is_ok()),
        ),
        Pattern::Optional(c) => {
            if match_character(input, *c.clone(),captured).is_ok() {
                return Ok(&input[1..])
            } else {
                return Ok(input)
            }
        },
        Pattern::RepeatedOptional(c) => {
            loop {
                if match_character(input, *c.clone(), captured).is_ok() {
                     if input.is_empty() {
                         break;
                     }
                    input = &input[1..];
                } else {
                    println!(" not match ");
                    break;
                }
            }

            return Ok(input)
        },
        Pattern::Either((left, right)) => {
            if let Ok(res) = check_branch(input, left, captured) {
                Ok(res)
            } else if let Ok(res) = check_branch(input, right, captured) {
                Ok(res)
            } else {
                Err(input)
            }
        },
        Pattern::CaptureGroup(group) => {
            let start = input;
            if let Ok(res) = check_branch(input, group, captured) {

                let cg = &start[..start.len()-res.len()];
                captured.push(cg.to_string());

                Ok(res)
            } else {
                Err(&input[1..])
            }
        },
        Pattern::Reference(index) => {
            let group = captured.get(index - 1).unwrap();
            if input.starts_with(group) {
                Ok(&input[group.len()..])
            } else {
                Err(input)
            }
        }
    }

}


fn to_match_result(inp : &str , has_match : bool) -> Result<&str , &str> {

    if has_match {
        return Ok(&inp[1..]);
    }
    else {
        Err(&inp[1..])
    }
}


  
fn check_branch<'a>(input: &'a str, chars: Vec<Pattern>, captured : &mut Vec<String>) -> Result<&'a str, &'a str> {
    let mut input_mut = input;

    for ch in chars {
        match match_character(input_mut, ch, captured) {
            Ok(res) => {
                println!("{}", res);
                input_mut = res;
            }
            Err(_) => {
                return Err(input);
            }
        }
    }

    println!("sting returned :{}", input_mut);
    Ok(input_mut)
}