use std::boxed::Box;
use std::result::Result;
use std::ops::Deref;

use peg_rs::interfaces::*;
use peg_rs::grammar_nodes::empty::EmptyNode;

pub struct ProductionNode {
    pub child: Box<GrammarNode>
}

pub struct Production {
    pub name: String,
    pub child: Box<Buildable>
}

impl GrammarNode for ProductionNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        match self.child.run(input) {
            ParseResult::Success(parse_data) => {
                ParseResult::Success(
                    ParseData {
                        match_data: MatchData::Collect(HashMap::new()),
                        call_list: parse_data.call_list
                    }
                )
            }
            ParseResult::Failure => ParseResult::Failure,
        }
    }
}

impl Production {
    pub fn new(name: &str, child: Box<Buildable>) -> Production {
        Production {
            name: name.to_string(),
            child,
        }
    }

    pub fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Rc<RefCell<ProductionNode>>, String> {
        if map.contains_key(&self.name) {
            Result::Ok(map.get(&self.name).unwrap().clone())
        } else {
            let this: Rc<RefCell<ProductionNode>> = Rc::new(RefCell::new(
                ProductionNode {
                    child: Box::new(EmptyNode),
                }
            ));
            map.insert(self.name.clone(), this.clone());
            match self.child.build(map, prods) {
                Result::Ok(boxed_node) => {
                    this.deref().borrow_mut().child = boxed_node;
                    Result::Ok(this.clone())
                },
                Result::Err(err) => Result::Err(err),
            }
        }
    }
}

#[test]
fn test_production() {
    use ::*;

    /*
    //Outline of this grammar
    Prod1 <- 'test' ('cool' | Prod2)
    Prod2 <- Prod1 'yeet'
    */

    let grammar = GrammarBuilder::new(
        Production::new("Prod1",
            Union::new(vec!(
                StrLit::new("test"),
                Choice::new(vec!(
                    StrLit::new("cool"),
                    ProductionRef::new("Prod2"),
                )),
            ))
        ))
        .add_prod(Production::new("Prod2",
            Union::new(vec!(
                ProductionRef::new("Prod1"),
                StrLit::new("yeet"),
            ))
        ))
        .build().unwrap();

    assert!(!grammar.parse("test"));
    assert!(grammar.parse("testtestcoolyeet"));
    assert!(!grammar.parse("testtesttesttest"));
    assert!(grammar.parse("testcoolyeet"));
}