fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments were supplied!");
    } else {
        if args.len() != 2 {
            panic!("Not enough arguments were supplied!");
        }
    }

    println!("Are you sure that you want to permanently scramble this file? y/n");

    let mut final_check = String::new();
    std::io::stdin()
        .read_line(&mut final_check)
        .expect("Failed to read user input!");

    let yes_options: Vec<&str> = vec!["y", "yes"];

    if yes_options.contains(&final_check.trim()) {
        println!("SCRAM!");
    } else {
        println!("NOT SCRAMMING!");
    }
}

//Added this line to change the version lol
//IMPORTANT NOTE: Currently this only scrambles text files behond recognition <3
