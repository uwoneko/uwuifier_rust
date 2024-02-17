use std::{env, fs};
use uwuifier_rust::uwuify;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: uwuifier_rust input.txt output.txt");
        return;
    }

    let content = fs::read_to_string(&args[1]).expect("could not read file");

    let uwuified_content = uwuify(&content);

    fs::write(&args[2], uwuified_content).expect("could not write to file");
}
