use std::collections::HashMap;
use peg_rs::grammars::grammar_nodes::production::Production;
use peg_rs::grammars::grammar::Grammar;

struct GrammarBuilder {
    root_prod: String,
    productions: HashMap<String, Production>
}

impl GrammarBuilder {
    fn new() -> GrammarBuilder {
        GrammarBuilder {
            root_prod: "",
            productions: HashMap::new()
        }
    }

    fn addProd(&mut self, name: String, prod: Production) {
        if self.productions.is_empty() {
            self.root_prod = name
        }
        self.productions.insert(name, prod)
    }

    fn build(self) -> Grammar {
        unimplemented!()
    }
}