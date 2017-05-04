use ast::{Program, Operation, Variable, Value, VarVal, VarRef, Block};

use pom::char_class::alphanum;
use pom::combinator::*;
use pom::Parser;

use parser::util::*;

fn var<'a>() -> Combinator<impl Parser<'a, u8, Output=VarRef>> {
  is_a(alphanum)
    .repeat(1..)
    .convert(String::from_utf8)
    .map(|name| VarRef (name))
}

fn val<'a>() -> Combinator<impl Parser<'a, u8, Output=Value>> {
  str() | hex() | dec()
}

fn var_val<'a>() -> Combinator<impl Parser<'a, u8, Output=VarVal>> {
  var().map(|var| VarVal::Var(var)) | val().map(|val| VarVal::Val(val))
}

fn set<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"SET") * space_only() * var() - space_only() + var_val();
  tuple.map(|(var, val)| Operation::Set(var, val))
}

fn print<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"PRINT") * space_only() * var() - space_only() + num();
  tuple.map(|(var, val)| Operation::Print(var, val))
}

fn op<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  set() | print()
}

pub fn ops<'a>() -> Combinator<impl Parser<'a, u8, Output=Block>> {
  list(op(), newline()).map(|ops| Block(ops))
}
