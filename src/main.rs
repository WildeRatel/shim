fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        println!("Arg 1: {}\n Arg 2: {}", args[1], args[2]);
    } else {
        if args.len() > 3 {
            panic!("Too many arguments were supplied!");
        } else {
            panic!("Not enough arguments were supplied!");
        }
    }
}

//Hello arno!
