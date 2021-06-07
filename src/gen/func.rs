use crate::{
  ast::{BinaryOperator, Expr, Function, If},
  vm::instr::Instruction,
};
use std::{
  cell::RefCell,
  collections::HashMap,
  ops::Deref,
  rc::{Rc, Weak},
};

use super::{
  error::{CompileError, CompileResult},
  visit::Visitor,
};

#[derive(Clone)]
pub struct CompileFns<'buf>(Rc<RefCell<HashMap<String, CompileFn<'buf>>>>);

#[derive(Clone)]
struct CompileFnsWeak<'buf>(Weak<RefCell<HashMap<String, CompileFn<'buf>>>>);

impl<'buf> CompileFnsWeak<'buf> {
  pub fn add(&self, key: &str, value: CompileFn<'buf>) {
    self
      .0
      .upgrade()
      .expect("Dropped reference to `CompileFns`")
      .deref()
      .borrow_mut()
      .insert(key.to_string(), value);
  }
}

impl<'buf> CompileFns<'buf> {
  pub fn insert(&self, key: &str, value: CompileFn<'buf>) {
    self.0.deref().borrow_mut().insert(key.to_string(), value);
  }
}

pub struct CompileFn<'buf> {
  fns: CompileFnsWeak<'buf>,
  instr: Vec<Instruction<'buf>>,
  name: &'buf str,
  params: Vec<&'buf str>,
}

impl<'buf> CompileFn<'buf> {
  fn compile(fns: CompileFnsWeak<'buf>, expr: &Function<'buf>) -> CompileResult<Self> {
    Ok({
      let mut compiled = Self {
        fns,
        instr: Vec::new(),

        name: expr.name,
        params: expr.params.clone(),
      };

      compiled.visit(&expr.body)?;
      compiled
    })
  }
}

impl<'buf> Visitor<'buf> for CompileFn<'buf> {
  type Error = CompileError;

  fn visit_if(&mut self, expr: &If<'buf>) -> Result<(), Self::Error> {
    match &expr.condition {
      Expr::Noop(_) => {}
      Expr::UnaryOp(_) => {}
      Expr::BinaryOp(binary) => match binary.op {
        BinaryOperator::Lt => {}
        _ => todo!(),
      },
      Expr::RefVar(_) => {}
      Expr::RefParam(_) => {}
      Expr::Compound(_) => {}
      Expr::Number(value) => {}
      Expr::String(value) => {}
      _ => todo!(),
    };

    todo!()
  }

  fn visit_function(&mut self, expr: &Function<'buf>) -> CompileResult<()> {
    let compiled = Self::compile(self.fns.clone(), expr)?;
    self.fns.add(compiled.name, compiled);

    Ok(())
  }
}
