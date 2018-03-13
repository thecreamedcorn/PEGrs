pub use std::collections::HashMap;
pub use std::rc::Rc;
use peg_rs::grammars::matches::match_node::MatchNode;
use peg_rs::grammars::parse_result::*;

pub struct MatchCollector<'a> {
    collection: HashMap<String, Vec<Rc<MatchNode<'a>>>>,
}

impl<'a> MatchCollector<'a> {
    pub fn new() -> MatchCollector<'a> {
        MatchCollector {
            collection: HashMap::new(),
        }
    }

    pub fn add(&mut self, match_data: MatchData<'a>) {
        match match_data {
            MatchData::COLLECT(collection) => {
                for (key, mut vec) in collection {
                    if self.collection.contains_key(&key) {
                        self.collection.get_mut(&key).unwrap().append(&mut vec);
                    } else {
                        self.collection.insert(key, vec);
                    }
                }
            }
            MatchData::MATCH(key, mc) => {
                if self.collection.contains_key(&key) {
                    self.collection.get_mut(&key).unwrap().push(Rc::new(mc));
                } else {
                    self.collection.insert(key, vec!(Rc::new(mc)));
                }
            }
        }
    }

    pub fn get_collection(self) ->  HashMap<String, Vec<Rc<MatchNode<'a>>>> {
        self.collection
    }
}