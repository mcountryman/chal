//! High-level intermediate representation.

use crate::util::uuid::Uuid;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Hir<'buf> {
  fns: Vec<Function<'buf>>,
  scopes: Vec<Scope<'buf>>,
}

#[derive(Debug, Clone)]
pub struct Scope<'buf> {
  body: Vec<Instruction<'buf>>,
  parent: Option<ScopeId>,
  children: Vec<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeId(Uuid);

#[derive(Debug, Clone)]
pub struct Function<'buf> {
  scope: Scope<'buf>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct LabelId(Uuid);

#[derive(Debug, Clone)]
pub enum FunctionRef<'buf> {
  BuiltIn(&'buf str),
  Function(Rc<Function<'buf>>),
}

#[derive(Debug, Clone)]
pub enum Instruction<'buf> {
  Nop,
  Label(LabelId),

  LdStr(&'buf str),
  LdF64(f64),
  LdImport(&'buf str),

  StVar(&'buf str),
  LdVar(&'buf str),
  DefVar(&'buf str),

  Jmp(LabelId),
  JmpEq(LabelId),
  JmpNEq(LabelId),
  JmpLt(LabelId),
  JmpGt(LabelId),
  JmpLtEq(LabelId),
  JmpGtEq(LabelId),

  Call(FunctionRef<'buf>),
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
