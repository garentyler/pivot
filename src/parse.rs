#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Quote(char),
    Parenthesis { closing: bool },
    Whitespace(String),
    Unknown,
}

#[derive(Debug)]
pub enum InterpreterError {
    /// Error parsing source
    ParseError(String),
    /// Unexpected EOF
    UnexpectedEOF,
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
            Ok((Token::Float(current_value.parse::<f64>().unwrap()), chars_consumed))
        } else {
            Ok((Token::Integer(current_value.parse::<i64>().unwrap()), chars_consumed))
        }
    }
    fn tokenize_bool(chars: &[char]) -> Result<(Token, usize), ()> {
        if chars.len() >= 5 && chars[0..5] == ['f', 'a', 'l', 's', 'e'] {
            Ok((Token::Boolean(false), 5))
        } else if chars.len() >= 4 && chars[0..4] == ['t', 'r', 'u', 'e'] {
            Ok((Token::Boolean(true), 4))
        } else {
            Err(())
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
        } else if let Ok((num, chars_consumed)) = tokenize_bool(&source[index..]) {
            tokens.push(num);
            index += chars_consumed;
        } else if let Ok((operator, chars_consumed)) = tokenize_operator(&source[index..]) {
            tokens.push(operator);
            index += chars_consumed;
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstNode {
    Primitive(AstPrimitive),
    Negate { body: Box<AstNode> },
    Add { left: Box<AstNode>, right: Box<AstNode> },
    Subtract { left: Box<AstNode>, right: Box<AstNode> },
    Multiply { left: Box<AstNode>, right: Box<AstNode> },
    Divide { left: Box<AstNode>, right: Box<AstNode> },
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
        } else {
            Err(InterpreterError::ParseError("Expected literal".to_owned()))
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
            return Err(InterpreterError::ParseError("No closing parenthesis".to_owned()));
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
                    Token::Minus => AstNode::Negate { body: Box::new(body) },
                    Token::Bang => AstNode::Negate { body: Box::new(body) },
                    _ => return Err(InterpreterError::ParseError("Impossible".to_owned())),
                },
                index
            ))
        }
    }
    fn parse_multiplication_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
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
    fn parse_expression(tokens: &[Token]) -> Result<(AstNode, usize), InterpreterError> {
        parse_addition_expression(tokens)
    }
    let (ast, _) = parse_expression(tokens)?;
    Ok(ast)
}
pub fn evaluate(ast: &AstNode) -> Result<AstPrimitive, InterpreterError> {
    use std::mem::discriminant;
    use AstNode::*;
    match ast {
        Primitive(p) => Ok(p.clone()),
        Negate { body } => {
            if let AstPrimitive::Integer(body) = evaluate(body)? {
                Ok(AstPrimitive::Integer(body * -1))
            } else if let AstPrimitive::Boolean(body) = evaluate(body)? {
                Ok(AstPrimitive::Boolean(!body))
            }else {
                Err(InterpreterError::ParseError("Can only negate integers and bools".to_owned()))
            }
        },
        Add { left, right } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            if discriminant(&left) != discriminant(&right) {
                Err(InterpreterError::ParseError("Mismatched types".to_owned()))
            } else {
                if let AstPrimitive::Integer(left) = left {
                    if let AstPrimitive::Integer(right) = right {
                       return Ok(AstPrimitive::Integer(left + right));
                    }
                }
                if let AstPrimitive::Float(left) = left {
                    if let AstPrimitive::Float(right) = right {
                       return Ok(AstPrimitive::Float(left + right));
                    }
                }
                if let AstPrimitive::String(left) = left {
                    if let AstPrimitive::String(right) = right {
                       return Ok(AstPrimitive::String(format!("{}{}", left, right)));
                    }
                }
                Err(InterpreterError::ParseError("Can only add integers, strings, and floats".to_owned()))
            }
        }
        Subtract { left, right } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            if discriminant(&left) != discriminant(&right) {
                Err(InterpreterError::ParseError("Mismatched types".to_owned()))
            } else {
                if let AstPrimitive::Integer(left) = left {
                    if let AstPrimitive::Integer(right) = right {
                       return Ok(AstPrimitive::Integer(left - right));
                    }
                }
                if let AstPrimitive::Float(left) = left {
                    if let AstPrimitive::Float(right) = right {
                       return Ok(AstPrimitive::Float(left - right));
                    }
                }
                Err(InterpreterError::ParseError("Can only subtract integers and floats".to_owned()))
            }
        }
        Multiply { left, right } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            if discriminant(&left) != discriminant(&right) {
                Err(InterpreterError::ParseError("Mismatched types".to_owned()))
            } else {
                if let AstPrimitive::Integer(left) = left {
                    if let AstPrimitive::Integer(right) = right {
                       return Ok(AstPrimitive::Integer(left * right));
                    }
                }
                if let AstPrimitive::Float(left) = left {
                    if let AstPrimitive::Float(right) = right {
                       return Ok(AstPrimitive::Float(left * right));
                    }
                }
                Err(InterpreterError::ParseError("Can only multiply integers and floats".to_owned()))
            }
        }
        Divide { left, right } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            if discriminant(&left) != discriminant(&right) {
                Err(InterpreterError::ParseError("Mismatched types".to_owned()))
            } else {
                if let AstPrimitive::Integer(left) = left {
                    if let AstPrimitive::Integer(right) = right {
                       return Ok(AstPrimitive::Integer(left / right));
                    }
                }
                if let AstPrimitive::Float(left) = left {
                    if let AstPrimitive::Float(right) = right {
                       return Ok(AstPrimitive::Float(left / right));
                    }
                }
                Err(InterpreterError::ParseError("Can only divide integers and floats".to_owned()))
            }
        }
        Null => Err(InterpreterError::ParseError("Cannot evaluate null".to_owned())),
    }
}
pub fn interpret(source: &str) -> Result<(), InterpreterError> {
    println!("source: {:?}", source);
    let tokens = tokenize(source);
    println!("tokens: {:?}", tokens);
    let ast = parse(&tokens?);
    println!("ast: {:?}", ast);
    let value = evaluate(&ast?);
    println!("value: {:?}", value);
    Ok(())
}