#![feature(conservative_impl_trait)]
extern crate pom;

use std::env;
use std::fs::File;
use std::io::{self, Read};

mod parser;
mod ast;

fn main() {
  let args: Vec<String> = env::args().collect::<Vec<_>>()[1..].to_vec();

  let contents = read_file(&args[0]).unwrap();
  let file = String::from_utf8(contents.clone()).unwrap();

  println!("FILE\n\n{:?}\n\nEND", &file);
  println!("{:?}", parser::parse(file));
}

fn read_file(input: &str) -> io::Result<Vec<u8>> {
  let mut contents = vec![];

  File::open(input)?.read_to_end(&mut contents)?;

  Ok(contents)
}

