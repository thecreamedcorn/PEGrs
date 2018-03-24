pub use peg_rs::utils::parsable::Parsable;
pub use peg_rs::utils::parse_result::*;

pub trait GrammarNode {
    fn run(&self, input: &mut Parsable) -> ParseResult;
}