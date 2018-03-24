use std::rc::Rc;
use std::ops::Deref;

use peg_rs::interfaces::*;
use peg_rs::grammar_nodes::production::ProductionNode;
use peg_rs::utils::capture_tree::CaptureTree;

pub struct SemActNode {
    pub child: Box<GrammarNode>,
    pub func: Rc<Fn(&CaptureTree)>,
}

pub struct SemAct {
    child: Box<Buildable>,
    func: Rc<Fn(&CaptureTree)>,
}

impl SemAct {
    pub fn new(child: Box<Buildable>, func: Rc<Fn(&CaptureTree)>) -> Box<SemAct> {
        Box::new(SemAct {
            child,
            func: func.clone(),
        })
    }
}

impl GrammarNode for SemActNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        let begin = input.get_loc();
        match self.child.run(input) {
            ParseResult::Success(mut parse_data) => {
                match parse_data.match_data {
                    MatchData::Collect(collection) => {
                        parse_data.call_list.push((
                            self.func.clone(),
                            Rc::new(
                                CaptureTree {
                                    content: input.sub_string(begin, input.get_loc()),
                                    children: collection.clone(),
                                }
                            )
                        ));
                        ParseResult::Success(ParseData {
                            match_data: MatchData::Collect(collection),
                            call_list: parse_data.call_list,
                        })
                    },
                    MatchData::Match(string, node) => {
                        let match_ref = Rc::new(node);
                        let mut map = HashMap::new();
                        map.insert(string, vec!(match_ref.clone()));
                        parse_data.match_data = MatchData::Collect(map.clone());

                        parse_data.call_list.push((
                            self.func.clone(),
                            Rc::new(CaptureTree {
                                content: input.sub_string(begin, input.get_loc()),
                                children: map.clone(),
                            })
                        ));

                        ParseResult::Success(parse_data)
                    }
                }
            },
            ParseResult::Failure => ParseResult::Failure,
        }
    }
}

impl Buildable for SemAct {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.deref().build(map, prods) {
            Result::Ok(child) => {
                Result::Ok(Box::new(SemActNode {
                    child,
                    func: self.func.clone(),
                }))
            },
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_semantic_actions() {
    use std::cell::RefCell;
    use ::*;

    let num: Rc<RefCell<i64>> = Rc::new(RefCell::new(5));

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod1",
            SemAct::new(
                StrLit::new("test"),
                Rc::new({
                    let num_copy = num.clone();
                    move |_ct: &CaptureTree| {
                        *(num_copy.borrow_mut()) = 10;
                    }
                })
            )
        ))
        .build().unwrap();

    grammar.parse("test");
    assert_eq!(*num.borrow(), 10);
}