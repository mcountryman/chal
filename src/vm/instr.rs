#[derive(Debug, Clone)]
pub enum Instruction<'a> {
  Nop,

  LdNull,
  LdTrue,
  LdFalse,
  LdStr(&'a str),
  LdF64(f64),
  LdAddr(usize),
  LdImport(&'a str),

  LdLoc(u8),
  StLoc(u8),

  Jmp(isize),
  JmpEq(isize),
  JmpNEq(isize),
  JmpLt(isize),
  JmpGt(isize),
  JmpLtEq(isize),
  JmpGtEq(isize),

  Call,
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
