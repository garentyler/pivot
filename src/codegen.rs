use crate::{parse::AstNode, InterpreterError};

pub fn codegen(_ast: &AstNode) -> Result<String, InterpreterError> {
    Ok(String::new())
}
