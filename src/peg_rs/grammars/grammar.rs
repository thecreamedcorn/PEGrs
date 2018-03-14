use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;

pub struct Grammar {
    pub root: Rc<RefCell<ProductionNode>>,
}

impl Grammar {
    pub fn parse<'a>(&self, string: &'a str ) -> bool {
        let result = self.root.deref().borrow().run(&mut Parsable::new(string));
        match result {
            ParseResult::SUCCESS(parse_data) => {
                for (mut func, match_node) in parse_data.call_list {
                    func(&match_node)
                }
                true
            }
            ParseResult::FAILURE => false
        }
    }
}