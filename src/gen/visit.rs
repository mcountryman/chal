use crate::ast::{
  Assign, BinaryOp, Call, Expr, Function, If, NumberLit, RefParam, RefVar, StringLit, UnaryOp,
};

pub trait Visitor {
  type Error;

  fn visit(&mut self, expr: &Expr<'_>) -> Result<(), Self::Error> {
    match expr {
      Expr::Noop(_) => Ok(()),

      Expr::String(expr) => self.visit_string(&expr),
      Expr::Number(expr) => self.visit_number(&expr),

      Expr::If(expr) => self.visit_if(&expr),
      Expr::Call(expr) => self.visit_call(&expr),
      Expr::Assign(expr) => self.visit_assign(&expr),
      Expr::Function(expr) => self.visit_function(&expr),
      Expr::UnaryOp(expr) => self.visit_unary(&expr),
      Expr::BinaryOp(expr) => self.visit_binary(&expr),

      Expr::RefVar(expr) => self.visit_var(&expr),
      Expr::RefParam(expr) => self.visit_param(&expr),

      Expr::Compound(expr) => {
        for expr in &expr.0 {
          self.visit(expr)?;
        }

        Ok(())
      }
    }
  }

  fn visit_string(&mut self, expr: &StringLit<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_number(&mut self, expr: &NumberLit) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_if(&mut self, expr: &If<'_>) -> Result<(), Self::Error> {
    self.visit(&expr.condition)?;
    self.visit(&expr.body)?;

    if let Some(expr) = &expr.fallthrough {
      self.visit(expr)?;
    }

    Ok(())
  }

  fn visit_call(&mut self, expr: &Call<'_>) -> Result<(), Self::Error> {
    if let Some(expr) = &expr.args {
      self.visit(expr)?;
    }

    Ok(())
  }

  fn visit_assign(&mut self, expr: &Assign<'_>) -> Result<(), Self::Error> {
    self.visit(&expr.expr)
  }

  fn visit_function(&mut self, expr: &Function<'_>) -> Result<(), Self::Error> {
    self.visit(&expr.body)
  }

  fn visit_unary(&mut self, expr: &UnaryOp<'_>) -> Result<(), Self::Error> {
    self.visit(&expr.expr)
  }

  fn visit_binary(&mut self, expr: &BinaryOp<'_>) -> Result<(), Self::Error> {
    self.visit(&expr.lhs)?;
    self.visit(&expr.rhs)?;

    Ok(())
  }

  fn visit_var(&mut self, expr: &RefVar<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_param(&mut self, expr: &RefParam<'_>) -> Result<(), Self::Error> {
    Ok(())
  }
}
