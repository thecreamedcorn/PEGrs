use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::matches::match_collector::*;

pub struct OneOrMoreNode {
    child: Box<GrammarNode>,
}

pub struct OneOrMore {
    child: Box<Buildable>,
}

impl OneOrMore {
    pub fn new(child: Box<Buildable>) -> Box<OneOrMore> {
        Box::new(OneOrMore{child})
    }
}

impl GrammarNode for OneOrMoreNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        let mut match_data = MatchCollector::new();
        let mut call_list = Vec::new();

        match self.child.run(input) {
            ParseResult::Success(mut parse_data) => {
                match_data.add(parse_data.match_data);
                call_list.append(&mut parse_data.call_list);
            },
            ParseResult::Failure => return ParseResult::Failure,
        }

        loop {
            match self.child.run(input) {
                ParseResult::Success(mut parse_data) => {
                    match_data.add(parse_data.match_data);
                    call_list.append(&mut parse_data.call_list);
                },
                ParseResult::Failure => break,
            }
        }

        ParseResult::Success(
            ParseData {
                match_data: MatchData::Collect(match_data.get_collection()),
                call_list,
            }
        )
    }
}

impl Buildable for OneOrMore {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(grammar_node) => Result::Ok(
                Box::new(OneOrMoreNode {
                    child: grammar_node
                })
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_one_or_more() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            OneOrMore::new(
                StrLit::new("test")
            )
        )).build().unwrap();

    assert!(grammar.parse("test"));
    assert!(grammar.parse("testtest"));
    assert!(grammar.parse("testtesttest"));
    assert!(!grammar.parse(""));
}