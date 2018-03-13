pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use peg_rs::grammars::grammar_nodes::production::*;
pub use peg_rs::grammars::grammar_node::GrammarNode;

pub trait Buildable {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String>;
}