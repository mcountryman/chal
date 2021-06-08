pub mod error;
pub mod expr;
pub mod visit;

pub use error::*;
pub use expr::*;
pub use visit::*;

use crate::{
  lex::{Lexer, Token, TokenKind},
  types::Span,
};
use std::iter::Peekable;

pub struct Parser<'buf> {
  tokens: Peekable<Lexer<'buf>>,
}

impl<'buf> Parser<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      tokens: Lexer::new(buf).peekable(),
    }
  }

  pub fn parse(&mut self) -> ParseResult<'buf, Expr<'buf>> {
    Ok(self.next_expr(0, false)?.unwrap_or_else(|| Noop.into()))
  }

  fn next_expr(&mut self, limit: usize, in_paren: bool) -> ParseResult<'buf, Option<Expr<'buf>>> {
    let mut exprs = Vec::with_capacity(1);

    loop {
      if let Some(Ok(Token(_, TokenKind::RParen))) = self.tokens.peek() {
        break;
      };

      if limit > 0 && exprs.len() >= limit {
        break;
      }

      match self.tokens.next().transpose()? {
        Some(Token(span, TokenKind::LParen)) => {
          if let Some(expr) = self.next_expr(255, true)? {
            exprs.push(expr);
          }

          match self.tokens.next().transpose()? {
            Some(Token(_, TokenKind::RParen)) => {}
            Some(Token(span, _)) => return Err(ParseError::expected_left_paren(&span)),
            None => return Err(ParseError::expected_left_paren(&span)),
          }
        }

        Some(token) => {
          if in_paren && exprs.is_empty() {
            if let Some(expr) = self.next_stmt(&token)? {
              exprs.push(expr);
              break;
            }
          }

          if let Some(expr) = self.next_simple(&token)? {
            exprs.push(expr);
          } else {
            return Err(ParseError::unexpected_token(&token));
          }
        }

        None => break,
      }
    }

    Ok(match exprs.len() {
      0 => None,
      1 => Some(exprs[0].clone()),
      _ => Some(Compound(exprs).into()),
    })
  }

  fn next_stmt(&mut self, token: &Token<'buf>) -> ParseResult<'buf, Option<Expr<'buf>>> {
    Ok(Some(match token {
      // (var ident expr)
      Token(span, TokenKind::Ident("var")) => Define {
        ident: self.next_ident(&span)?,
        expr: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_var_expr(&span))?,
      }
      .into(),

      // (if expr expr expr?)
      Token(span, TokenKind::Ident("if")) => If {
        condition: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_if_condition(&span))?,
        body: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_if_body(&span))?,
        fallthrough: self.next_expr(1, false)?,
      }
      .into(),

      // (fun ident (ident*) expr)
      Token(span, TokenKind::Ident("fun")) => Function {
        name: self.next_ident(&span)?,
        params: self.next_params(&span)?,
        body: self
          .next_expr(0, false)?
          .ok_or_else(|| ParseError::expected_func_body(&span))?,
      }
      .into(),

      Token(span, TokenKind::Ident("equal")) => self.next_binary_op(BinaryOperator::Eq, span)?,
      Token(span, TokenKind::Ident("neq")) => self.next_binary_op(BinaryOperator::NEq, span)?,

      // (ident expr*)
      Token(_, TokenKind::Ident(ident)) => match self.tokens.peek().cloned() {
        Some(Ok(Token(_, TokenKind::RParen))) => RefParam(ident).into(),
        _ => Call {
          name: ident,
          args: self.next_expr(0, false)?,
        }
        .into(),
      },

      Token(span, TokenKind::Var(ident)) => match self.tokens.peek().cloned() {
        Some(Ok(Token(paren, TokenKind::LParen))) => {
          // Consume `(`
          self.tokens.next().transpose()?;

          let name = self.next_ident(&span)?;
          let args = self.next_expr(0, false)?;

          match self
            .tokens
            .next()
            .transpose()?
            .ok_or_else(|| ParseError::expected_right_paren(&paren))?
          {
            Token(_, TokenKind::RParen) => {}
            Token(span, _) => return Err(ParseError::expected_right_paren(&span)),
          }

          Assign {
            ident,
            expr: Call { name, args }.into(),
          }
          .into()
        }
        _ => RefVar(ident).into(),
      },

      Token(span, TokenKind::Add) => self.next_binary_op(BinaryOperator::Add, span)?,
      Token(span, TokenKind::Sub) => self.next_binary_op(BinaryOperator::Sub, span)?,
      Token(span, TokenKind::Mul) => self.next_binary_op(BinaryOperator::Mul, span)?,
      Token(span, TokenKind::Div) => self.next_binary_op(BinaryOperator::Div, span)?,
      Token(span, TokenKind::Pow) => self.next_binary_op(BinaryOperator::Pow, span)?,
      Token(span, TokenKind::Mod) => self.next_binary_op(BinaryOperator::Mod, span)?,
      Token(span, TokenKind::BOr) => self.next_binary_op(BinaryOperator::BOr, span)?,
      Token(span, TokenKind::BAnd) => self.next_binary_op(BinaryOperator::BAnd, span)?,
      Token(span, TokenKind::BLShift) => self.next_binary_op(BinaryOperator::LShift, span)?,
      Token(span, TokenKind::BRShift) => self.next_binary_op(BinaryOperator::RShift, span)?,
      Token(span, TokenKind::Gt) => self.next_binary_op(BinaryOperator::Gt, span)?,
      Token(span, TokenKind::Lt) => self.next_binary_op(BinaryOperator::Lt, span)?,
      Token(span, TokenKind::GtEq) => self.next_binary_op(BinaryOperator::GtEq, span)?,
      Token(span, TokenKind::LtEq) => self.next_binary_op(BinaryOperator::LtEq, span)?,

      Token(span, TokenKind::BNot) => self.next_unary_op(UnaryOperator::BNot, span)?,
      Token(span, TokenKind::AddInc) => self.next_unary_op(UnaryOperator::AddInc, span)?,
      Token(span, TokenKind::SubInc) => self.next_unary_op(UnaryOperator::SubInc, span)?,

      _ => return Ok(None),
    }))
  }

  fn next_simple(&mut self, token: &Token<'buf>) -> ParseResult<'buf, Option<Expr<'buf>>> {
    Ok(Some(match token {
      Token(_, TokenKind::Var(value)) => RefVar(value).into(),
      Token(_, TokenKind::Ident(value)) => RefParam(value).into(),
      Token(_, TokenKind::Number(value)) => NumberLit(*value).into(),
      Token(_, TokenKind::String(value)) => StringLit(value.clone()).into(),

      _ => return Ok(None),
    }))
  }

  fn next_binary_op(
    &mut self,
    op: BinaryOperator,
    span: &Span<'buf>,
  ) -> ParseResult<'buf, Expr<'buf>> {
    Ok(
      BinaryOp {
        op,
        lhs: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_op_lhs(span))?,
        rhs: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_op_lhs(span))?,
      }
      .into(),
    )
  }

  fn next_unary_op(
    &mut self,
    op: UnaryOperator,
    span: &Span<'buf>,
  ) -> ParseResult<'buf, Expr<'buf>> {
    Ok(
      UnaryOp {
        op,
        expr: self
          .next_expr(1, false)?
          .ok_or_else(|| ParseError::expected_op_lhs(span))?,
      }
      .into(),
    )
  }

  fn next_ident(&mut self, beg: &Span<'buf>) -> ParseResult<'buf, &'buf str> {
    match self.tokens.next().transpose()? {
      Some(Token(_, TokenKind::Ident(ident))) => Ok(ident),
      Some(Token(span, _)) => Err(ParseError::expected_ident(&span)),
      _ => Err(ParseError::expected_ident(beg)),
    }
  }

  fn next_params(&mut self, beg: &Span<'buf>) -> ParseResult<'buf, Vec<&'buf str>> {
    let mut params = Vec::new();

    match self.tokens.next().transpose()? {
      Some(Token(_, TokenKind::LParen)) => {}
      Some(Token(span, _)) => return Err(ParseError::expected_left_paren(&span)),
      None => return Err(ParseError::expected_left_paren(beg)),
    };

    loop {
      match self.tokens.next().transpose()? {
        Some(Token(_, TokenKind::Ident(ident))) => params.push(ident),
        Some(Token(_, TokenKind::RParen)) => return Ok(params),
        Some(Token(span, _)) => return Err(ParseError::expected_right_paren(&span)),
        None => return Err(ParseError::expected_right_paren(beg)),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::borrow::Cow;

  #[test]
  fn test_parse_var() {
    assert_eq!(
      Parser::new("$variable").parse().unwrap(),
      RefVar("variable").into()
    );
  }

  #[test]
  fn test_parse_ident() {
    assert_eq!(
      Parser::new("variable").parse().unwrap(),
      RefParam("variable").into()
    );
  }

  #[test]
  fn test_parse_number() {
    assert_eq!(
      Parser::new("69420").parse().unwrap(),
      NumberLit(69420.0).into()
    );
  }

  #[test]
  fn test_parse_string() {
    assert_eq!(
      Parser::new("\"string\"").parse().unwrap(),
      StringLit(Cow::from("string")).into()
    );
  }

  #[test]
  fn test_nested() {
    assert_eq!(
      Parser::new("((((((\"string\"))))))").parse().unwrap(),
      StringLit(Cow::from("string")).into()
    );
  }

  #[test]
  fn test_compound() {
    assert_eq!(
      Parser::new("(1 2)").parse().unwrap(),
      Compound(vec![NumberLit(1.0).into(), NumberLit(2.0).into()]).into()
    );
  }

  #[test]
  fn test_parse_define() {
    assert_eq!(
      Parser::new("(var variable 1)").parse().unwrap(),
      Define {
        ident: "variable",
        expr: NumberLit(1.0).into()
      }
      .into()
    );

    assert_eq!(
      Parser::new("((var variable (1)))").parse().unwrap(),
      Define {
        ident: "variable",
        expr: NumberLit(1.0).into()
      }
      .into()
    );
  }

  #[test]
  fn test_if() {
    assert_eq!(
      Parser::new("(if $variable 1 0)").parse().unwrap(),
      If {
        condition: RefVar("variable").into(),
        body: NumberLit(1.0).into(),
        fallthrough: Some(NumberLit(0.0).into())
      }
      .into()
    );

    assert_eq!(
      Parser::new("(if $variable 1)").parse().unwrap(),
      If {
        condition: RefVar("variable").into(),
        body: NumberLit(1.0).into(),
        fallthrough: None
      }
      .into()
    );
  }

  #[test]
  fn test_func() {
    assert_eq!(
      Parser::new("(fun function (a b c d) 1)").parse().unwrap(),
      Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: NumberLit(1.0).into()
      }
      .into()
    );

    assert_eq!(
      Parser::new("(fun function (a b c d) (1 2 3 a b c d))")
        .parse()
        .unwrap(),
      Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: Compound(vec![
          NumberLit(1.0).into(),
          NumberLit(2.0).into(),
          NumberLit(3.0).into(),
          RefParam("a").into(),
          RefParam("b").into(),
          RefParam("c").into(),
          RefParam("d").into(),
        ])
        .into()
      }
      .into()
    );
  }

  #[test]
  fn test_call() {
    assert_eq!(
      Parser::new("(function 1 2 3 4)").parse().unwrap(),
      Call {
        name: "function",
        args: Some(
          Compound(vec![
            NumberLit(1.0).into(),
            NumberLit(2.0).into(),
            NumberLit(3.0).into(),
            NumberLit(4.0).into(),
          ])
          .into()
        )
      }
      .into()
    );
  }

  #[test]
  fn test_call_ret() {
    assert_eq!(
      Parser::new("($output (function 1 2 3 4))").parse().unwrap(),
      Assign {
        ident: "output",
        expr: Call {
          name: "function",
          args: Some(
            Compound(vec![
              NumberLit(1.0).into(),
              NumberLit(2.0).into(),
              NumberLit(3.0).into(),
              NumberLit(4.0).into(),
            ])
            .into()
          )
        }
        .into()
      }
      .into()
    );
  }

  #[test]
  fn test_binop() {
    let mut tests = [
      (Parser::new("(+ 0 1)"), BinaryOperator::Add),
      (Parser::new("(- 0 1)"), BinaryOperator::Sub),
      (Parser::new("(* 0 1)"), BinaryOperator::Mul),
      (Parser::new("(/ 0 1)"), BinaryOperator::Div),
      (Parser::new("(^ 0 1)"), BinaryOperator::Pow),
      (Parser::new("(% 0 1)"), BinaryOperator::Mod),
      (Parser::new("(equal 0 1)"), BinaryOperator::Eq),
      (Parser::new("(neq 0 1)"), BinaryOperator::NEq),
      (Parser::new("(< 0 1)"), BinaryOperator::Lt),
      (Parser::new("(<= 0 1)"), BinaryOperator::LtEq),
      (Parser::new("(> 0 1)"), BinaryOperator::Gt),
      (Parser::new("(>= 0 1)"), BinaryOperator::GtEq),
      (Parser::new("(| 0 1)"), BinaryOperator::BOr),
      (Parser::new("(& 0 1)"), BinaryOperator::BAnd),
      (Parser::new("(<< 0 1)"), BinaryOperator::LShift),
      (Parser::new("(>> 0 1)"), BinaryOperator::RShift),
    ];

    for (parser, op) in tests.iter_mut() {
      let left = parser.parse().unwrap();
      let right = BinaryOp {
        op: *op,
        lhs: NumberLit(0.0).into(),
        rhs: NumberLit(1.0).into(),
      }
      .into();

      assert_eq!(left, right);
    }
  }

  #[test]
  pub fn test_stmt_expr_chain() {
    assert!(Parser::new("(if 1 1 1 3)").parse().is_err())
  }

  #[test]
  pub fn test_parse_errors_chal() {
    assert!(Parser::new(include_str!("../../data/errors.chal"))
      .parse()
      .is_err())
  }

  #[test]
  pub fn test_parse_fizzbuzz_chal() {
    Parser::new(include_str!("../../data/fizzbuzz.chal"))
      .parse()
      .unwrap();
  }

  #[test]
  pub fn test_parse_math_chal() {
    Parser::new(include_str!("../../data/math.chal"))
      .parse()
      .unwrap();
  }

  #[test]
  pub fn test_parse_recursion_chal() {
    Parser::new(include_str!("../../data/recursion.chal"))
      .parse()
      .unwrap();
  }

  #[test]
  pub fn test_parse_string_chal() {
    Parser::new(include_str!("../../data/string.chal"))
      .parse()
      .unwrap();
  }

  #[test]
  pub fn test_parse_whitespace_chal() {
    Parser::new(include_str!("../../data/whitespace.chal"))
      .parse()
      .unwrap();
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  pub fn test_parse_stress() {
    let merged = concat!(
      include_str!("../../data/fizzbuzz.chal"),
      include_str!("../../data/math.chal"),
      include_str!("../../data/recursion.chal"),
      include_str!("../../data/string.chal"),
      include_str!("../../data/whitespace.chal"),
    )
    .repeat(1_000);

    Parser::new(&merged).parse().unwrap();
  }
}
