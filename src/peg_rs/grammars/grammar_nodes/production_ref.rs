use std::boxed::Box;
use std::result::Result;
use std::ops::Deref;

use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::*;
use peg_rs::grammars::buildable::*;

pub struct ProductionRefNode {
    pub prod: Rc<RefCell<ProductionNode>>,
}

pub struct ProductionRef {
    pub name: String
}

impl GrammarNode for ProductionRefNode {
    fn run<'a>(&self, input: &mut Parsable<'a>) -> ParseResult<'a> {
        self.prod.deref().borrow_mut().run(input)
    }
}

impl ProductionRef {
    pub fn new(string: &str) -> ProductionRef {
        ProductionRef{ name: string.to_string() }
    }
}

impl Buildable for ProductionRef {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match prods.get(&self.name) {
            Option::Some(prod) => {
                match prod.build(map, prods) {
                    Result::Ok(prod) => Result::Ok(
                        Box::new(ProductionRefNode {
                            prod
                        })
                    ),
                    Result::Err(err) => Result::Err(err),
                }
            },
            Option::None => Result::Err(format!("could not find production of name {}", &self.name)),
        }
    }
}