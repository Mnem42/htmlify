use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum MDItem{
    Heading(usize,String),
    Line(String)
}


pub struct Parser <'a>{
    contained: Iter<'a, String>
}

fn check_if_heading(input:&str) -> Option<MDItem>{
    // Skip all hashes and collect
    let chars = input
        .trim()
        .chars()
        .by_ref()
        .skip_while(|x| *x=='#')
        .collect::<String>();

    // Find the length difference between both input and chars (after trimming
    // both to remove meaningless spaces)
    let length_difference = input.trim().len()-chars.trim().len();

    // If there's no difference, it's not a heading
    match length_difference{
        0 => Some(MDItem::Line(input.trim().to_string())),
        x => Some(MDItem::Heading(x-1,chars.trim().to_string()))
    }
}

impl Parser<'_>{
    pub fn new(input: &[String]) -> Parser<'_>{
        Parser{
            contained:input.iter()
        }
    }

    fn parse(self, input: &str) -> Option<MDItem>{
        check_if_heading(input)
    }
}

impl Iterator for Parser<'_>{
    type Item = MDItem;

    fn next(&mut self) -> Option<MDItem>{
        let item = self.contained.next()?;
        check_if_heading(item)
    }
}