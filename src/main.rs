fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
    } else {
        panic!("Not enough arguments were provided!");
    }
}
