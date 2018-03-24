pub use std::collections::HashMap;
pub use std::rc::Rc;

use peg_rs::utils::capture_tree::CaptureTree;

pub struct ParseData {
    pub match_data: MatchData,
    pub call_list: Vec<(Rc<Fn(&CaptureTree)>, Rc<CaptureTree>)>,
}

pub enum MatchData {
    Match(String, CaptureTree),
    Collect(HashMap<String, Vec<Rc<CaptureTree>>>),
}

pub enum ParseResult {
    Success(ParseData),
    Failure,
}

impl ParseResult {
    pub fn new_empty() -> ParseResult {
        ParseResult::Success(
            ParseData {
                match_data: MatchData::Collect(HashMap::new()),
                call_list: Vec::new(),
            }
        )
    }
}