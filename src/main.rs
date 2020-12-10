fn main() {
    let src = r#"function add(left, right) { return 5; }"#;
    let ast = pivot::parse(src);
    println!("{}", ast);
}
