pub use std::collections::HashMap;
pub use std::rc::Rc;
use peg_rs::grammars::matches::match_node::MatchNode;

pub struct ParseData<'a> {
    pub match_data: MatchData<'a>,
    pub call_list: Vec<(Box<FnMut(&MatchNode<'a>)>, Rc<MatchNode<'a>>)>,
}

pub enum MatchData<'a> {
    MATCH(String, MatchNode<'a>),
    COLLECT(HashMap<String, Vec<Rc<MatchNode<'a>>>>),
}

pub enum ParseResult<'a> {
    SUCCESS(ParseData<'a>),
    FAILURE,
}

impl<'a> ParseResult<'a> {
    pub fn new_empty() -> ParseResult<'a> {
        ParseResult::SUCCESS(
            ParseData {
                match_data: MatchData::COLLECT(HashMap::new()),
                call_list: Vec::new(),
            }
        )
    }
}