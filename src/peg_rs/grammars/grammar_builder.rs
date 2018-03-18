use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use peg_rs::grammars::grammar_nodes::*;
use peg_rs::grammars::grammar_nodes::production::ProductionNode;
use peg_rs::grammars::grammar::Grammar;

pub struct GrammarBuilder {
    root_prod: String,
    productions: HashMap<String, Production>,
}

impl GrammarBuilder {
    pub fn new(prod: Production) -> GrammarBuilder {
        let mut result = GrammarBuilder {
            root_prod: "".to_string(),
            productions: HashMap::new()
        };
        result.root_prod = prod.name.clone();
        result.productions.insert(prod.name.clone(), prod);
        result
    }

    pub fn add_prod(mut self, prod: Production) -> GrammarBuilder {
        self.productions.insert(prod.name.clone(), prod);
        self
    }

    pub fn build(self) -> Result<Grammar, String> {
        let mut productions : HashMap<String, Rc<RefCell<ProductionNode>>> = HashMap::new();
        match self.productions.get(&self.root_prod).unwrap().build(&mut productions, &self.productions) {
            Result::Ok(root) => Result::Ok(
                Grammar { root: root.clone() }
            ),
            Result::Err(err) => Result::Err(err),
        }
    }
}