use super::TokenKind;

pub enum Expression<'buf> {
  If(If<'buf>),
  Var(Var<'buf>),
  Call(Call<'buf>),
  String(&'buf str),
  Number(f64),
  UnaryOp(UnaryOp<'buf>),
  BinaryOp(BinaryOp<'buf>),
  Function(Function<'buf>),
  Reference(Reference<'buf>),
  Compound(Vec<Expression<'buf>>),
}

pub struct If<'buf> {
  pub condition: Box<Expression<'buf>>,
  pub body: Box<Expression<'buf>>,
  pub fallthrough: Box<Expression<'buf>>,
}

pub struct Var<'buf> {
  pub ident: &'buf str,
  pub value: Box<Expression<'buf>>,
}

pub struct Call<'buf> {
  name: &'buf str,
  params: Block<'buf>,
}

pub struct Block<'buf> {
  body: Vec<Expression<'buf>>,
}

pub struct Function<'buf> {
  name: &'buf str,
  params: Vec<&'buf str>,
  body: Block<'buf>,
}

pub struct Reference<'buf> {
  name: &'buf str,
}

pub struct UnaryOp<'buf> {
  op: TokenKind<'buf>,
  operand: Box<Expression<'buf>>,
}

pub struct BinaryOp<'buf> {
  op: TokenKind<'buf>,
  lhs: Box<Expression<'buf>>,
  rhs: Box<Expression<'buf>>,
}
