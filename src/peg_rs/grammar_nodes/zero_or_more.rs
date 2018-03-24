use peg_rs::interfaces::*;
use peg_rs::utils::match_collector::MatchCollector;

pub struct ZeroOrMoreNode {
    child: Box<GrammarNode>,
}

pub struct ZeroOrMore {
    child: Box<Buildable>,
}

impl ZeroOrMore {
    pub fn new(child: Box<Buildable>) -> Box<ZeroOrMore> {
        Box::new(ZeroOrMore{child})
    }
}

impl GrammarNode for ZeroOrMoreNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        let mut match_data = MatchCollector::new();
        let mut call_list = Vec::new();

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

impl Buildable for ZeroOrMore {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(grammar_node) => Result::Ok(
                Box::new(ZeroOrMoreNode {
                    child: grammar_node
                })
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_zero_or_more() {
    use ::*;

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            ZeroOrMore::new(
                StrLit::new("test")
            )
        )).build().unwrap();

    assert!(grammar.parse("test"));
    assert!(grammar.parse("testtest"));
    assert!(grammar.parse("testtesttest"));
    assert!(grammar.parse(""));
}