use std::str::{self};

use pom::combinator::*;
use pom::Parser;

use ast::{Program};

use parser::util::*;
use parser::vars::vars;
use parser::ops::ops;

fn bfasm<'a>() -> Combinator<impl Parser<'a, u8, Output=Program>> {
  let res = space() * vars() - space() - sym(b'!') - space() + ops() - space() - end();
  res.map(|(vars, ops)| Program(vars, ops))
}

pub fn parse(i: String) {
  let str = i.as_bytes();
  println!("{:?}", str::from_utf8(str).unwrap());
  println!("{:?}", bfasm().parse(str));
}
