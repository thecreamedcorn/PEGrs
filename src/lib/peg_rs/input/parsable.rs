use std::str::Chars;

pub trait Parsable {
    type Loc: Clone;

    fn peek(&self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn get_loc(&self) -> Self::Loc;
    fn goto_loc(&mut self, loc: &Self::Loc);
}

pub struct ParsableString<'a> {
    string: &'a str,
    loc: Chars<'a>,
    cur: Option<char>
}

impl<'a> ParsableString<'a> {
    pub fn new(string: &'a str) -> ParsableString<'a> {
        let mut loc = string.chars();
        let cur = loc.next();

        ParsableString { string, loc, cur }
    }
}

impl<'a> Parsable for ParsableString<'a> {
    type Loc = Chars<'a>;

    fn peek(&self) -> Option<char> {
        self.cur
    }

    fn next(&mut self) -> Option<char> {
        self.cur = self.loc.next();
        self.peek()
    }

    fn get_loc(&self) -> Self::Loc {
        self.loc.clone()
    }

    fn goto_loc(&mut self, loc: &Self::Loc) {
        self.loc = loc.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsable_string() {
        let mut input = ParsableString::new("test");
        let loc = input.get_loc();
        assert_eq!(input.peek().unwrap(), 't');
        input.next();
        assert_eq!(input.next().unwrap(), 's');
        input.goto_loc(&loc);
        assert_eq!(input.next().unwrap(), 'e');
        input.goto_loc(&loc);
    }
}