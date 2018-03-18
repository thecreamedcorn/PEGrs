use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;

pub struct Grammar {
    pub root: Rc<RefCell<ProductionNode>>,
}

impl Grammar {
    pub fn parse(&self, string: &str ) -> bool {
        let result = self.root.deref().borrow().run(&mut Parsable::new(string));
        match result {
            ParseResult::Success(parse_data) => {
                for (func, match_node) in parse_data.call_list {
                    func.deref()(match_node.deref())
                }
                true
            }
            ParseResult::Failure => false
        }
    }
}