use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>()[1..].to_vec();

    println!("I got {:?} arguments: {:?}.", args.len(), args);

    let file = File::open(&args[0]).expect("Could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Could not read file");

    println!("FILE\n\n{}\n\nEND", contents);
}
