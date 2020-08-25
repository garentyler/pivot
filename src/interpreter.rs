use super::parser::*;

pub fn interpret(ast: Program) {
    for stmt in ast {
        use Statement::*;
        match stmt {
            FunctionCall { name, arguments } => {
                let name: &str = &name;
                // println!("{} called!", name);
                match name {
                    "log" => {
                        let mut args: Vec<String> = vec![];
                        for a in arguments {
                            match a {
                                Expression::Literal(literal) => match literal {
                                    Literal::StringLiteral(s) => args.push(s),
                                    Literal::IntLiteral(i) => args.push(format!("{}i", i)),
                                    Literal::FloatLiteral(f) => {
                                        if f.fract() == 0.0 {
                                            args.push(format!("{}.0f", f));
                                        } else {
                                            args.push(format!("{}f", f));
                                        }
                                    }
                                    Literal::BooleanLiteral(b) => args.push(format!("{}", b)),
                                },
                                Expression::Null => {
                                    args.push("null".to_owned());
                                }
                            }
                        }
                        println!("{}", args.join(", "));
                    }
                    _ => {}
                }
            }
            Nop => {
                // println!("No-op!");
            }
        }
    }
}
