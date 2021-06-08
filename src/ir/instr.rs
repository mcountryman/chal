use super::scope::Local;
use crate::util::uuid::Uuid;
use std::borrow::Cow;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label(Uuid);

#[derive(Debug, Clone)]
pub enum Instruction<'a> {
  Nop,

  LdNull,
  LdTrue,
  LdFalse,
  LdStr(Cow<'a, str>),
  LdF64(f64),
  LdLoc(Local),
  LdAddr(usize),
  LdImport(&'a str),

  StLoc(Local),

  Label(Label),

  Jmp(Label),
  JmpEq(Label),
  JmpNEq(Label),
  JmpLt(Label),
  JmpGt(Label),
  JmpLtEq(Label),
  JmpGtEq(Label),

  Call(Label),
  CallF(&'a str),
  Ret,

  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  Eq,
  NEq,
  Lt,
  Gt,
  LtEq,
  GtEq,

  BOr,
  BNot,
  BAnd,
  LShift,
  RShift,
}
