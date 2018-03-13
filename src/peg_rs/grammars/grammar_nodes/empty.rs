use peg_rs::grammars::grammar_node::*;

pub struct EmptyNode;

impl GrammarNode for EmptyNode {
    fn run<'a>(&self, _input: &mut Parsable<'a>) -> ParseResult<'a> {
        ParseResult::new_empty()
    }
}