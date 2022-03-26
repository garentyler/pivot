use crate::{tokenize::Token, InterpreterError};

#[derive(Clone, PartialEq, Debug)]
pub enum AstPrimitive {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}
#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for AstPrimitive {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            AstPrimitive::Float(f) => format!("{}", f).hash(state),
            _ => self.hash(state),
        }
    }
}
impl std::fmt::Display for AstPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use AstPrimitive::*;
        match self {
            Integer(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Boolean(b) => write!(f, "{}", b),
            Null => write!(f, "null"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstNode {
    Primitive(AstPrimitive),
    Identifier(String),
    Negate {
        body: Box<AstNode>,
    },
    Add {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Subtract {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Multiply {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Divide {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Declare {
        identifier: String,
    },
    Assign {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    FunctionCall {
        identifier: String,
        arguments: Vec<AstNode>,
    },
    Program {
        statements: Vec<AstNode>,
    },
    Null,
}

pub fn parse(tokens: &[Token]) -> Result<AstNode, InterpreterError> {
    fn parse_function_call(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        let identifier;
        if let Token::Identifier(id) = tokens[index].clone() {
            identifier = id;
            index += 1;
        } else {
            return Err(InterpreterError::UnexpectedToken);
        }
        if !matches!(tokens[index], Token::Parenthesis { closing: false }) {
            return Ok((AstNode::Identifier(identifier), 1));
        } else {
            index += 1;
        }
        // Check if it closes right away.
        if matches!(tokens[index], Token::Parenthesis { closing: true }) {
            index += 1;
            return Ok((
                AstNode::FunctionCall {
                    identifier,
                    arguments: vec![],
                },
                index,
            ));
        }

        let mut arguments = vec![];
        if let Ok((argument, tokens_consumed)) = parse_expression(&tokens[index..]) {
            arguments.push(argument);
            index += tokens_consumed;
        }
        while index + 2 < tokens.len() {
            if tokens[index] != Token::Comma {
                break;
            }
            index += 1;
            if let Ok((argument, tokens_consumed)) = parse_expression(&tokens[index..]) {
                arguments.push(argument);
                index += tokens_consumed;
            } else {
                break;
            }
        }
        if matches!(tokens[index], Token::Parenthesis { closing: true }) {
            index += 1;
            Ok((
                AstNode::FunctionCall {
                    identifier,
                    arguments,
                },
                index,
            ))
        } else {
            Err(InterpreterError::UnexpectedToken)
        }
    }
    fn parse_primary_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        if tokens.is_empty() {
            Err(InterpreterError::UnexpectedEOF)
        } else if let Token::Integer(n) = &tokens[0] {
            Ok((AstNode::Primitive(AstPrimitive::Integer(*n)), 1))
        } else if let Token::Float(n) = &tokens[0] {
            Ok((AstNode::Primitive(AstPrimitive::Float(*n)), 1))
        } else if let Token::Boolean(n) = &tokens[0] {
            Ok((AstNode::Primitive(AstPrimitive::Boolean(*n)), 1))
        } else if let Token::String(s) = &tokens[0] {
            Ok((AstNode::Primitive(AstPrimitive::String(s.clone())), 1))
        } else if let Token::Identifier(_) = &tokens[0] {
            parse_function_call(tokens)
        } else if tokens[0] == Token::Keyword("let".to_owned()) {
            if tokens.len() < 2 {
                Err(InterpreterError::UnexpectedEOF)
            } else if let Token::Identifier(s) = &tokens[1] {
                Ok((
                    AstNode::Declare {
                        identifier: s.clone(),
                    },
                    2,
                ))
            } else {
                Err(InterpreterError::UnexpectedToken)
            }
        } else {
            Err(InterpreterError::UnexpectedToken)
        }
    }
    fn parse_grouped_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        // '('
        if !matches!(tokens[index], Token::Parenthesis { closing: false }) {
            return parse_primary_expression(tokens);
        } else {
            index += 1;
        }
        // expression of any kind
        let (value, tokens_consumed) = parse_expression(&tokens[index..])?;
        index += tokens_consumed;
        // ')'
        if !matches!(tokens[index], Token::Parenthesis { closing: true }) {
            return Err(InterpreterError::ParseError(
                "No closing parenthesis".to_owned(),
            ));
        } else {
            index += 1;
        }
        Ok((value, index))
    }
    fn parse_unary_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        if tokens[index] != Token::Minus && tokens[index] != Token::Bang {
            parse_grouped_expression(&tokens[index..])
        } else {
            let operation = tokens[index].clone();
            index += 1;
            let (body, tokens_consumed) = parse_unary_expression(&tokens[index..])?;
            index += tokens_consumed;
            Ok((
                match operation {
                    Token::Minus => AstNode::Negate {
                        body: Box::new(body),
                    },
                    Token::Bang => AstNode::Negate {
                        body: Box::new(body),
                    },
                    _ => return Err(InterpreterError::ParseError("Impossible".to_owned())),
                },
                index,
            ))
        }
    }
    fn parse_multiplication_expression(
        tokens: &[Token],
    ) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        let (mut value, tokens_consumed) = parse_unary_expression(&tokens[index..])?;
        index += tokens_consumed;
        while index < tokens.len() {
            if tokens[index] != Token::Star && tokens[index] != Token::Slash {
                break;
            }
            let operation = tokens[index].clone();
            index += 1;
            let (right, tokens_consumed) = parse_unary_expression(&tokens[index..])?;
            index += tokens_consumed;
            value = match operation {
                Token::Star => AstNode::Multiply {
                    left: Box::new(value),
                    right: Box::new(right),
                },
                Token::Slash => AstNode::Divide {
                    left: Box::new(value),
                    right: Box::new(right),
                },
                _ => return Err(InterpreterError::ParseError("Impossible".to_owned())),
            };
        }
        Ok((value, index))
    }
    fn parse_addition_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        let (mut value, tokens_consumed) = parse_multiplication_expression(&tokens[index..])?;
        index += tokens_consumed;
        while index < tokens.len() {
            if tokens[index] != Token::Plus && tokens[index] != Token::Minus {
                break;
            }
            let operation = tokens[index].clone();
            index += 1;
            let (right, tokens_consumed) = parse_multiplication_expression(&tokens[index..])?;
            index += tokens_consumed;
            value = match operation {
                Token::Plus => AstNode::Add {
                    left: Box::new(value),
                    right: Box::new(right),
                },
                Token::Minus => AstNode::Subtract {
                    left: Box::new(value),
                    right: Box::new(right),
                },
                _ => return Err(InterpreterError::ParseError("Impossible".to_owned())),
            };
        }
        Ok((value, index))
    }
    fn parse_assign_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        let mut index = 0;
        let (identifier, tokens_consumed) = parse_addition_expression(&tokens[index..])?;
        index += tokens_consumed;
        if index < tokens.len() && tokens[index] == Token::Equals {
            index += 1;
        } else {
            return Ok((identifier, index));
        }
        let (value, tokens_consumed) = parse_addition_expression(&tokens[index..])?;
        index += tokens_consumed;
        Ok((
            AstNode::Assign {
                left: Box::new(identifier),
                right: Box::new(value),
            },
            index,
        ))
    }
    fn parse_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        parse_assign_expression(tokens)
    }
    let mut statements = vec![];
    let mut index = 0;
    loop {
        if index >= tokens.len() {
            break;
        }
        let (statement, tokens_consumed) = parse_expression(&tokens[index..])?;
        statements.push(statement);
        index += tokens_consumed;
    }
    Ok(AstNode::Program { statements })
}
