fn main() {
    let src = "log(2 + 4, variable)";
    let ast = pivot::parse(src);
    println!("{}", ast);
}
