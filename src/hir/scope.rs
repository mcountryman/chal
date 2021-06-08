use super::local::Locals;
use crate::util::uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scopes<'buf> {
  scopes: HashMap<ScopeId, Scope<'buf>>,
}

impl<'buf> Scopes<'buf> {
  pub fn get(&self) -> &Scope<'buf> {
    todo!()
  }
}

#[derive(Debug, Clone)]
pub struct Scope<'buf> {
  locals: Locals<'buf>,
  parent: Option<ScopeId>,
  children: Vec<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(Uuid);
