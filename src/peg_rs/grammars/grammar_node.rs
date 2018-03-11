use super::super::input::parsable::Parsable;
use super::matches::match_node::MatchTree;

pub enum ParseSuccess {
    SUCCESS,
    FAILURE
}

pub enum MatchParse {
    MATCH(String, MatchNode),
    COLLECT(HashMap<String, Vec<MatchNode>>)
}

pub struct ParseResult {
    successful: ParseSuccess,
    matches: MatchParse,
    call_list: Vec<Box<FnMut(MatchTree)>>
}

pub trait GrammarNode {
    fn run(input: &mut Parseable) -> ParseResult;
}