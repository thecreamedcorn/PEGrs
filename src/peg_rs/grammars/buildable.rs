use std::rc::Rc;
use std::collections::HashMap;
use peg_rs::grammars::grammar_node::GrammarNode;

pub trait Buildable {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Rc<GrammarNode>;
}