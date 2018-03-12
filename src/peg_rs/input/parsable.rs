use std::str::CharIndices;

pub struct Parsable<'a> {
    string: &'a str,
    loc: CharIndices<'a>,
    cur: Option<(usize, char)>
}

impl<'a> Parsable<'a> {
    pub fn new(string: &'a str) -> Parsable {
        let mut loc = string.char_indices();
        let cur = loc.next();
        Parsable { string, loc, cur }
    }

    pub fn peek(&self) -> Option<char> {
        match self.cur {
            Option::Some((_, c)) => Option::Some(c),
            Option::None => Option::None
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let cur = self.peek();
        self.cur = self.loc.next();
        cur
    }

    pub fn get_loc(&self) -> usize {
        match self.cur {
            Option::Some((s, _)) => s,
            Option::None => self.string.len()
        }
    }

    pub fn goto_loc(&mut self, loc: usize) {
        if loc >= self.string.len() {
            self.loc = self.string.char_indices();
            self.cur = Option::None
        } else {
            self.loc = self.string[loc..].char_indices();
            self.cur = self.loc.next()
        }
    }

    pub fn sub(&self, loc1: usize, loc2: usize) -> &str {
        &self.string[loc1..loc2]
    }
}


#[test]
fn test_parsable() {
    let mut input = Parsable::new("test");
    let loc = input.get_loc();
    assert_eq!(input.peek().unwrap(), 't');
    input.next();
    assert_eq!(input.next().unwrap(), 'e');
    input.goto_loc(loc);
    assert_eq!(input.next().unwrap(), 't');
    assert_eq!(input.sub(0, 4), "test");
}