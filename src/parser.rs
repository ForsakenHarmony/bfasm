use ast::{Program, Operation, Variable, Value, VarVal};
use nom::{IResult, ErrorKind, alpha, digit, hex_u32, float, be_u32};

use std::str;
use std::str::FromStr;
use std::u8;

fn new_var_val(name: String) -> Variable {
  Variable { name, val: Value::Var(0) }
}

fn new_var_arr(params: (String, u32)) -> Variable {
  Variable { name: params.0, val: Value::Arr(Vec::with_capacity(params.1 as usize)) }
}

fn to_int(str: &str) -> u32 {
  str.parse::<u32>().unwrap()
}

fn new_val_from_num(n: u32) -> Value {
  Value::Var(n as u8)
}

fn new_val_from_string(s: String) -> Value {
  Value::Arr(s.as_str().as_bytes().to_vec())
}

named!(string <String>, map!(map_res!(alpha, str::from_utf8), String::from));
named!(pub number <u32>, map!(map_res!(digit, str::from_utf8), to_int));

named!(var_val<Variable>, map!(string, new_var_val));
named!(var_arr<Variable>, complete!(map!(pair!(string, delimited!(tag!("["), number, tag!("]"))), new_var_arr)));
named!(var<Variable>, alt!(var_arr | var_val));

named!(pub val_dec<Value>, map!(number, new_val_from_num));
named!(pub val_hex<Value>, map!(do_parse!(tag!("0x") >> num: hex_u32 >> (num)), new_val_from_num));
named!(val_str<Value>, map!(delimited!(tag!("\""), string, tag!("\"")), new_val_from_string));
named!(pub val<Value>, alt!(val_dec | val_hex | val_str));

//named!(pub );

pub fn parse(i: String) {
  let lines = i.split(&['\r', '\n'][..])
               .filter(|l| !l.is_empty())
               .filter(|l| !l.starts_with("//"))
               .collect::<Vec<_>>();
  
  let mut at_ops = false;
  let mut program = Program::new();
  
  for l in lines {
    if l == "!" {
      at_ops = true;
      continue;
    }
    let line = l.as_bytes();
    if !at_ops {
      program.push_var(var(line).unwrap().1);
      continue;
    }
  }
  
  println!("{:?}", program);
}
