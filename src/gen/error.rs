use std::{error::Error, fmt::Display};

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Clone)]
pub enum CompileError {}

impl Display for CompileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for CompileError {}
