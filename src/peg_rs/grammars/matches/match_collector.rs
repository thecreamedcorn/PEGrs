pub use std::collections::HashMap;
pub use std::rc::Rc;
use peg_rs::grammars::matches::match_node::CaptureTree;
use peg_rs::grammars::parse_result::*;

pub struct MatchCollector {
    collection: HashMap<String, Vec<Rc<CaptureTree>>>,
}

impl MatchCollector {
    pub fn new() -> MatchCollector {
        MatchCollector {
            collection: HashMap::new(),
        }
    }

    pub fn add(&mut self, match_data: MatchData) {
        match match_data {
            MatchData::Collect(collection) => {
                for (key, mut vec) in collection {
                    if self.collection.contains_key(&key) {
                        self.collection.get_mut(&key).unwrap().append(&mut vec);
                    } else {
                        self.collection.insert(key, vec);
                    }
                }
            }
            MatchData::Match(key, mc) => {
                if self.collection.contains_key(&key) {
                    self.collection.get_mut(&key).unwrap().push(Rc::new(mc));
                } else {
                    self.collection.insert(key, vec!(Rc::new(mc)));
                }
            }
        }
    }

    pub fn get_collection(self) ->  HashMap<String, Vec<Rc<CaptureTree>>> {
        self.collection
    }
}