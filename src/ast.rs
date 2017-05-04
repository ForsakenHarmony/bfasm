#[derive(Debug, PartialEq)]
pub struct Variable {
  pub name: String,
  pub val: Value,
}

#[derive(Debug, PartialEq)]
pub struct VarRef(pub String);

#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Operation>);

#[derive(Debug, PartialEq)]
pub enum Value {
  Arr(Vec<u8>),
  Var(u8)
}

#[derive(Debug, PartialEq)]
pub enum VarVal {
  Var(VarRef),
  Val(Value),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
  While(VarRef, Block),
  Until(VarRef, Block),
  If(VarRef, Block),
  IfNot(VarRef, Block),
  
  Set(VarRef, VarVal),
  Copy(VarRef, VarRef),
  Add(VarRef, VarVal),
  Sub(VarRef, VarVal),
  Mul(VarRef, VarVal),
  Div(VarRef, VarVal),
  
  Read(VarRef, u32),
  Print(VarRef, u32),
  
  Bf(u32, &'static [u8])
}

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Variable>, pub Block);
