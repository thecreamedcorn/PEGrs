use std::collections::HashMap;
use std::rc::Rc;
use peg_rs::input::parsable::ContentRef;

#[derive(Debug)]
pub struct CaptureTree {
    pub content: ContentRef,
    pub children: HashMap<String, Vec<Rc<CaptureTree>>>,
}