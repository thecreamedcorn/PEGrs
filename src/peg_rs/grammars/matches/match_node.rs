use std::collections::HashMap;
use std::rc::Rc;

pub struct MatchNode<'a> {
    pub slice: &'a str,
    pub children: HashMap<String, Vec<Rc<MatchNode<'a>>>>
}