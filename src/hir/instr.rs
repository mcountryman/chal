use super::{function::HirFn, local::LocalId};
use crate::util::uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(Uuid);

#[derive(Debug, Clone)]
pub enum HirInstruction<'buf> {
  Nop,

  LdStr,
  LdF64,

  LdLoc(LocalId),
  StLoc(LocalId),

  Jmp(Label),
  JmpEq(Label),
  JmpNEq(Label),
  JmpLt(Label),
  JmpGt(Label),
  JmpLtEq(Label),
  JmpGtEq(Label),
  Label(Label),

  CallUdf(HirFn<'buf>),
  CallBuiltIn(&'buf str),

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
