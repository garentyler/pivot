pub mod interpreter;
pub mod parser;
pub mod tokenizer;

pub fn interpret(source: &str) {
    let tokens = tokenizer::tokenize(source);
    // println!("{:#?}", tokens);
    let ast = parser::parse(tokens);
    // println!("{:#?}", ast);
    interpreter::interpret(ast);
}
pub fn interpret_file(filename: &str) {
    let src = std::fs::read_to_string(filename).unwrap();
    interpret(&src);
}
