use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::matches::match_collector::*;

pub struct UnionNode {
    pub seq: Vec<Box<GrammarNode>>,
}

pub struct Union {
    seq: Vec<Box<Buildable>>
}

impl Union {
    pub fn new(seq: Vec<Box<Buildable>>) -> Box<Union> {
        Box::new(Union { seq })
    }
}

impl GrammarNode for UnionNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        let mut match_data = MatchCollector::new();
        let mut call_list = Vec::new();

        for rc in &self.seq {
            match rc.run(input) {
                ParseResult::Success(mut parse_data) => {
                    match_data.add(parse_data.match_data);
                    call_list.append(&mut parse_data.call_list);
                },
                ParseResult::Failure => return ParseResult::Failure,
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

impl Buildable for Union {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        let mut un = UnionNode {
            seq: Vec::new(),
        };
        for buildable in &self.seq {
            match buildable.build(map, prods) {
                Result::Ok(gn) => {
                    un.seq.push(gn)
                },
                Result::Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(Box::new(un))
    }
}

#[test]
fn test_union() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new("TestStrLit",
            Union::new(vec!(
                StrLit::new("test"),
                StrLit::new("cool"),
            ))
        ))
        .build().unwrap();

    assert!(grammar.parse("testcool"));
    assert!(!grammar.parse("testco"));
    assert!(!grammar.parse("testcoor"));
    assert!(grammar.parse("testcooling"));
}