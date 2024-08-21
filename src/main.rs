use std::fs::File;
use std::io::Write;
use std::process::exit;

#[derive(Hash)]
struct Hashable {
    hash: String,
}

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
        exit(0);
    }

    let file_content = std::fs::read_to_string(&args[1]).unwrap();

    let mut hashed_file = Hashable {
        hash: String::new(),
    };

    let mut final_hash: Vec<String> = Vec::new();
    for i in file_content.split("\n") {
        hashed_file.hash = i.to_string();

        let hash = shim::make_hash(&hashed_file);
        println!("{}", hash);
        final_hash.push(hash.to_string());
    }
    std::fs::remove_file(&args[1]).expect("Failed to read file!");
    let mut new_file = File::create(&args[1]).expect("Failed to create file!");
    for i in final_hash {
        writeln!(new_file, "{}", i).expect("Failed to write to file!");
    }
}

//Added this line to change the version lol
//IMPORTANT NOTE: Currently this only scrambles text files behond recognition <3
