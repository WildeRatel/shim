fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments were supplied!");
    } else {
        panic!("Not enough arguments were supplied!");
    }
}
