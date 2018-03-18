use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;

pub struct CaptureNode {
    pub name: String,
    pub child: Box<GrammarNode>,
}

pub struct Capture {
    name: String,
    child: Box<Buildable>,
}

impl GrammarNode for CaptureNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        match self.child.run(input) {
            ParseResult::Success(parse_data) => {
                match parse_data.match_data {
                    MatchData::Match(string, node) => {
                        let map = HashMap::new();
                        map.insert(string, node)
                    },
                    MatchData::Collect(collection) => {
                        ParseResult::Success(ParseData {
                            match_data: MatchData::Collect(collection),
                            call_list: parse_data.call_list,
                        })
                    },
                }
            },
            ParseResult::Failure => ParseResult::Failure,
        }
    }
}

impl Capture {
    fn new(name: String, child: Box<Buildable>) -> Capture {
        Capture {
            name,
            child,
        }
    }
}

impl Buildable for Capture {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(grammar_node) => Result::Ok(
                Box::new(CaptureNode {
                    name: self.name.clone(),
                    child: grammar_node,
                })
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}



