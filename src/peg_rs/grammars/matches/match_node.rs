use std::collections::HashMap;
use std::rc::Rc;

pub struct MatchNode<'a> {
    slice: &'a str,
    children: HashMap<String, Vec<MatchNode<'a>>>
}

impl<'a> MatchNode<'a> {
    fn get_children(&self) -> &HashMap<String, Vec<MatchNode<'a>>> {
        &self.children
    }

    fn get_slice(&self) -> &'a str {
        self.slice
    }
}