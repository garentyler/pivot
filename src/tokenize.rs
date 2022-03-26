use crate::InterpreterError;

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
        } else if current_value.contains('.') {
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
        let mut current_value = String::new();
        let mut chars_consumed = 0;
        fn is_quote(c: char) -> bool {
            matches!(c, '\'' | '"' | '`')
        }
        let start_quote = if is_quote(chars[chars_consumed]) {
            chars[chars_consumed]
        } else {
            return Err(());
        };
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
