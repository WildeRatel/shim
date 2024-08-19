fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments were supplied!");
    } else {
        if args.len() != 2 {
            panic!("Not enough arguments were supplied!");
        }
    }

    if args[1][args[1].len() - 5..args[1].len()].contains(".txt") != true {
        panic!("Not a text file!");
    }
}

//Added this line to change the version lol
