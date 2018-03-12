use std::boxed::Box;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::buildable::Buildable;

struct ProductionNode {
    pub name: String,
    pub child: Rc<GrammarNode>
}

pub struct Production {
    pub name: String,
    pub child: Box<Buildable>
}

pub struct ProductionRef {
    pub name: String
}

impl GrammarNode for ProductionNode {
    fn run<'a>(&self, input: &mut Parsable<'a>) -> ParseResult<'a> {
        let result = self.child.run(input);
        match result {
            ParseResult::SUCCESS(parse_data) => {
                ParseResult::SUCCESS(
                    ParseData {
                        match_data: MatchData::COLLECT(HashMap::new()),
                        call_list: parse_data.call_list
                    }
                )
            }
            ParseResult::FAILURE => ParseResult::FAILURE
        }
    }
}

impl Production {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>) -> Rc<GrammarNode> {

    }
}

impl Buildable for ProductionRef {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Rc<GrammarNode> {
        if prods.contains_key(self.name) {
            prods.get(self.name).unwrap()
        } else {
            let node = self.
        }
    }
}