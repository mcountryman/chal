use super::{error::VmResult, BuiltInRc};
use crate::ir::instr::Label;
use std::{
  borrow::{Borrow, Cow},
  cell::RefCell,
  fmt::{Debug, Display},
  ops::Deref,
  rc::Rc,
};

#[derive(Clone)]
pub enum Value {
  Null,
  Addr(usize),
  Bool(bool),
  Number(f64),
  String(Rc<RefCell<String>>),
  BuiltIn(BuiltInRc),
}

impl Value {
  pub fn as_string(&self) -> VmResult<Rc<RefCell<String>>> {
    match &self {
      Self::String(value) => Ok(value.clone()),
      Self::Number(value) => Ok(Rc::new(RefCell::new(value.to_string()))),
      _ => todo!("Bad type"),
    }
  }

  pub fn as_f64(&self) -> VmResult<f64> {
    match &self {
      Self::Number(value) => Ok(*value),
      _ => todo!("Bad type"),
    }
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Null => write!(f, "Value::Null"),
      Self::Addr(addr) => write!(f, "Value::Addr({})", addr),
      Self::Bool(value) => write!(f, "Value::Bool({})", value),
      Self::Number(value) => write!(f, "Value::Number({})", value),
      Self::String(value) => write!(f, "Value::String({})", value.deref().borrow()),
      Self::BuiltIn(_) => write!(f, "Value::Null"),
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Self::Null => matches!(other, Self::Null),
      Self::Addr(addr) => matches!(other, Self::Addr(other) if addr == other),
      Self::Bool(value) => matches!(other, Self::Bool(other) if value == other),
      Self::Number(value) => matches!(other, Self::Number(other) if value == other),
      Self::String(value) => matches!(other, Self::String(other) if value == other),
      Self::BuiltIn(_) => false,
    }
  }
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match self {
      Self::Null => None,
      Self::Addr(_) => None,
      Self::Bool(_) => None,
      Self::Number(value) => match other {
        Self::Number(other) => value.partial_cmp(other),
        _ => None,
      },
      Self::String(_) => None,
      Self::BuiltIn(_) => None,
    }
  }
}

impl Default for Value {
  fn default() -> Self {
    Value::Null
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Null => write!(f, "null"),
      Self::Addr(_) => todo!(),
      Self::BuiltIn(_) => todo!(),
      Self::Bool(value) => write!(f, "{}", value),
      Self::Number(value) => write!(f, "{}", value),
      Self::String(value) => write!(f, "{}", value.deref().borrow()),
    }
  }
}

impl From<usize> for Value {
  fn from(value: usize) -> Self {
    Value::Addr(value)
  }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Value::Bool(value)
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Value::Number(value)
  }
}

impl From<&str> for Value {
  fn from(value: &str) -> Self {
    value.to_string().into()
  }
}

impl<'a> From<Cow<'a, str>> for Value {
  fn from(value: Cow<'a, str>) -> Self {
    value.to_string().into()
  }
}

impl<'a> From<String> for Value {
  fn from(value: String) -> Self {
    Rc::new(RefCell::new(value)).into()
  }
}

impl<'a> From<Rc<RefCell<String>>> for Value {
  fn from(value: Rc<RefCell<String>>) -> Self {
    Self::String(value)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Step {
  Next,
  Jmp(Label),
  JmpAddr(usize),
}
