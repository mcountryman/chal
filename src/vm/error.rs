use std::{error::Error, fmt::Display};

pub type VmResult<T> = Result<T, VmError>;

#[derive(Debug, Clone)]
pub enum VmError {}

impl Display for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for VmError {}
