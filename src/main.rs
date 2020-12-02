fn main() {
    let src = "log(1, 2)";
    let ast = pivot::parse(src);
    println!("{}", ast);
}
