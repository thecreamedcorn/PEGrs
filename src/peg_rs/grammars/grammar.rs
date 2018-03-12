use std::rc::Rc;
use peg_rs::grammars::grammar_node::*;

pub struct Grammar {
    pub root: Rc<GrammarNode>
}

impl Grammar {
    pub fn parse<'a>(&self, string: &'a str ) -> bool {
        let result = self.root.run(&mut Parsable::new(string));
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