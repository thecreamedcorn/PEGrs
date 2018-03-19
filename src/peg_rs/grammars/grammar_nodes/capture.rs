use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::matches::capture_tree::CaptureTree;

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
        let begin = input.get_loc();
        match self.child.run(input) {
            ParseResult::Success(parse_data) => {
                match parse_data.match_data {
                    MatchData::Match(string, node) => {
                        let mut map = HashMap::new();
                        map.insert(string, vec!(Rc::new(node)));
                        ParseResult::Success(ParseData {
                            match_data: MatchData::Match(
                                self.name.clone(),
                                CaptureTree {
                                    content: input.sub_string(begin, input.get_loc()),
                                    children: map,
                                }
                            ),
                            call_list: parse_data.call_list,
                        })
                    },
                    MatchData::Collect(collection) => {
                        ParseResult::Success(ParseData {
                            match_data: MatchData::Match(
                                self.name.clone(),
                                CaptureTree {
                                    content: input.sub_string(begin, input.get_loc()),
                                    children: collection,
                                }
                            ),
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
    fn new(name: &str, child: Box<Buildable>) -> Box<Capture> {
        Box::new(Capture {
            name: name.to_string(),
            child,
        })
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

#[test]
fn test_capture() {
    use peg_rs::grammars::grammar_nodes::production::*;
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let string: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            SemAct::new(
                Capture::new(
                    "my_cap",
                    StrLit::new("test")
                ),
                Rc::new({
                    let string_copy = string.clone();
                    move |ct: &CaptureTree| {
                        println!("{:?}", ct);
                        *(string_copy.borrow_mut()) = ct.children.get("my_cap").unwrap()[0].content.to_string();
                    }
                })
            )
        ))
        .build().unwrap();

    grammar.parse("test");
    assert_eq!(*string.borrow(), "test");
}