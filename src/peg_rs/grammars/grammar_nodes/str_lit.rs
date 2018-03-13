use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::buildable::*;

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

impl StrLit {
    pub fn new(string: &str) -> StrLit {
        StrLit { string: string.to_string() }
    }
}

impl Buildable for StrLit {
    fn build(&self, _map: &mut HashMap<String, Rc<GrammarNode>>, _prods: &HashMap<String, Production>) -> Result<Rc<GrammarNode>, String> {
        Result::Ok(Rc::new(StrLitNode{ string: self.string.clone() }))
    }
}

#[test]
fn test_str_lit() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new()
        .add_prod(Production::new("TestStrLit",
                Box::new(StrLit::new("test"))
            )
        )
        .build().unwrap();

    assert!(grammar.parse("test"));
    assert!(!grammar.parse("te"));
    assert!(!grammar.parse("tess"));
    assert!(grammar.parse("testing"));
}