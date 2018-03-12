use std::boxed::Box;
use std::result::Result;
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
    pub fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Result<Rc<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(child) => Result::Ok(
                Rc::new(
                    ProductionNode {
                        name: self.name.clone(),
                        child,
                    }
                )
            ),
            err => err,
        }

    }
}

impl Buildable for ProductionRef {
    fn build(&self, map: &mut HashMap<String, Rc<GrammarNode>>, prods: &HashMap<String, Production>) -> Result<Rc<GrammarNode>, String> {
        if map.contains_key(&self.name) {
            Result::Ok(map.get(&self.name).unwrap().clone())
        } else {
            match prods.get(&self.name) {
                Option::Some(prod) => {
                    match prod.build(map, prods) {
                        Result::Ok(node) => {
                            map.insert(self.name.clone(), node.clone());
                            Result::Ok(node)
                        },
                        Result::Err(err) => Result::Err(err),
                    }
                },
                Option::None => {
                    Result::Err(format!("could not find production named '{}'", self.name))
                },
            }
        }
    }
}