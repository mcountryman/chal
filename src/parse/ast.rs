use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr<'buf> {
  Noop,

  // Literal
  String(Cow<'buf, str>),
  Number(f64),

  // Stmt
  If {
    condition: Box<Expr<'buf>>,
    body: Box<Expr<'buf>>,
    fallthrough: Option<Box<Expr<'buf>>>,
  },
  Call {
    name: &'buf str,
    args: Vec<Expr<'buf>>,
  },
  CallRet {
    var: &'buf str,
    name: &'buf str,
    args: Vec<Expr<'buf>>,
  },
  Assign {
    ident: &'buf str,
    expr: Box<Expr<'buf>>,
  },
  Function {
    name: &'buf str,
    params: Vec<&'buf str>,
    body: Box<Expr<'buf>>,
  },
  UnaryOp {
    op: UnaryOperator,
    expr: Box<Expr<'buf>>,
  },
  BinaryOp {
    lhs: Box<Expr<'buf>>,
    op: BinaryOperator,
    rhs: Box<Expr<'buf>>,
  },

  // Reference
  RefVar(&'buf str),
  RefParam(&'buf str),

  // Utility
  Compound(Vec<Expr<'buf>>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Block<'buf>(pub Vec<Expr<'buf>>);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum UnaryOperator {
  Neg,
  BNot,
  AddInc,
  SubInc,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  BOr,
  BAnd,
  BLShift,
  BRShift,

  Lt,
  LtEq,
  Gt,
  GtEq,
}
