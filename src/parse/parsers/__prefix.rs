use super::{ParseError, ParseResult};
use crate::parse::{
  ast::{Expression, If, Var},
  Span, Token, TokenKind, Tokenizer,
};
use std::iter::Peekable;

pub struct PrefixParser<'buf> {
  tokenizer: Peekable<Tokenizer<'buf>>,
}

impl<'buf> PrefixParser<'buf> {
  pub fn parse(&mut self) -> ParseResult<'buf, ()> {
    Ok(())
  }

  fn parse_stmts(&mut self) -> ParseResult<Vec<Expression<'buf>>> {
    let mut expressions = Vec::new();

    // loop {
    //   expressions.push();
    // }

    Ok(expressions)
  }

  fn next_expr(&mut self) -> ParseResult<Expression<'buf>> {
    self.next_if_or_else(TokenKind::LParen, ParseError::expected_left_paren)?;

    let token = self.next_or_else(ParseError::empty_expression_eof)?;
    let expr = match token.kind {
      TokenKind::Ident("if") => Expression::If(If {
        condition: Box::new(self.next_expr()?),
        body: Box::new(self.next_expr()?),
        fallthrough: Box::new(self.next_expr()?),
      }),
      TokenKind::Ident("var") => Expression::Var(Var {
        ident: self.next_var_token()?,
        value: Box::new(self.next_expr()?),
      }),
      TokenKind::Ident("fun") => {}
      _ => return Err(ParseError::unexpected_token(token)),
    };

    self.next_if_or_else(TokenKind::RParen, ParseError::expected_right_paren)?;

    Ok(expr)
  }

  fn next_or_else<F, R>(&mut self, or: F) -> ParseResult<'buf, Token<'buf>>
  where
    F: FnOnce(Span<'buf>) -> R,
    R: Into<ParseError<'buf>>,
  {
    match self.tokenizer.next() {
      Some(token) => match token {
        Ok(token) => Ok(token),
        Err(err) => Err(err.into()),
      },
      None => Err(or(self.tokenizer.span()).into()),
    }
  }

  fn next_if_or_else<F, R>(&mut self, kind: TokenKind<'_>, or: F) -> ParseResult<'buf, Token<'buf>>
  where
    F: FnOnce(Span<'buf>) -> R,
    R: Into<ParseError<'buf>>,
  {
    match self.tokenizer.next() {
      Some(token) => match token {
        Ok(token) if token.kind == kind => Ok(token),
        Err(err) => Err(err.into()),
        Ok(token) => Err(or(token.span).into()),
      },
      None => Err(or(self.tokenizer.span()).into()),
    }
  }

  fn next_var_token(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.tokenizer.next() {
      Some(token) => match token {
        Ok(Token {
          kind: TokenKind::Var(var),
          ..
        }) => Ok(var),
        Ok(token) => Err(ParseError::expected_ident(self.tokenizer.span())),
        Err(err) => Err(err.into()),
      },
      None => Err(ParseError::expected_ident(self.tokenizer.span())),
    }
  }

  fn next_ident_token(&mut self) -> ParseResult<'buf, Option<&'buf str>> {
    match self.tokenizer.peekable() {
      Some(token) => match token {
        Ok(Token {
          kind: TokenKind::Ident(ident),
          ..
        }) => Ok(ident),
        Ok(token) => Err(ParseError::expected_ident(self.tokenizer.span())),
        Err(err) => Err(err.into()),
      },
      None => Err(ParseError::expected_ident(self.tokenizer.span())),
    }
  }

  fn next_ident_token_ok(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.tokenizer.next() {
      Some(token) => match token {
        Ok(Token {
          kind: TokenKind::Ident(ident),
          ..
        }) => Ok(ident),
        Ok(token) => Err(ParseError::expected_ident(self.tokenizer.span())),
        Err(err) => Err(err.into()),
      },
      None => Err(ParseError::expected_ident(self.tokenizer.span())),
    }
  }

  fn next_ident_tokens(&mut self) -> ParseResult<'buf, Vec<&'buf str>> {
    match self.tokenizer.next() {
      Some(token) => match token {
        Ok(Token {
          kind: TokenKind::Ident(ident),
          ..
        }) => Ok(ident),
        Ok(token) => Err(ParseError::expected_ident(self.tokenizer.span())),
        Err(err) => Err(err.into()),
      },
      None => Err(ParseError::expected_ident(self.tokenizer.span())),
    }
  }
}
