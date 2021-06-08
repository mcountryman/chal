//! High-level intermediate representation.

pub mod functions;
pub mod instr;
pub mod scope;

use std::collections::HashMap;

use self::{
  instr::Instruction,
  scope::{Local, Scope, ScopeId},
};
use crate::{
  ast::{
    Assign, BinaryOp, BinaryOperator, Call, Define, Expr, Function, If, NumberLit, RefParam,
    RefVar, StringLit, UnaryOp, UnaryOperator,
  },
  gen::visit::Visitor,
  hir::{functions::get_fns, instr::Label},
};

pub fn compile<'buf>(expr: &Expr<'buf>) -> Result<Vec<Instruction<'buf>>, ()> {
  let mut hir = Hir {
    scope: ScopeId::new(0),
    scopes: vec![Scope::new()],
    functions: get_fns(expr)?,
    instructions: Vec::new(),
  };

  hir.visit(expr)?;

  Ok(hir.instructions)
}

#[derive(Debug, Clone)]
pub struct Hir<'a> {
  scope: ScopeId,
  scopes: Vec<Scope>,
  functions: HashMap<String, Label>,
  instructions: Vec<Instruction<'a>>,
}

impl<'a> Hir<'a> {
  fn scope(&self) -> &Scope {
    self.scope_by(self.scope)
  }

  fn scope_mut(&mut self) -> &mut Scope {
    self.scope_by_mut(self.scope)
  }

  fn scope_by(&self, id: ScopeId) -> &Scope {
    &self.scopes[id.into_inner()]
  }

  fn scope_by_mut(&mut self, id: ScopeId) -> &mut Scope {
    &mut self.scopes[id.into_inner()]
  }

  fn pop_scope(&mut self) -> ScopeId {
    let scope = &self.scopes[self.scope.into_inner()];
    let parent = scope.parent.unwrap_or(self.scope);

    self.scope = parent;

    parent
  }

  fn push_scope(&mut self) -> ScopeId {
    let scope = Scope::new();
    let scope_id = ScopeId::new(self.scopes.len());

    self.scopes.push(scope);

    scope_id
  }

  fn push_var(&mut self, name: &'a str) -> Local {
    let scope = self.scope_mut();
    let local_id = Local::default();

    if scope.vars.insert(name.to_string(), local_id).is_some() {
      todo!("Duplicate variable `{}` defined", name);
    }

    local_id
  }

  fn push_param(&mut self, name: &'a str) -> Local {
    let scope = self.scope_mut();
    let local_id = Local::default();

    if scope.params.insert(name.to_string(), local_id).is_some() {
      todo!("Duplicate variable `{}` defined", name);
    }

    local_id
  }

  fn get_var_id(&self, name: &str) -> Option<Local> {
    let mut scope = self.scope();

    loop {
      if scope.vars.contains_key(name) {
        return scope.vars.get(name).copied();
      }

      match &scope.parent {
        Some(parent) => scope = self.scope_by(*parent),
        None => break,
      };
    }

    None
  }

  fn get_param_id(&self, name: &str) -> Option<Local> {
    let mut scope = self.scope();

    loop {
      if scope.params.contains_key(name) {
        return scope.params.get(name).copied();
      }

      match &scope.parent {
        Some(parent) => scope = self.scope_by(*parent),
        None => break,
      };
    }

    None
  }

  fn push(&mut self, instruction: Instruction<'a>) {
    self.instructions.push(instruction);
  }
}

