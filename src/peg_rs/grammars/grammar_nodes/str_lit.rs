use peg_rs::grammars::grammar_node::*;

pub struct StrLitNode {
    pub string: String
}

pub struct StrLit {
    pub string: String
}

impl GrammarNode for StrLitNode {

    fn run<'a>(&self, input: &mut Parsable<'a>) -> ParseResult<'a> {
        let mut chars = self.string.chars();
        let begin = input.get_loc();

        loop {
            let str_in = chars.next();

            match str_in {
                Option::Some(str_char) => {
                    match input.next() {
                        Option::Some(in_char) => {
                            if in_char != str_char {
                                input.goto_loc(begin);
                                return ParseResult::FAILURE;
                            }
                        },
                        Option::None => {
                            input.goto_loc(begin);
                            return ParseResult::FAILURE;
                        }
                    }
                },
                Option::None => return ParseResult::new_empty()
            }
        }
    }
}

#[test]
fn test_str_lit() {
    let lit = StrLitNode{ string: "test".to_string() };
    let res = lit.run(&mut Parsable::new("test"));
    match res {
        ParseResult::FAILURE => panic!(),
        _ => ()
    }

    let res = lit.run(&mut Parsable::new("testing"));
    match res {
        ParseResult::FAILURE => panic!(),
        _ => ()
    }

    let res = lit.run(&mut Parsable::new("not"));
    match res {
        ParseResult::SUCCESS(_) => panic!(),
        _ => ()
    }

    let res = lit.run(&mut Parsable::new("te"));
    match res {
        ParseResult::SUCCESS(_) => panic!(),
        _ => ()
    }
}