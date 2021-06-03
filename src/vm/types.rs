use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value<'script> {
  Null,
  Number(f64),
  String(&'script str),
}

impl Default for Value<'_> {
  fn default() -> Self {
    Value::Null
  }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Step {
  Next,
  Jmp(usize),
}
