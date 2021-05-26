pub mod ast;
pub mod error;
pub mod tokens;
pub mod types;

pub use ast::*;
pub use error::*;
pub use tokens::*;
pub use types::*;

pub struct Parser<'buf> {
  tokens: Tokenizer<'buf>,
  peeked: Option<Option<Token<'buf>>>,
}

impl<'buf> Parser<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      tokens: Tokenizer::new(buf),
      peeked: None,
    }
  }

  pub fn parse(&mut self) -> ParseResult<'buf, Block<'buf>> {
    todo!()
  }

  fn next_expr(&mut self) -> ParseResult<'buf, Option<Expression<'buf>>> {
    Ok(match self.tokens.next().transpose()? {
      Some(token) => {}
      None => {}
    })
  }
}
