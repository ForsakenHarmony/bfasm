use std::str::{self, FromStr};

use pom::char_class::{alphanum, digit};
use pom::combinator::*;
use pom::Parser;

use ast::{Program, Operation, Variable, Value, VarVal, Block};

use parser::util::*;

fn var_val<'a>() -> Combinator<impl Parser<'a, u8, Output=Variable>> {
  is_a(alphanum)
    .repeat(1..)
    .convert(String::from_utf8)
    .map(|name| Variable {
      name,
      val: Value::Var(0)
    })
}

fn var_arr<'a>() -> Combinator<impl Parser<'a, u8, Output=Variable>> {
  let pair = is_a(alphanum).repeat(1..).convert(String::from_utf8)
    + sym(b'[')
    * is_a(digit).repeat(1..).collect().convert(str::from_utf8).convert(u32::from_str)
    - sym(b']');
  pair.map(|(name, size)| Variable {
    name,
    val: Value::Arr(Vec::with_capacity(size as usize))
  })
}

fn var<'a>() -> Combinator<impl Parser<'a, u8, Output=Variable>> {
  var_arr() | var_val()
}

pub fn vars<'a>() -> Combinator<impl Parser<'a, u8, Output=Vec<Variable>>> {
  list(var(), newline())
}
