use crate::{
  ast::{Assign, Expr},
  util::uuid::Uuid,
};
use std::{
  borrow::Cow,
  cell::{Ref, RefCell, RefMut},
  collections::HashMap,
  ops::Deref,
  rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Locals<'buf>(Rc<RefCell<LocalsImp<'buf>>>);

impl<'buf> Locals<'buf> {
  pub fn define_param(&self, name: &'buf str) -> LocalId {
    let mut imp = self.borrow_mut();
    let def = LocalDef {
      id: LocalId::default(),
      kind: LocalKind::Param,
      value: LocalValue::Unknown,
    };

    imp.defs.insert(def.id, def.clone());
    imp.defs_by_name.insert(name, def.id);

    def.id
  }

  pub fn define_var(&self, expr: &Assign<'buf>) -> LocalId {
    let mut imp = self.borrow_mut();
    let id = LocalId::default();
    let def = LocalDef {
      id,
      kind: LocalKind::Var,
      value: match &expr.expr {
        Expr::Number(value) => LocalValue::Number(value.0),
        Expr::String(value) => LocalValue::String(value.0.clone()),
        _ => LocalValue::Expr(expr.expr.clone()),
      },
    };

    imp.defs.insert(def.id, def);
    imp.defs_by_name.insert(expr.ident, id);

    id
  }

  pub fn borrow(&self) -> Ref<LocalsImp<'buf>> {
    self.0.deref().borrow()
  }

  pub fn borrow_mut(&self) -> RefMut<LocalsImp<'buf>> {
    self.0.deref().borrow_mut()
  }
}

#[derive(Debug, Clone)]
pub struct LocalsImp<'buf> {
  defs: HashMap<LocalId, LocalDef<'buf>>,
  defs_by_name: HashMap<&'buf str, LocalId>,

  refs_by_id: HashMap<LocalId, LocalRef>,
  refs_by_expr: HashMap<Expr<'buf>, LocalRef>,

  sets_by_id: HashMap<LocalId, LocalSet<'buf>>,
  sets_by_expr: HashMap<Expr<'buf>, LocalSet<'buf>>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalId(Uuid);

#[derive(Debug, Clone)]
pub struct LocalDef<'buf> {
  id: LocalId,
  kind: LocalKind,
  value: LocalValue<'buf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalRef(LocalId);

#[derive(Debug, Clone)]
pub struct LocalSet<'buf> {
  id: LocalId,
  value: LocalValue<'buf>,
}

#[derive(Debug, Clone)]
pub enum LocalValue<'buf> {
  Unknown,
  Expr(Expr<'buf>),
  Number(f64),
  String(Cow<'buf, str>),
}

#[derive(Debug, Clone)]
pub enum LocalKind {
  Var,
  Param,
}
