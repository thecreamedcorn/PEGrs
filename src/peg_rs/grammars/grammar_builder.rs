use std::collections::HashMap;
use std::boxed::Box;
use peg_rs::grammars::grammar_nodes::*;
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

    pub fn add_prod(mut self, prod: Production) -> GrammarBuilder{
        if self.productions.is_empty() {
            self.root_prod = prod.name.clone()
        }
        self.productions.insert(prod.name.clone(), prod);
        self
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