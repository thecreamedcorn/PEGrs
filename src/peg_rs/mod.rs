pub mod grammar_nodes;
pub mod utils;
mod interfaces;

#[test]
fn calculator_test() {
    use std::cell::RefCell;
    use std::rc::Rc;

    use ::*;

    let stack_orig: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));

    /*
    Add <- WS Mul (WS ('+' | '-') Add)?
    Mul <- Paren (WS ('*' ! '/') WS Mul)?
    Paren <- '(' Add WS ')' / WS Num
    Num <- '-'? [0-9]+ ('.'[0-9]*)? (('e' | 'E') (+|-)? [0-9]+)?
    WS <- [\n \t\b]
    */
    let grammar = GrammarBuilder::new(
        Production::new(
            "Add",
            Union::new(vec!(
                ProductionRef::new("WS"),
                ProductionRef::new("Mul"),
                ZeroOrOne::new(
                    SemAct::new(
                        Union::new(vec!(
                            ProductionRef::new("WS"),
                            Capture::new(
                                "op",
                                Choice::new(vec!(
                                    StrLit::new("+"),
                                    StrLit::new("-"),
                                ))
                            ),
                            ProductionRef::new("Add"),

                        )),
                        Rc::new({
                            let stack = stack_orig.clone();
                            move |ct: &CaptureTree| {
                                let n2 = stack.borrow_mut().pop().unwrap();
                                println!("popping {:?}", n2);
                                let n1 = stack.borrow_mut().pop().unwrap();
                                println!("popping {:?}", n1);

                                if ct.children.get("op").unwrap()[0].content.to_string() == "+" {
                                    stack.borrow_mut().push(n1 + n2);
                                    println!("pushing {:?}", n1 + n2);
                                } else {
                                    stack.borrow_mut().push(n1 - n2);
                                    println!("pushing {:?}", n1 - n2);
                                }
                            }
                        })
                    )
                ),
            ))
        ))
        .add_prod(
            Production::new(
                "Mul",
                Union::new(vec!(
                    ProductionRef::new("Paren"),
                    ZeroOrOne::new(
                        SemAct::new(
                            Union::new(vec!(
                                ProductionRef::new("WS"),
                                Capture::new(
                                    "op",
                                    Choice::new(vec!(
                                        StrLit::new("*"),
                                        StrLit::new("/"),
                                    ))
                                ),
                                ProductionRef::new("WS"),
                                ProductionRef::new("Mul")
                            )),
                            Rc::new({
                                let stack = stack_orig.clone();
                                move |ct: &CaptureTree| {
                                    let n2 = stack.borrow_mut().pop().unwrap();
                                    println!("popping {:?}", n2);
                                    let n1 = stack.borrow_mut().pop().unwrap();
                                    println!("popping {:?}", n1);

                                    if ct.children.get("op").unwrap()[0].content.to_string() == "*" {
                                        stack.borrow_mut().push(n1 * n2);
                                        println!("pushing {:?}", n1 * n2);
                                    } else {
                                        stack.borrow_mut().push(n1 / n2);
                                        println!("pushing {:?}", n1 / n2);
                                    }
                                }
                            })
                        )
                    ),
                ))
            )
        )
        .add_prod(
            Production::new(
                "Paren",
                Choice::new(vec!(
                    Union::new(vec!(
                        StrLit::new("("),
                        ProductionRef::new("Add"),
                        ProductionRef::new("WS"),
                        StrLit::new(")"),
                    )),
                    Union::new(vec!(
                        ProductionRef::new("WS"),
                        ProductionRef::new("Num"),
                    ))
                ))
            )
        )
        .add_prod(
            Production::new(
                "Num",
                SemAct::new(
                    Union::new(vec!(
                        ZeroOrOne::new(
                            StrLit::new("-")
                        ),
                        OneOrMore::new(
                            CharClass::new("0-9")
                        ),
                        ZeroOrOne::new(
                            Union::new(vec!(
                                StrLit::new("."),
                                OneOrMore::new(
                                    CharClass::new("0-9")
                                ),
                            ))
                        ),
                        ZeroOrOne::new(
                            Union::new(vec!(
                                Choice::new(vec!(
                                    StrLit::new("e"),
                                    StrLit::new("E"),
                                )),
                                ZeroOrOne::new(
                                    Choice::new(vec!(
                                        StrLit::new("+"),
                                        StrLit::new("-"),
                                    ))
                                ),
                                OneOrMore::new(
                                    CharClass::new("0-9")
                                ),
                            ))
                        ),
                    )),
                    Rc::new({
                        let stack = stack_orig.clone();
                        move |ct: &CaptureTree| {
                            stack.borrow_mut().push(ct.content.to_string().parse::<f64>().unwrap());
                            println!("pushing {:?}", ct.content.to_string().parse::<f64>().unwrap());
                        }
                    })
                )
            )
        )
        .add_prod(
            Production::new(
                "WS",
                ZeroOrMore::new(
                    CharClass::new(" \n\t")
                )
            )
        )
        .build().unwrap();

    grammar.parse("5 * (2 + 2) * 1");
    assert_eq!(stack_orig.borrow()[0], 20.0);
}