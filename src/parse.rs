#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Keyword(String),
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Equals,
    Semicolon,
    Quote(char),
    Parenthesis { closing: bool },
    Whitespace(String),
    Unknown,
}

#[derive(Debug)]
pub enum InterpreterError {
    /// Error parsing source
    ParseError(String),
    /// Unexpected token
    UnexpectedToken,
    /// Mismatched types
    MismatchedTypes,
    /// Type error
    TypeError,
    /// Unexpected EOF
    UnexpectedEOF,
    /// Expected value
    ExpectedValue,
}
impl<T> From<Option<T>> for InterpreterError {
    fn from(value: Option<T>) -> InterpreterError {
        InterpreterError::ExpectedValue
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, InterpreterError> {
    fn tokenize_number(chars: &[char]) -> Result<(Token, usize), ()> {
        let mut current_value = String::new();
        let mut chars_consumed = 0;
        for c in chars {
            if !c.is_digit(10) && *c != '.' {
                break;
            }
            current_value.push(*c);
            chars_consumed += 1;
        }
        if chars_consumed == 0 {
            Err(())
        } else if current_value.contains(".") {
            Ok((
                Token::Float(current_value.parse::<f64>().unwrap()),
                chars_consumed,
            ))
        } else {
            Ok((
                Token::Integer(current_value.parse::<i64>().unwrap()),
                chars_consumed,
            ))
        }
    }
    fn tokenize_identifier(chars: &[char]) -> Result<(Token, usize), ()> {
        let mut current_value = String::new();
        let mut chars_consumed = 0;
        if chars[chars_consumed].is_alphabetic() {
            current_value.push(chars[chars_consumed]);
        } else {
            return Err(());
        }
        chars_consumed += 1;
        while chars_consumed < chars.len()
            && (chars[chars_consumed].is_alphanumeric() || chars[chars_consumed] == '_')
        {
            current_value.push(chars[chars_consumed]);
            chars_consumed += 1;
        }
        match &current_value[..] {
            "true" => Ok((Token::Boolean(true), 4)),
            "false" => Ok((Token::Boolean(false), 5)),
            "let" => Ok((Token::Keyword(current_value), chars_consumed)),
            _ => Ok((Token::Identifier(current_value), chars_consumed)),
        }
    }
    fn tokenize_string(chars: &[char]) -> Result<(Token, usize), ()> {
        let start_quote;
        let mut current_value = String::new();
        let mut chars_consumed = 0;
        fn is_quote(c: char) -> bool {
            match c {
                '\'' | '"' | '`' => true,
                _ => false,
            }
        }
        if is_quote(chars[chars_consumed]) {
            start_quote = chars[chars_consumed];
        } else {
            return Err(());
        }
        chars_consumed += 1;
        while chars_consumed < chars.len() && chars[chars_consumed] != start_quote {
            current_value.push(chars[chars_consumed]);
            chars_consumed += 1;
        }
        chars_consumed += 1;
        Ok((Token::String(current_value), chars_consumed))
    }
    fn tokenize_whitespace(chars: &[char]) -> Result<(Token, usize), ()> {
        let mut current_value = String::new();
        let mut chars_consumed = 0;
        for c in chars {
            if !c.is_whitespace() {
                break;
            }
            chars_consumed += 1;
            current_value.push(*c);
        }
        if chars_consumed == 0 {
            Err(())
        } else {
            Ok((Token::Whitespace(current_value), chars_consumed))
        }
    }
    fn tokenize_operator(chars: &[char]) -> Result<(Token, usize), ()> {
        if chars.is_empty() {
            Err(())
        } else if chars[0] == '+' {
            Ok((Token::Plus, 1))
        } else if chars[0] == '-' {
            Ok((Token::Minus, 1))
        } else if chars[0] == '*' {
            Ok((Token::Star, 1))
        } else if chars[0] == '/' {
            Ok((Token::Slash, 1))
        } else if chars[0] == '!' {
            Ok((Token::Bang, 1))
        } else if chars[0] == '=' {
            Ok((Token::Equals, 1))
        } else {
            Err(())
        }
    }

    let source = source.chars().collect::<Vec<char>>();
    let mut tokens = vec![];
    let mut index = 0;
    while index < source.len() {
        if let Ok((_whitespace, chars_consumed)) = tokenize_whitespace(&source[index..]) {
            // Ignore whitespace
            index += chars_consumed;
        } else if let Ok((num, chars_consumed)) = tokenize_number(&source[index..]) {
            tokens.push(num);
            index += chars_consumed;
        } else if let Ok((num, chars_consumed)) = tokenize_string(&source[index..]) {
            tokens.push(num);
            index += chars_consumed;
        } else if let Ok((num, chars_consumed)) = tokenize_identifier(&source[index..]) {
            tokens.push(num);
            index += chars_consumed;
        } else if let Ok((operator, chars_consumed)) = tokenize_operator(&source[index..]) {
            tokens.push(operator);
            index += chars_consumed;
        } else if source[index] == ',' {
            tokens.push(Token::Comma);
            index += 1;
        } else if source[index] == ';' {
            tokens.push(Token::Semicolon);
            index += 1;
        } else if source[index] == '(' {
            tokens.push(Token::Parenthesis { closing: false });
            index += 1;
        } else if source[index] == ')' {
            tokens.push(Token::Parenthesis { closing: true });
            index += 1;
        } else {
            // Skip if things fail
            index += 1;
        }
    }
    Ok(tokens)
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstPrimitive {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Null,
}
impl std::hash::Hash for AstPrimitive {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            AstPrimitive::Float(f) => format!("{}", f).hash(state),
            _ => self.hash(state),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstNode {
    Primitive(AstPrimitive),
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
    Program {
        statements: Vec<AstNode>,
    },
    Null,
}

pub fn parse(tokens: &[Token]) -> Result<AstNode, InterpreterError> {
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
        } else if let Token::Identifier(s) = &tokens[0] {
            Ok((AstNode::Primitive(AstPrimitive::Identifier(s.clone())), 1))
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
            return parse_grouped_expression(&tokens[index..]);
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
        let statement = parse_expression(&tokens[index..]);
        if let Ok((statement, tokens_consumed)) = statement {
            statements.push(statement);
            index += tokens_consumed;
        } else {
            break;
        }
    }
    Ok(AstNode::Program { statements })
}
pub fn interpret(ast: &AstNode) -> Result<(), InterpreterError> {
    use std::{collections::HashMap, mem::discriminant};
    use AstNode::*;
    let mut vars: HashMap<String, Option<AstPrimitive>> = HashMap::new();
    if let Program { statements } = ast {
        for statement in statements {
            let _ = interpret_statement(statement, &mut vars)?;
        }
    }
    fn interpret_statement(
        ast: &AstNode,
        vars: &mut HashMap<String, Option<AstPrimitive>>,
    ) -> Result<Option<AstPrimitive>, InterpreterError> {
        match ast {
            Primitive(p) => {
                if let AstPrimitive::Identifier(id) = p {
                    if let Some(val) = vars.get(id) {
                        if let Some(val) = val {
                            Ok(Some(val.clone()))
                        } else {
                            Err(InterpreterError::ParseError(
                                "Variable used before definition".to_owned(),
                            ))
                        }
                    } else {
                        Err(InterpreterError::ParseError(
                            "Variable used before declaration".to_owned(),
                        ))
                    }
                } else {
                    Ok(Some(p.clone()))
                }
            }
            Negate { body } => {
                if let Some(AstPrimitive::Integer(body)) = interpret_statement(body, vars)? {
                    Ok(Some(AstPrimitive::Integer(body * -1)))
                } else if let Some(AstPrimitive::Boolean(body)) = interpret_statement(body, vars)? {
                    Ok(Some(AstPrimitive::Boolean(!body)))
                } else {
                    Err(InterpreterError::TypeError)
                }
            }
            Declare { identifier } => {
                vars.insert(identifier.clone(), None);
                Ok(None)
            }
            Assign { left, right } => {
                if let AstNode::Declare { identifier } = Box::leak(left.clone()) {
                    let _ = interpret(left)?;
                    let value = interpret_statement(right, vars)?;
                    vars.insert(identifier.clone(), value);
                    Ok(Some(vars.get(identifier).unwrap().clone().unwrap().clone()))
                } else if let AstNode::Primitive(AstPrimitive::Identifier(id)) =
                    Box::leak(left.clone())
                {
                    let id = id.clone();
                    let value = interpret_statement(right, vars)?;
                    vars.insert(id.clone(), value);
                    Ok(Some(vars.get(&id).unwrap().clone().unwrap().clone()))
                } else {
                    Err(InterpreterError::TypeError)
                }
            }
            Add { left, right } => {
                let left =
                    interpret_statement(left, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                let right =
                    interpret_statement(right, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                if discriminant(&left) != discriminant(&right) {
                    Err(InterpreterError::MismatchedTypes)
                } else {
                    if let AstPrimitive::Integer(left) = left {
                        if let AstPrimitive::Integer(right) = right {
                            return Ok(Some(AstPrimitive::Integer(left + right)));
                        }
                    }
                    if let AstPrimitive::Float(left) = left {
                        if let AstPrimitive::Float(right) = right {
                            return Ok(Some(AstPrimitive::Float(left + right)));
                        }
                    }
                    if let AstPrimitive::String(left) = left {
                        if let AstPrimitive::String(right) = right {
                            return Ok(Some(AstPrimitive::String(format!("{}{}", left, right))));
                        }
                    }
                    Err(InterpreterError::TypeError)
                }
            }
            Subtract { left, right } => {
                let left =
                    interpret_statement(left, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                let right =
                    interpret_statement(right, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                if discriminant(&left) != discriminant(&right) {
                    Err(InterpreterError::MismatchedTypes)
                } else {
                    if let AstPrimitive::Integer(left) = left {
                        if let AstPrimitive::Integer(right) = right {
                            return Ok(Some(AstPrimitive::Integer(left - right)));
                        }
                    }
                    if let AstPrimitive::Float(left) = left {
                        if let AstPrimitive::Float(right) = right {
                            return Ok(Some(AstPrimitive::Float(left - right)));
                        }
                    }
                    Err(InterpreterError::TypeError)
                }
            }
            Multiply { left, right } => {
                let left =
                    interpret_statement(left, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                let right =
                    interpret_statement(right, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                if discriminant(&left) != discriminant(&right) {
                    Err(InterpreterError::MismatchedTypes)
                } else {
                    if let AstPrimitive::Integer(left) = left {
                        if let AstPrimitive::Integer(right) = right {
                            return Ok(Some(AstPrimitive::Integer(left * right)));
                        }
                    }
                    if let AstPrimitive::Float(left) = left {
                        if let AstPrimitive::Float(right) = right {
                            return Ok(Some(AstPrimitive::Float(left * right)));
                        }
                    }
                    Err(InterpreterError::TypeError)
                }
            }
            Divide { left, right } => {
                let left =
                    interpret_statement(left, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                let right =
                    interpret_statement(right, vars)?.ok_or(InterpreterError::ExpectedValue)?;
                if discriminant(&left) != discriminant(&right) {
                    Err(InterpreterError::MismatchedTypes)
                } else {
                    if let AstPrimitive::Integer(left) = left {
                        if let AstPrimitive::Integer(right) = right {
                            return Ok(Some(AstPrimitive::Integer(left / right)));
                        }
                    }
                    if let AstPrimitive::Float(left) = left {
                        if let AstPrimitive::Float(right) = right {
                            return Ok(Some(AstPrimitive::Float(left / right)));
                        }
                    }
                    Err(InterpreterError::TypeError)
                }
            }
            _ => Err(InterpreterError::TypeError),
        }
    }
    Ok(())
}
pub fn run(source: &str) -> Result<(), InterpreterError> {
    println!("source: {:?}", source);
    let tokens = tokenize(source);
    println!("tokens: {:?}", tokens);
    let ast = parse(&tokens?);
    println!("ast: {:?}", ast);
    let value = interpret(&ast?);
    println!("value: {:?}", value);
    Ok(())
}
