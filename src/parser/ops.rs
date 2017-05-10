use ast::{Operation, Value, VarVal, VarRef, Block};

use pom::char_class::alphanum;
use pom::combinator::*;
use pom::Parser;

use parser::util::*;

fn var<'a>() -> Combinator<impl Parser<'a, u8, Output=VarRef>> {
  is_a(alphanum)
    .repeat(1..)
    .convert(String::from_utf8)
    .map(|name| VarRef(name))
}

fn val<'a>() -> Combinator<impl Parser<'a, u8, Output=Value>> {
  str() | hex() | dec()
}

fn var_val<'a>() -> Combinator<impl Parser<'a, u8, Output=VarVal>> {
  var().map(|var| VarVal::Var(var)) | val().map(|val| VarVal::Val(val))
}

fn if_<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"IF") * space_only() * var();

  let block = tuple + space() * ops() - space() - seq(b"END");

  block.map(|(var, block)| Operation::If(var, block))
}

//fn ifnot<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
//  let tuple = seq(b"IFNOT") * space_only() * var();
//
//  let block = tuple + space() * ops() - space() - seq(b"END");
//
//  block.map(|(var, block)| Operation::IfNot(var, block))
//}
//
//fn while_<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
//  let tuple = seq(b"WHILE") * space_only() * var();
//
//  let block = tuple + space() * ops() - space() - seq(b"END");
//
//  block.map(|(var, block)| Operation::While(var, block))
//}
//
//fn until<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
//  let tuple = seq(b"Until") * space_only() * var();
//
//  let block = tuple + space() * ops() - space() - seq(b"END");
//
//  block.map(|(var, block)| Operation::Until(var, block))
//}

fn set<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"SET") * space_only() * var() - space_only() + val();
  tuple.map(|(var, val)| Operation::Set(var, val))
}

fn cpy<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"CPY") * space_only() * var() - space_only() + var();
  tuple.map(|(var, val)| Operation::Copy(var, val))
}

fn add<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"ADD") * space_only() * var() - space_only() + var_val();
  tuple.map(|(var, val)| Operation::Add(var, val))
}

fn sub<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"SUB") * space_only() * var() - space_only() + var_val();
  tuple.map(|(var, val)| Operation::Sub(var, val))
}

fn mul<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"MUL") * space_only() * var() - space_only() + var_val();
  tuple.map(|(var, val)| Operation::Mul(var, val))
}

fn div<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"DIV") * space_only() * var() - space_only() + var_val();
  tuple.map(|(var, val)| Operation::Div(var, val))
}

fn read<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"READ") * space_only() * var() - space_only() + num();
  tuple.map(|(var, val)| Operation::Read(var, val))
}

fn print<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"PRINT") * space_only() * var() - space_only() + num();
  tuple.map(|(var, val)| Operation::Print(var, val))
}

fn bf<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  let tuple = seq(b"BF") * space_only() * num() - space_only() + one_of(b"<>+-.,[]").repeat(1..);
  tuple.map(|(var, val)| Operation::Bf(var, val))
}

fn op<'a>() -> Combinator<impl Parser<'a, u8, Output=Operation>> {
  if_() | /* ifnot() | while_() | until() | */ set() | cpy() | add() | sub() | mul() | div() | read() | print() | bf()
}

pub fn ops<'a>() -> Combinator<impl Parser<'a, u8, Output=Block>> {
  list(op(), newline()).map(|ops| Block(ops))
}