impl<'buf> Visitor<'buf> for Hir<'buf> {
  type Error = ();

  fn visit_var(&mut self, var: &RefVar<'buf>) -> Result<(), Self::Error> {
    match self.get_var_id(var.0) {
      Some(local) => {
        self.push(Instruction::LdLoc(local));
        Ok(())
      }
      None => todo!("Undefined variable `{}`", var.0),
    }
  }

  fn visit_param(&mut self, param: &RefParam<'buf>) -> Result<(), Self::Error> {
    match self.get_param_id(param.0) {
      Some(local) => {
        self.push(Instruction::LdLoc(local));
        Ok(())
      }
      None => todo!("Undefined parameter `{}`", param.0),
    }
  }

  fn visit_assign(&mut self, expr: &Assign<'buf>) -> Result<(), Self::Error> {
    let local = self
      .get_var_id(expr.ident)
      .unwrap_or_else(|| panic!("Undefined parameter `{}`", expr.ident));

    self.visit(&expr.expr)?;
    self.push(Instruction::StLoc(local));

    Ok(())
  }

  fn visit_define(&mut self, expr: &Define<'buf>) -> Result<(), Self::Error> {
    let local = self.push_var(expr.ident);

    self.visit(&expr.expr)?;
    self.push(Instruction::StLoc(local));

    Ok(())
  }

  fn visit_number(&mut self, lit: &NumberLit) -> Result<(), Self::Error> {
    self.push(Instruction::LdF64(lit.0));

    Ok(())
  }

  fn visit_string(&mut self, lit: &StringLit<'buf>) -> Result<(), Self::Error> {
    self.push(Instruction::LdStr(lit.0.clone()));

    Ok(())
  }

  fn visit_unary(&mut self, expr: &UnaryOp<'buf>) -> Result<(), Self::Error> {
    match &expr.op {
      UnaryOperator::Neg => {
        self.visit(&expr.expr)?;
        self.push(Instruction::LdF64(-1.0));
        self.push(Instruction::Mul);
      }
      UnaryOperator::BNot => {
        self.visit(&expr.expr)?;
        self.push(Instruction::BNot);
      }
      _ => panic!("AddInc/SubInc unary expressions were a mistake."),
    }

    Ok(())
  }

  fn visit_binary(&mut self, expr: &BinaryOp<'buf>) -> Result<(), Self::Error> {
    self.visit(&expr.rhs)?;
    self.visit(&expr.lhs)?;
    self.push(match &expr.op {
      BinaryOperator::Add => Instruction::Add,
      BinaryOperator::Sub => Instruction::Sub,
      BinaryOperator::Mul => Instruction::Mul,
      BinaryOperator::Div => Instruction::Div,
      BinaryOperator::Mod => Instruction::Mod,
      BinaryOperator::Pow => Instruction::Pow,

      BinaryOperator::BOr => Instruction::BOr,
      BinaryOperator::BAnd => Instruction::BAnd,
      BinaryOperator::LShift => Instruction::LShift,
      BinaryOperator::RShift => Instruction::RShift,

      BinaryOperator::Eq => Instruction::Eq,
      BinaryOperator::NEq => Instruction::NEq,
      BinaryOperator::Lt => Instruction::Lt,
      BinaryOperator::LtEq => Instruction::LtEq,
      BinaryOperator::Gt => Instruction::Gt,
      BinaryOperator::GtEq => Instruction::GtEq,
    });

    Ok(())
  }

  fn visit_call(&mut self, expr: &Call<'buf>) -> Result<(), Self::Error> {
    match self.functions.get(expr.name).cloned() {
      Some(label) => self.push(Instruction::Call(label)),
      None => self.push(Instruction::CallF(expr.name)),
    }

    Ok(())
  }

  /// # Example
  ///
  /// Layout for less than on two numbers
  /// ```
  ///   LdF64(1.0)
  ///   LdF64(0.0)
  ///   JmpEq(body_label)
  ///     LdStr("Not equal")
  ///     CallF("println")
  ///     Jmp(end_label)
  ///   Label(body_label)
  ///     LdStr("Equal")
  ///     CallF("println")
  ///   Label(end_label)
  /// ```
  fn visit_if(&mut self, expr: &If<'buf>) -> Result<(), Self::Error> {
    let end_label = Label::default();
    let body_label = Label::default();

    match &expr.condition {
      Expr::BinaryOp(binary) if binary.op == BinaryOperator::Eq => {
        self.visit(&binary.rhs)?;
        self.visit(&binary.lhs)?;
        self.push(Instruction::JmpEq(body_label));
      }
      Expr::BinaryOp(binary) if binary.op == BinaryOperator::Lt => {
        self.visit(&binary.rhs)?;
        self.visit(&binary.lhs)?;
        self.push(Instruction::JmpLt(body_label));
      }
      expr => {
        self.visit(expr)?;
        self.push(Instruction::LdTrue);
        self.push(Instruction::JmpEq(body_label));
      }
    }

    if let Some(fallthrough) = &expr.fallthrough {
      self.push_scope();
      self.visit(fallthrough)?;
      self.pop_scope();
      self.push(Instruction::Jmp(end_label));
    }

    self.push(Instruction::Label(body_label));

    self.push_scope();
    self.visit(&expr.body)?;
    self.pop_scope();
    self.push(Instruction::Label(end_label));

    Ok(())
  }

  fn visit_function(&mut self, expr: &Function<'buf>) -> Result<(), Self::Error> {
    self.push_scope();

    let end_label = Label::default();
    let fn_label = self
      .functions
      .get(expr.name)
      .cloned()
      .expect("Function defined after HIR initial scan");

    self.push(Instruction::Jmp(end_label));
    self.push(Instruction::Label(fn_label));

    expr.params.iter().for_each(|param| {
      let local = self.push_param(param);
      self.push(Instruction::LdLoc(local));
    });

    self.visit(&expr.body)?;
    self.push(Instruction::Ret);
    self.push(Instruction::Label(end_label));

    self.pop_scope();

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::ast::Parser;

  #[test]
  fn test_compile() {
    let expr = Parser::new(include_str!("../../data/recursion.chal"))
      .parse()
      .unwrap();

    let instr = super::compile(&expr).unwrap();

    println!("{:?}", instr);
  }
}
