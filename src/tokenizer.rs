#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier,
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    LeftParen,
    RightParen,
    Semicolon,
    Comma,
}
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub index: usize,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut current: usize = 0;
    while let Some(c) = chars.get(current) {
        if c.is_alphabetic() {
            read_identifier(&chars, &mut current, &mut tokens);
        } else if c.is_digit(10) {
            read_number(&chars, &mut current, &mut tokens);
        } else {
            match c {
                '\'' | '"' => read_string(&chars, &mut current, &mut tokens, c),
                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        value: "(".to_owned(),
                        index: current,
                    });
                    current += 1;
                }
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        value: ")".to_owned(),
                        index: current,
                    });
                    current += 1;
                }
                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        value: ";".to_owned(),
                        index: current,
                    });
                    current += 1;
                }
                ',' => {
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        value: ",".to_owned(),
                        index: current,
                    });
                    current += 1;
                }
                _ => current += 1, // Just skip it if it's incorrect.
            }
        }
    }
    tokens
}
fn read_identifier(chars: &Vec<char>, current: &mut usize, tokens: &mut Vec<Token>) {
    let original_current = *current;
    let mut identifier = String::new();
    while let Some(c) = chars.get(*current) {
        if c.is_alphabetic() {
            identifier.push(*c);
            *current += 1;
        } else {
            break;
        }
    }
    tokens.push(Token {
        kind: TokenKind::Identifier,
        value: identifier,
        index: original_current,
    });
}
fn read_string(chars: &Vec<char>, current: &mut usize, tokens: &mut Vec<Token>, delimiter: &char) {
    let original_current = *current;
    let mut string = String::new();
    *current += 1; // Move forward from the first delimiter.
    while let Some(c) = chars.get(*current) {
        if c == delimiter {
            *current += 1;
            break;
        } else {
            string.push(*c);
            *current += 1;
        }
    }
    tokens.push(Token {
        kind: TokenKind::StringLiteral,
        value: string,
        index: original_current,
    });
}
fn read_number(chars: &Vec<char>, current: &mut usize, tokens: &mut Vec<Token>) {
    let original_current = *current;

    let mut kind = TokenKind::IntLiteral;

    let mut num = String::new();
    while let Some(c) = chars.get(*current) {
        if c.is_digit(10) {
            num.push(*c);
            *current += 1;
        } else if let Some(n) = chars.get(*current + 1) {
            if *c == 'f' {
                kind = TokenKind::FloatLiteral;
                *current += 1;
                break;
            } else if *c == 'i' {
                kind = TokenKind::IntLiteral;
                *current += 1;
                break;
            } else if *c == '.' && n.is_digit(10) {
                num.push(*c);
                num.push(*n);
                kind = TokenKind::FloatLiteral;
                *current += 2;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    tokens.push(Token {
        kind: kind,
        value: num,
        index: original_current,
    });
}