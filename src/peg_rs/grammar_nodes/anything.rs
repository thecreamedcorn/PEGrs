use peg_rs::interfaces::*;
use peg_rs::utils::parse_result::ParseResult;
use peg_rs::grammar_nodes::production::ProductionNode;

pub struct AnythingNode;

pub struct Anything;

impl Anything {
    pub fn new() -> Anything {
        Anything {}
    }
}

impl GrammarNode for AnythingNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        input.next();
        ParseResult::new_empty()
    }
}

impl Buildable for Anything {
    fn build(&self, _map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, _prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        Result::Ok(Box::new(AnythingNode {}))
    }
}