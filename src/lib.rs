pub mod codegen;
pub mod parse;
pub mod tokenize;

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
    /// Unimplemented
    Unimplemented,
}
impl<T> From<Option<T>> for InterpreterError {
    fn from(_value: Option<T>) -> InterpreterError {
        InterpreterError::ExpectedValue
    }
}

pub fn compile(source: &str) -> Result<String, InterpreterError> {
    let tokens = tokenize::tokenize(source);
    let ast = parse::parse(&tokens?);
    codegen::codegen(&ast?)
}
