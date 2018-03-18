use peg_rs::grammars::grammar_node::*;

pub struct EmptyNode;

impl GrammarNode for EmptyNode {
    fn run(&self, _input: &mut Parsable) -> ParseResult {
        ParseResult::new_empty()
    }
}