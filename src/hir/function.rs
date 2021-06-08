use super::scope::Scope;
use crate::{ast::If, gen::visit::Visitor, util::uuid::Uuid};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct HirFns<'buf>(Rc<RefCell<HirFnsImp<'buf>>>);

#[derive(Debug, Clone)]
pub struct HirFnsImp<'buf> {
  fns: HashMap<HirFnId, HirFn<'buf>>,
  fns_by_name: HashMap<&'buf str, HirFnId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirFnId(Uuid);

#[derive(Debug, Clone)]
pub struct HirFn<'buf> {
  scope: Scope<'buf>,
  current: Scope<'buf>,
}

impl<'buf> Visitor<'buf> for HirFn<'buf> {
  type Error = ();

  fn visit_if(&mut self, expr: &If<'buf>) -> Result<(), Self::Error> {
    todo!()
  }
}
