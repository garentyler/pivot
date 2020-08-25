use super::tokenizer::{Token, TokenKind};

pub fn parse(mut tokens: Vec<Token>) -> Program {
    let mut stmts: Vec<Vec<Token>> = vec![];
    let mut current = 0;
    loop {
        if current >= tokens.len() {
            break;
        }
        // print!("{:?}", tokens[current]);
        if tokens[current].kind == TokenKind::Semicolon {
            stmts.push(tokens.drain(..=current).collect::<Vec<Token>>());
            current = 0;
        } else {
            current += 1;
        }
    }
    let mut statements = vec![];
    // for s in &stmts {
    //     for t in s {
    //         print!("{:?}", t.kind);
    //     }
    //     print!("\n");
    // }
    for s in stmts {
        statements.push(parse_statement(s));
    }
    statements
}

fn parse_statement(statement: Vec<Token>) -> Statement {
    if statement.len() == 1 {
        // Must just be a semicolon.
        return Statement::Nop;
    }
    let parse_function_call = |tokens: &Vec<Token>| -> Option<Statement> {
        // Check for <identifier>( ... );
        if tokens[0].kind != TokenKind::Identifier {
            return None;
        } else if tokens[1].kind != TokenKind::LeftParen {
            return None;
        } else if tokens[tokens.len() - 2].kind != TokenKind::RightParen {
            return None;
        } else if tokens[tokens.len() - 1].kind != TokenKind::Semicolon {
            return None;
        } else {
            let function_name = tokens[0].value.clone();
            let mut args = vec![];

            let mut current = 2;
            loop {
                args.push(parse_expression(tokens, &mut current));
                if tokens[current].kind == TokenKind::Comma {
                    current += 1;
                }
                if tokens[current].kind == TokenKind::RightParen {
                    break;
                }
            }

            Some(Statement::FunctionCall {
                name: function_name,
                arguments: args,
            })
        }
    };
    // The only form of statement.
    parse_function_call(&statement).expect("could not parse function call")
}
fn parse_expression(tokens: &Vec<Token>, current: &mut usize) -> Expression {
    if tokens[*current].kind == TokenKind::StringLiteral {
        let out = Expression::Literal(Literal::StringLiteral(tokens[*current].value.clone()));
        *current += 1;
        out
    } else if tokens[*current].kind == TokenKind::IntLiteral {
        let val = tokens[*current]
            .value
            .clone()
            .parse::<i32>()
            .expect("could not parse int literal");
        let out = Expression::Literal(Literal::IntLiteral(val));
        *current += 1;
        out
    } else if tokens[*current].kind == TokenKind::FloatLiteral {
        let val = tokens[*current]
            .value
            .clone()
            .parse::<f32>()
            .expect("could not parse float literal");
        let out = Expression::Literal(Literal::FloatLiteral(val));
        *current += 1;
        out
    } else {
        Expression::Null
    }
}

pub type Program = Vec<Statement>;
#[derive(Debug, PartialEq)]
pub enum Statement {
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Nop, // Equivalent to a C nop statement.
}
#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Null,
}
#[derive(Debug, PartialEq)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i32),
    FloatLiteral(f32),
}
