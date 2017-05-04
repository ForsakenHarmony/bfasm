use std::str::{self, FromStr};
use std::u32;

use pom::char_class::{hex_digit, alphanum, digit};
use pom::combinator::*;
use pom::Parser;

use ast::{Program, Operation, Variable, Value, VarVal, Block};

pub fn space_only<'a>() -> Combinator<impl Parser<'a, u8, Output=()>> {
  one_of(b" \t").repeat(1..).discard()
}

pub fn space<'a>() -> Combinator<impl Parser<'a, u8, Output=()>> {
  one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn newline<'a>() -> Combinator<impl Parser<'a, u8, Output=()>> {
  one_of(b" \t\r\n").repeat(1..).discard()
}

pub fn num<'a>() -> Combinator<impl Parser<'a, u8, Output=u32>> {
  is_a(digit).repeat(1..).collect()
             .convert(str::from_utf8).convert(u32::from_str)
}

pub fn dec<'a>() -> Combinator<impl Parser<'a, u8, Output=Value>> {
  num().map(|num| Value::Var(num as u8))
}

pub fn hex<'a>() -> Combinator<impl Parser<'a, u8, Output=Value>> {
  seq(b"0x") * is_a(hex_digit).repeat(1..).map(|digits| {
    let mut res = 0u32;
    
    // Do not parse more than 2 characters for a u8
    let mut parsed =
      if digits.len() > 2 {
        &digits[..2]
      } else {
        &digits[..]
      };
    
    for &e in parsed {
      let digit = e as char;
      let value = digit.to_digit(16).unwrap_or(0);
      res = value + (res << 4);
    }
    
    return Value::Var(res as u8);
  })
}

pub fn str<'a>() -> Combinator<impl Parser<'a, u8, Output=Value>> {
  let char_string = none_of(b"\\\"").repeat(1..).convert(String::from_utf8);
  let string = sym(b'"') * char_string.repeat(0..) - sym(b'"');
  string.map(|strings| strings.concat())
        .map(|str| Value::Arr(str.as_str().as_bytes().to_vec()))
}
