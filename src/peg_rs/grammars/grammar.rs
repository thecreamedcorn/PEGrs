use std::boxed::Box;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::matches::match_node::MatchNode;

pub struct Grammar {
    pub root: Box<GrammarNode>
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