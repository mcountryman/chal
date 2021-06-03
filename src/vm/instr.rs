#[derive(Debug, Clone)]
pub enum Instruction<'a> {
  Nop,

  LdStr(&'a str),
  LdF64(f64),

  Store(u8),

  JmpEq(usize),
  JmpNEq(usize),
  JmpLt(usize),
  JmpGt(usize),
  JmpLtEq(usize),
  JmpGtEq(usize),

  Call(usize),
  CallVirt(&'a str),
  Ret,

  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  BOr,
  BAnd,
  BLShift,
  BRShift,
}
