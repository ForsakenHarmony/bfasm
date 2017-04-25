#[derive(Debug, PartialEq)]
pub struct Variable {
  pub name: String,
  pub val: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
  Arr(Vec<u8>),
  Var(u8)
}

#[derive(Debug, PartialEq)]
pub enum VarVal {
  Var(Variable),
  Val(Value),
}

#[derive(Debug, PartialEq)]
pub enum Operation {
  While(Variable),
  If(Variable),
  Until(Variable),
  End,
  Set(Variable, Value),
  Copy(Variable, Variable),
  Add(Variable, VarVal),
  Sub(Variable, VarVal),
  Mul(Variable, VarVal),
  Div(Variable, VarVal),
  Read(Variable, u32),
  Print(Variable, u32),
  Bf(u32, &'static [u8])
}

#[derive(Debug, PartialEq)]
pub struct Program(Vec<Variable>, Vec<Operation>);

impl Program {
  pub fn new() -> Program {
    Program(Vec::new(), Vec::new())
  }
  
  pub fn push_var(&mut self, var: Variable) {
    self.0.push(var);
  }
  
  pub fn push_instruction(&mut self, op: Operation) {
    self.1.push(op);
  }
}
