pub use peg_rs::input::parsable::Parsable;
pub use peg_rs::grammars::parse_result::*;

pub trait GrammarNode {
    fn run(&self, input: &mut Parsable) -> ParseResult;
}