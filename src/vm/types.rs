use super::BuiltInRc;
use crate::ir::instr::Label;
use std::{borrow::Cow, fmt::Debug, rc::Rc};

#[derive(Clone)]
pub enum Value {
  Null,
  Addr(usize),
  Bool(bool),
  Number(f64),
  String(Rc<String>),
  BuiltIn(BuiltInRc),
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Null => write!(f, "Value::Null"),
      Self::Addr(addr) => write!(f, "Value::Addr({})", addr),
      Self::Bool(value) => write!(f, "Value::Bool({})", value),
      Self::Number(value) => write!(f, "Value::Number({})", value),
      Self::String(value) => write!(f, "Value::String({})", value),
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
    Value::String(Rc::new(value.to_string()))
  }
}

impl<'a> From<Cow<'a, str>> for Value {
  fn from(value: Cow<'a, str>) -> Self {
    Value::String(Rc::new(value.to_string()))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Step {
  Next,
  Jmp(Label),
  JmpAddr(usize),
}
