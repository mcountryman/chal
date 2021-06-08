//! Scoped variable and local tracking.

use crate::util::uuid::Uuid;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(usize);

impl ScopeId {
  pub fn new(id: usize) -> Self {
    Self(id)
  }

  pub fn into_inner(self) -> usize {
    self.0
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Local(Uuid);

#[derive(Debug, Clone)]
pub struct Scope {
  pub vars: HashMap<String, Local>,
  pub params: HashMap<String, Local>,

  pub parent: Option<ScopeId>,
  pub children: Vec<ScopeId>,
}

impl Scope {
  pub fn new() -> Self {
    Self {
      vars: Default::default(),
      params: Default::default(),
      parent: None,
      children: Default::default(),
    }
  }
}

impl Default for Scope {
  fn default() -> Self {
    Self::new()
  }
}
