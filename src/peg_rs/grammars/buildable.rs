pub use std::rc::Rc;
pub use std::collections::HashMap;
pub use peg_rs::grammars::grammar_nodes::production::Production;
pub use peg_rs::grammars::grammar_node::GrammarNode;

pub trait Buildable {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Result<Rc<GrammarNode>, String>;
}