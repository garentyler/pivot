fn main() {
    let ast = pivot::parse::parse(r"log(2)");
    println!("{}", ast);
}
