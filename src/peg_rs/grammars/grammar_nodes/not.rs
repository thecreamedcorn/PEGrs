use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::buildable::*;

struct NotNode {
    pub child: Box<GrammarNode>,
}

struct Not {
    child: Box<Buildable>,
}

impl Not {
    pub fn new(child: Box<Buildable>) -> Not {
        Not { child }
    }
}

impl GrammarNode for NotNode {
    fn run(&self, input: &mut Parsable) -> ParseResult {
        let begin = input.get_loc();
        match self.child.run(input) {
            ParseResult::Success(parse_data) => {
                input.goto_loc(begin);
                ParseResult::new_empty()
            },
            ParseResult::Failure => ParseResult::Failure,
        }
    }
}

impl Buildable for Not {
    fn build(&self, map: &mut HashMap<String, Rc<RefCell<ProductionNode>>>, prods: &HashMap<String, Production>) -> Result<Box<GrammarNode>, String> {
        match self.child.build(map, prods) {
            Result::Ok(gn) => NotNode {
                child: gn,
            },
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_not() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod",
            Box::new(Union::new(vec!(
                Box::new(StrLit::new("test")),
                Box::new(Not::new(
                    Box::new(StrLit::new("no"))
                )),
            )))
        ))
        .build().unwrap();

    assert!(grammar.parse("testtest"));
    assert!(grammar.parse("test"));
    assert!(!grammar.parse("testno"));
}