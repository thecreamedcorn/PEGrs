use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;

pub struct ChoiceNode {
    pub choices: Vec<Box<GrammarNode>>,
}

pub struct Choice {
    choices: Vec<Box<Buildable>>
}

impl Choice {
    pub fn new(choices: Vec<Box<Buildable>>) -> Choice {
        Choice { choices }
    }
}

impl GrammarNode for ChoiceNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        for boxed in &self.choices {
            match boxed.run(input) {
                ParseResult::Success(mut parse_data) => return ParseResult::Success(parse_data),
                ParseResult::Failure => ()
            }
        }
        ParseResult::Failure
    }
}

impl Buildable for Choice {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        let mut ch = ChoiceNode {
            choices: Vec::new(),
        };
        for buildable in &self.choices {
            match buildable.build(map, prods) {
                Result::Ok(gn) => {
                    ch.choices.push(gn)
                },
                Result::Err(err) => return Result::Err(err),
            }
        }
        Result::Ok(Box::new(ch))
    }
}

#[test]
fn test_choice() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new("TestStrLit",
            Box::new(Choice::new(vec!(
                Box::new(StrLit::new("test")),
                Box::new(StrLit::new("cool")),
            )))
        ))
        .build().unwrap();

    assert!(grammar.parse("test"));
    assert!(grammar.parse("cool"));
    assert!(!grammar.parse("bad"));
}