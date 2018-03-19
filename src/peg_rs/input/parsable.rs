use std::ops::Deref;
use std::rc::Rc;

pub struct Parsable {
    content: Rc<Vec<char>>,
    cur: usize,
}

#[derive(Debug)]
pub struct ContentRef {
    content: Rc<Vec<char>>,
    start: usize,
    end: usize,
}

impl Parsable {
    pub fn new(string: &str) -> Parsable {
        Parsable {
            content: Rc::new(string.chars().collect()),
            cur: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.cur < self.content.len() {
            Option::Some(self.content[self.cur])
        } else {
            Option::None
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let cur = self.peek();
        if cur != Option::None {
            self.cur += 1
        }
        cur
    }

    pub fn get_loc(&self) -> usize {
        self.cur
    }

    pub fn goto_loc(&mut self, loc: usize) {
        if loc < self.content.len() {
            self.cur = loc;
        }
    }

    pub fn sub_string(&self, start: usize, end: usize) -> ContentRef {
        ContentRef {
            content: self.content.clone(),
            start,
            end,
        }
    }
}

impl ContentRef {
    pub fn to_string(&self) -> String {
        self.content.deref()[self.start..self.end].iter().collect()
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
    assert_eq!(input.sub_string(0, 4).to_string(), "test".to_string());
}