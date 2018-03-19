use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::buildable::*;

pub struct StrLitNode {
    pub string: String
}

pub struct StrLit {
    string: String
}

impl GrammarNode for StrLitNode {

    fn run(&self, input: &mut Parsable) -> ParseResult {
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
                                return ParseResult::Failure;
                            }
                        },
                        Option::None => {
                            input.goto_loc(begin);
                            return ParseResult::Failure;
                        }
                    }
                },
                Option::None => return ParseResult::new_empty()
            }
        }
    }
}

impl StrLit {
    pub fn new(string: &str) -> Box<StrLit> {
        Box::new(StrLit { string: string.to_string() })
    }
}

impl Buildable for StrLit {
    fn build(&self, _map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, _prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        Result::Ok(Box::new(StrLitNode{ string: self.string.clone() }))
    }
}

#[test]
fn test_str_lit() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new("TestStrLit",
            StrLit::new("test")
        ))
        .build().unwrap();

    assert!(grammar.parse("test"));
    assert!(!grammar.parse("te"));
    assert!(!grammar.parse("tess"));
    assert!(grammar.parse("testing"));
}