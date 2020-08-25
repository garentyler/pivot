fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: pivot <filename>");
    } else {
        pivot::interpret_file(&args[1]);
    }
}
