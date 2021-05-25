use super::TokenKind;

pub enum Stmt<'buf> {
  If(If<'buf>),
  Call(Call<'buf>),
  UnaryOp(UnaryOp<'buf>),
  BinaryOp(BinaryOp<'buf>),
  Function(Function<'buf>),
}

pub enum Expression<'buf> {
  If(If<'buf>),
  Call(Call<'buf>),
  UnaryOp(UnaryOp<'buf>),
  BinaryOp(BinaryOp<'buf>),
  Function(Function<'buf>),
  Compound(Vec<Expression<'buf>>),
  Reference(Reference<'buf>),
}

pub struct If<'buf> {
  condition: Box<Expression<'buf>>,
  body: Box<Stmt<'buf>>,
  fallthrough: Box<Stmt<'buf>>,
}

pub struct Call<'buf> {
  name: &'buf str,
  params: Block<'buf>,
}

pub struct Block<'buf> {
  body: Vec<Stmt<'buf>>,
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
