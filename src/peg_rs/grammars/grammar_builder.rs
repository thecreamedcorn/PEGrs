use std::collections::HashMap;
use std::boxed::Box;
use peg_rs::grammars::grammar_nodes::production::Production;
use peg_rs::grammars::grammar::Grammar;

pub struct GrammarBuilder {
    root_prod: String,
    productions: HashMap<String, Production>,
}

impl GrammarBuilder {
    pub fn new() -> GrammarBuilder {
        GrammarBuilder {
            root_prod: "".to_string(),
            productions: HashMap::new()
        }
    }

    pub fn add_prod(&mut self, prod: Production) {
        if self.productions.is_empty() {
            self.root_prod = prod.name.clone()
        }
        self.productions.insert(prod.name.clone(), prod);
    }

    pub fn build(self) -> Result<Grammar, String> {
        match self.productions.get(&self.root_prod).unwrap().build(&mut HashMap::new(), &self.productions) {
            Result::Ok(root) => Result::Ok(
                Grammar { root }
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}

#[test]
fn test_grammar_builder() {
    use peg_rs::grammars::grammar_nodes::str_lit::StrLit;
    use peg_rs::input::parsable::Parsable;

    let mut gb = GrammarBuilder::new();
    gb.add_prod(Production {
        name: "Prod1".to_string(),
        child: Box::new(StrLit {
            string: "test".to_string()
        }),
    });
    let grammar = gb.build().unwrap();
    assert!(grammar.parse("test"));
}