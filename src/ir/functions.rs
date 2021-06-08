use super::instr::Label;
use crate::ast::{Expr, Function, Visitor};
use std::collections::HashMap;

pub fn get_fns<'buf>(expr: &Expr<'buf>) -> Result<HashMap<String, Label>, ()> {
  let mut fns = Functions(Default::default());

  fns.visit(expr)?;

  Ok(fns.0)
}

struct Functions(HashMap<String, Label>);

impl<'buf> Visitor<'buf> for Functions {
  type Error = ();

  fn visit_function(&mut self, expr: &Function<'buf>) -> Result<(), Self::Error> {
    let label = Label::default();

    self.0.insert(expr.name.to_string(), label);

    self.visit(&expr.body)
  }
}
