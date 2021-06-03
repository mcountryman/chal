//! Byte code generation from AST.

pub mod error;
pub mod visit;

use self::{
  error::{CompileError, CompileResult},
  visit::Visitor,
};
use crate::{
  ast::{Expr, RefVar},
  vm::instr::Instruction,
};

pub struct Compiler {
  instructions: Vec<Instruction>,
}

impl Visitor for Compiler {
  type Error = CompileError;

  fn visit_var(&mut self, expr: &RefVar<'_>) -> Result<(), Self::Error> {
    todo!()
  }
}

pub fn compile(ast: &Expr<'_>) -> CompileResult<Vec<Instruction>> {
  todo!()
}
