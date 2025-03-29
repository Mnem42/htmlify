use std::slice::Iter;

#[derive(Debug)]
pub enum MDItem{
    Heading(usize,String),
    Line(String)
}


pub struct Parser <'a>{
    contained: Iter<'a, String>
}

impl MDItem{
    pub fn make_heading_from_str(input:&str) -> Option<MDItem>{
        let chars = input.trim().chars();
        let first_token = chars.take_while(|x| *x=='#').size_hint().0;
        if first_token 
    }
}

impl <'a>Parser<'a>{
    pub fn new(input: &[String]) -> Parser<'_>{
        Parser{
            contained:input.iter()
        }
    }
}

impl <'a>Iterator for Parser<'a>{
    type Item = MDItem;

    fn next(&mut self) -> Option<MDItem>{
        let item = self.contained.next()?;
        MDItem::make_heading_from_str(item)
    }
}