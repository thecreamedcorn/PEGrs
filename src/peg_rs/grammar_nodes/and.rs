use peg_rs::grammar_nodes::not::Not;
use peg_rs::interfaces::buildable::Buildable;

pub struct And;

impl And {
    pub fn new(child: Box<Buildable>) -> Box<Buildable> {
        Not::new(Not::new(child))
    }
}

#[test]
fn test_and() {
    use ::*;

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