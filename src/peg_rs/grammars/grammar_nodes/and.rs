use peg_rs::grammars::grammar_node::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::buildable::*;
use peg_rs::grammars::grammar_nodes::not::Not;


pub struct And;

impl And {
    pub fn new(child: Box<Buildable>) -> Box<Buildable> {
        Not::new(Not::new(child))
    }
}

#[test]
fn test_and() {
    use peg_rs::grammars::grammar_nodes::*;
    use peg_rs::grammars::grammar_builder::GrammarBuilder;

    let grammar = GrammarBuilder::new(
        Production::new(
            "Prod",
            Union::new(vec!(
                StrLit::new("test"),
                And::new(
                    StrLit::new("no")
                ),
            ))
        ))
        .build().unwrap();

    assert!(!grammar.parse("testtest"));
    assert!(!grammar.parse("test"));
    assert!(grammar.parse("testno"));
}