use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr<'buf> {
  Noop(Noop),

  // Literal
  String(StringLit<'buf>),
  Number(NumberLit),

  // Stmt
  If(Box<If<'buf>>),
  Call(Box<Call<'buf>>),
  Assign(Box<Assign<'buf>>),
  Define(Box<Define<'buf>>),
  Function(Box<Function<'buf>>),
  UnaryOp(Box<UnaryOp<'buf>>),
  BinaryOp(Box<BinaryOp<'buf>>),

  // Reference
  RefVar(RefVar<'buf>),
  RefParam(RefParam<'buf>),

  // Utility
  Compound(Box<Compound<'buf>>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Noop;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StringLit<'buf>(pub Cow<'buf, str>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NumberLit(pub f64);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct If<'buf> {
  pub condition: Expr<'buf>,
  pub body: Expr<'buf>,
  pub fallthrough: Option<Expr<'buf>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Call<'buf> {
  pub name: &'buf str,
  pub args: Option<Expr<'buf>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Assign<'buf> {
  pub ident: &'buf str,
  pub expr: Expr<'buf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Define<'buf> {
  pub ident: &'buf str,
  pub expr: Expr<'buf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function<'buf> {
  pub name: &'buf str,
  pub params: Vec<&'buf str>,
  pub body: Expr<'buf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnaryOp<'buf> {
  pub op: UnaryOperator,
  pub expr: Expr<'buf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BinaryOp<'buf> {
  pub lhs: Expr<'buf>,
  pub op: BinaryOperator,
  pub rhs: Expr<'buf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RefVar<'buf>(pub &'buf str);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RefParam<'buf>(pub &'buf str);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Compound<'buf>(pub Vec<Expr<'buf>>);

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
  LShift,
  RShift,

  Eq,
  NEq,
  Lt,
  LtEq,
  Gt,
  GtEq,
}

impl From<Noop> for Expr<'_> {
  fn from(expr: Noop) -> Self {
    Expr::Noop(expr)
  }
}

impl<'buf> From<StringLit<'buf>> for Expr<'buf> {
  fn from(expr: StringLit<'buf>) -> Self {
    Expr::String(expr)
  }
}

impl<'buf> From<NumberLit> for Expr<'buf> {
  fn from(expr: NumberLit) -> Self {
    Expr::Number(expr)
  }
}

impl<'buf> From<If<'buf>> for Expr<'buf> {
  fn from(expr: If<'buf>) -> Self {
    Expr::If(Box::new(expr))
  }
}

impl<'buf> From<Call<'buf>> for Expr<'buf> {
  fn from(expr: Call<'buf>) -> Self {
    Expr::Call(Box::new(expr))
  }
}

impl<'buf> From<Assign<'buf>> for Expr<'buf> {
  fn from(expr: Assign<'buf>) -> Self {
    Expr::Assign(Box::new(expr))
  }
}

impl<'buf> From<Define<'buf>> for Expr<'buf> {
  fn from(expr: Define<'buf>) -> Self {
    Expr::Define(Box::new(expr))
  }
}

impl<'buf> From<Function<'buf>> for Expr<'buf> {
  fn from(expr: Function<'buf>) -> Self {
    Expr::Function(Box::new(expr))
  }
}

impl<'buf> From<UnaryOp<'buf>> for Expr<'buf> {
  fn from(expr: UnaryOp<'buf>) -> Self {
    Expr::UnaryOp(Box::new(expr))
  }
}

impl<'buf> From<BinaryOp<'buf>> for Expr<'buf> {
  fn from(expr: BinaryOp<'buf>) -> Self {
    Expr::BinaryOp(Box::new(expr))
  }
}

impl<'buf> From<RefVar<'buf>> for Expr<'buf> {
  fn from(expr: RefVar<'buf>) -> Self {
    Expr::RefVar(expr)
  }
}

impl<'buf> From<RefParam<'buf>> for Expr<'buf> {
  fn from(expr: RefParam<'buf>) -> Self {
    Expr::RefParam(expr)
  }
}

impl<'buf> From<Compound<'buf>> for Expr<'buf> {
  fn from(expr: Compound<'buf>) -> Self {
    Expr::Compound(Box::new(expr))
  }
}
