use peg_rs::grammars::buildable::*;
use peg_rs::grammars::matches::capture_tree::CaptureTree;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;

type CharTester = Fn(char) -> bool;

pub struct CharClassNode {
    pub char_testers: Vec<Box<CharTester>>,
}

pub struct CharClass {
    string: String,
}

impl CharClass {
    pub fn new(string: &str) -> Box<CharClass> {
        Box::new(CharClass{ string: string.to_string() })
    }
}

impl GrammarNode for CharClassNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        match input.peek() {
            Option::Some(c) => {
                for ref tester in &self.char_testers {
                    if (*tester)(c) {
                        input.next();
                        return ParseResult::new_empty();
                    }
                }
                ParseResult::Failure
            },
            Option::None => ParseResult::Failure
        }
    }
}

impl Buildable for CharClass {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        Result::Ok(Box::new(CharClassNode{char_testers: parse_string(&self.string)}))
    }
}

/*
Set <- (CharSet / Char)+
CharSet <- '{' NamedCharSet '}'
         / Char '-' Char
NamedCharSet <- 'alpha'
              / 'lower'
              / 'upper'
              / 'ws'
              / 'alnum'
              / 'cont'
              / 'num'
Char <- '\'? .
*/
fn parse_string(string: &str) -> Vec<Box<CharTester>> {
    let chars: Vec<char> = string.chars().collect();
    let mut loc: usize = 0;
    let mut result: Vec<Box<CharTester>> = Vec::new();

    while loc < chars.len() {
        let left = chars.len() - loc;

        match parse_char(&chars, loc) {
            Option::Some((c1, len)) => {

                loc += len;
                let left = chars.len() - loc;
                if left >= 1 && chars[loc] == '-' {
                    match parse_char(&chars, loc + 1) {
                        Option::Some((c2, len)) => {
                            loc += len + 1;

                            result.push(create_range_char_match(c1, c2))
                        }
                        Option::None => {
                            result.push(create_single_char_match(c1))
                        }
                    }
                } else {
                    result.push(create_single_char_match(c1))
                }
            },
            Option::None => break,
        }
    }
    result
}

fn parse_char(chars: &Vec<char>, loc: usize) -> Option<(char, usize)> {
    let left = chars.len() - loc;

    if left < 1 {
        return Option::None
    }

    if chars[loc] == '\\' {
        if left < 2 {
            Option::None
        } else {
            Option::Some((chars[loc + 1], 2))
        }
    } else {
        Option::Some((chars[loc], 1))
    }
}

fn create_single_char_match(c: char) -> Box<CharTester> {
    Box::new(move |test: char| c == test)
}

fn create_range_char_match(start: char, end: char) -> Box<CharTester> {
    Box::new(move |test: char| test >= start && test <= end)
}

#[test]
fn test_char_class() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let num: Rc<RefCell<i64>> = Rc::new(RefCell::new(5));

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            CharClass::new("\\\\\\-0-9a-zA-Z")
        )).build().unwrap();

    assert!(grammar.parse("7"));
    assert!(grammar.parse("B"));
    assert!(grammar.parse("\\"));
    assert!(grammar.parse("-"));
    assert!(!grammar.parse("."));
}