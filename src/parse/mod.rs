pub mod ast;
pub mod error;
pub mod tokens;
pub mod types;

pub use ast::*;
pub use error::*;
pub use tokens::*;
pub use types::*;

use std::iter::Peekable;

pub struct Parser<'buf> {
  tokens: Peekable<Tokenizer<'buf>>,
}

impl<'buf> Parser<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      tokens: Tokenizer::new(buf).peekable(),
    }
  }

  pub fn parse(&mut self) -> ParseResult<'buf, Expr<'buf>> {
    Ok(self.next_expr(false)?.unwrap_or(Expr::Noop))
  }

  fn next_expr(&mut self, first: bool) -> ParseResult<'buf, Option<Expr<'buf>>> {
    if let Some(Ok(Token(_, TokenKind::RParen))) = self.tokens.peek() {
      return Ok(None);
    };

    let next = match self.tokens.next().transpose()? {
      Some(next) => next,
      None => return Ok(None),
    };

    Ok(match next {
      Token(span, TokenKind::LParen) => {
        let exprs = self.next_exprs(true)?;
        match self.tokens.next().transpose()? {
          Some(token) if token.is_right_paren() => {}
          Some(Token(span, _)) => return Err(ParseError::expected_right_paren(&span)),
          _ => return Err(ParseError::expected_right_paren(&span)),
        };

        match exprs.len() {
          0 => None,
          1 => Some(exprs[0].clone()),
          _ => Some(Expr::Compound(exprs)),
        }
      }

      Token(span, TokenKind::Ident("var")) if first => Some(Expr::Assign {
        ident: self.next_ident(&span)?,
        expr: self
          .next_expr(false)?
          .map(Box::new)
          .ok_or_else(|| ParseError::expected_var_expr(&span))?,
      }),

      Token(span, TokenKind::Ident("if")) if first => Some(Expr::If {
        condition: self
          .next_expr(false)?
          .map(Box::new)
          .ok_or_else(|| ParseError::expected_if_condition(&span))?,
        body: self
          .next_expr(false)?
          .map(Box::new)
          .ok_or_else(|| ParseError::expected_if_body(&span))?,
        fallthrough: self.next_expr(false)?.map(Box::new),
      }),

      Token(span, TokenKind::Ident("fun")) if first => Some(Expr::Function {
        name: self.next_ident(&span)?,
        params: self.next_params(&span)?,
        body: self
          .next_expr(false)?
          .map(Box::new)
          .ok_or_else(|| ParseError::expected_func_body(&span))?,
      }),

      Token(_, TokenKind::Ident(ident)) if first => Some(match self.tokens.peek().cloned() {
        Some(Ok(Token(_, TokenKind::RParen))) => Expr::RefParam(ident),
        _ => Expr::Call {
          name: ident,
          args: self.next_exprs(false)?,
        },
      }),

      Token(span, TokenKind::Var(var)) if first => Some(match self.tokens.peek().cloned() {
        Some(Ok(Token(paren, TokenKind::LParen))) => {
          // Consume `(`
          self.tokens.next().transpose()?;

          let name = self.next_ident(&span)?;
          let args = self.next_exprs(false)?;

          match self
            .tokens
            .next()
            .transpose()?
            .ok_or_else(|| ParseError::expected_right_paren(&paren))?
          {
            Token(_, TokenKind::RParen) => {}
            Token(span, _) => return Err(ParseError::expected_right_paren(&span)),
          }

          Expr::CallRet { var, name, args }
        }
        _ => Expr::RefVar(var),
      }),

      Token(span, TokenKind::Add) if first => Some(self.next_binary_op(BinaryOperator::Add, span)?),
      Token(span, TokenKind::Sub) if first => Some(self.next_binary_op(BinaryOperator::Sub, span)?),
      Token(span, TokenKind::Mul) if first => Some(self.next_binary_op(BinaryOperator::Mul, span)?),
      Token(span, TokenKind::Div) if first => Some(self.next_binary_op(BinaryOperator::Div, span)?),
      Token(span, TokenKind::Pow) if first => Some(self.next_binary_op(BinaryOperator::Pow, span)?),
      Token(span, TokenKind::Mod) if first => Some(self.next_binary_op(BinaryOperator::Mod, span)?),
      Token(span, TokenKind::BOr) if first => Some(self.next_binary_op(BinaryOperator::BOr, span)?),
      Token(span, TokenKind::BAnd) if first => {
        Some(self.next_binary_op(BinaryOperator::BAnd, span)?)
      }
      Token(span, TokenKind::BLShift) if first => {
        Some(self.next_binary_op(BinaryOperator::BLShift, span)?)
      }
      Token(span, TokenKind::BRShift) if first => {
        Some(self.next_binary_op(BinaryOperator::BRShift, span)?)
      }
      Token(span, TokenKind::Gt) if first => Some(self.next_binary_op(BinaryOperator::Gt, span)?),
      Token(span, TokenKind::Lt) if first => Some(self.next_binary_op(BinaryOperator::Lt, span)?),
      Token(span, TokenKind::GtEq) if first => {
        Some(self.next_binary_op(BinaryOperator::GtEq, span)?)
      }
      Token(span, TokenKind::LtEq) if first => {
        Some(self.next_binary_op(BinaryOperator::LtEq, span)?)
      }

      Token(span, TokenKind::BNot) if first => Some(self.next_unary_op(UnaryOperator::BNot, span)?),
      Token(span, TokenKind::AddInc) if first => {
        Some(self.next_unary_op(UnaryOperator::AddInc, span)?)
      }
      Token(span, TokenKind::SubInc) if first => {
        Some(self.next_unary_op(UnaryOperator::SubInc, span)?)
      }

      token => Some(self.next_simple(token)?),
    })
  }

  fn next_simple(&mut self, token: Token<'buf>) -> ParseResult<'buf, Expr<'buf>> {
    match token {
      Token(_, TokenKind::Var(value)) => Ok(Expr::RefVar(value)),
      Token(_, TokenKind::Ident(value)) => Ok(Expr::RefParam(value)),
      Token(_, TokenKind::Number(value)) => Ok(Expr::Number(value)),
      Token(_, TokenKind::String(value)) => Ok(Expr::String(value)),

      _ => Err(ParseError::unexpected_token(&token)),
    }
  }

  fn next_exprs(&mut self, mut first: bool) -> ParseResult<'buf, Vec<Expr<'buf>>> {
    let mut exprs = Vec::new();

    while let Some(expr) = self.next_expr(first)? {
      first = false;
      exprs.push(expr);
    }

    Ok(exprs)
  }

  fn next_binary_op(
    &mut self,
    op: BinaryOperator,
    span: Span<'buf>,
  ) -> ParseResult<'buf, Expr<'buf>> {
    Ok(Expr::BinaryOp {
      op,
      lhs: self
        .next_expr(false)?
        .map(Box::new)
        .ok_or_else(|| ParseError::expected_op_lhs(&span))?,
      rhs: self
        .next_expr(false)?
        .map(Box::new)
        .ok_or_else(|| ParseError::expected_op_lhs(&span))?,
    })
  }

  fn next_unary_op(
    &mut self,
    op: UnaryOperator,
    span: Span<'buf>,
  ) -> ParseResult<'buf, Expr<'buf>> {
    Ok(Expr::UnaryOp {
      op,
      expr: self
        .next_expr(false)?
        .map(Box::new)
        .ok_or_else(|| ParseError::expected_op_lhs(&span))?,
    })
  }

  //   fn next_expr(&mut self) -> ParseResult<'buf, Option<Expr<'buf>>> {
  //     Ok(match self.tokens.next().transpose()? {
  //       Some(token) => match token.kind {
  //         TokenKind::LParen => {
  //           let expr = self.next_expr_many()?;
  //           match self.tokens.next().transpose()? {
  //             Some(token) if token.is_right_paren() => {}
  //             Some(token) => return Err(ParseError::expected_right_paren(token.span)),
  //             None => return Err(ParseError::expected_right_paren(self.tokens.span())),
  //           };

  //           Some(expr)
  //         }

  //         TokenKind::Ident("var") => Some(Expr::Assign {
  //           ident: self.next_ident()?,
  //           expr: self
  //             .next_expr()?
  //             .map(Box::new)
  //             .ok_or_else(|| ParseError::expected_var_expr(self.tokens.span()))?,
  //         }),

  //         TokenKind::Ident("if") => Some(Expr::If {
  //           condition: self
  //             .next_expr()?
  //             .map(Box::new)
  //             .ok_or_else(|| ParseError::expected_if_condition(self.tokens.span()))?,
  //           body: self
  //             .next_expr()?
  //             .map(Box::new)
  //             .ok_or_else(|| ParseError::expected_if_body(self.tokens.span()))?,
  //           fallthrough: self.next_expr()?.map(Box::new),
  //         }),

  //         TokenKind::Ident("fun") => Some(Expr::Function {
  //           name: self.next_ident()?,
  //           params: self.next_params()?,
  //           body: self
  //             .next_expr()?
  //             .map(Box::new)
  //             .ok_or_else(|| ParseError::expected_func_body(self.tokens.span()))?,
  //         }),

  //         _ => Some(self.next_expr_simple(token)?),
  //       },
  //       None => None,
  //     })
  //   }

  //   fn next_expr_simple(&mut self, token: Token<'buf>) -> ParseResult<'buf, Expr<'buf>> {
  //     match token.kind {
  //       TokenKind::Number(value) => Ok(Expr::Number(value)),
  //       TokenKind::String(value) => Ok(Expr::String(value)),
  //       TokenKind::Var(value) => Ok(Expr::RefVar(value)),
  //       TokenKind::Ident(value) => Ok(Expr::RefParam(value)),
  //       _ => Err(ParseError::unexpected_token(token)),
  //     }
  //   }

  //   fn next_expr_many(&mut self) -> ParseResult<'buf, Expr<'buf>> {
  //     let mut exprs = Vec::new();

  //     while let Some(expr) = self.next_expr()? {
  //       exprs.push(expr);
  //     }

  //     Ok(match exprs.len() {
  //       0 => Expr::Noop,
  //       1 => exprs[0].clone(),
  //       _ => Expr::Compound(exprs),
  //     })
  //   }

  fn next_var(&mut self, beg: &Span<'buf>) -> ParseResult<'buf, &'buf str> {
    match self.tokens.next().transpose()? {
      Some(Token(span, TokenKind::Var(var))) => Ok(var),
      Some(Token(span, _)) => Err(ParseError::expected_var_expr(&span)),
      _ => Err(ParseError::expected_var_expr(beg)),
    }
  }

  fn next_ident(&mut self, beg: &Span<'buf>) -> ParseResult<'buf, &'buf str> {
    match self.tokens.next().transpose()? {
      Some(Token(span, TokenKind::Ident(ident))) => Ok(ident),
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
  use super::{Expr, Parser};
  use std::borrow::Cow;

  #[test]
  fn test_parse_var() {
    assert_eq!(
      Parser::new("$variable").parse().unwrap(),
      Expr::RefVar("variable")
    );
  }

  #[test]
  fn test_parse_ident() {
    assert_eq!(
      Parser::new("variable").parse().unwrap(),
      Expr::RefParam("variable")
    );
  }

  #[test]
  fn test_parse_number() {
    assert_eq!(Parser::new("69420").parse().unwrap(), Expr::Number(69420.0));
  }

  #[test]
  fn test_parse_string() {
    assert_eq!(
      Parser::new("\"string\"").parse().unwrap(),
      Expr::String(Cow::from("string"))
    );
  }

  #[test]
  fn test_nested() {
    assert_eq!(
      Parser::new("((((((\"string\"))))))").parse().unwrap(),
      Expr::String(Cow::from("string"))
    );
  }

  #[test]
  fn test_compund() {
    assert_eq!(
      Parser::new("(1 2)").parse().unwrap(),
      Expr::Compound(vec![Expr::Number(1.0), Expr::Number(2.0)])
    );
  }

  #[test]
  fn test_parse_assign() {
    assert_eq!(
      Parser::new("(var variable 1)").parse().unwrap(),
      Expr::Assign {
        ident: "variable",
        expr: Box::new(Expr::Number(1.0))
      }
    );

    assert_eq!(
      Parser::new("((var variable (1)))").parse().unwrap(),
      Expr::Assign {
        ident: "variable",
        expr: Box::new(Expr::Number(1.0))
      }
    );
  }

  #[test]
  fn test_if() {
    assert_eq!(
      Parser::new("(if $variable 1 0)").parse().unwrap(),
      Expr::If {
        condition: Box::new(Expr::RefVar("variable")),
        body: Box::new(Expr::Number(1.0)),
        fallthrough: Some(Box::new(Expr::Number(0.0)))
      }
    );

    assert_eq!(
      Parser::new("(if $variable 1)").parse().unwrap(),
      Expr::If {
        condition: Box::new(Expr::RefVar("variable")),
        body: Box::new(Expr::Number(1.0)),
        fallthrough: None
      }
    );
  }

  #[test]
  fn test_func() {
    assert_eq!(
      Parser::new("(fun function (a b c d) 1)").parse().unwrap(),
      Expr::Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: Box::new(Expr::Number(1.0))
      }
    );

    assert_eq!(
      Parser::new("(fun function (a b c d) (1 2 3 a b c d))")
        .parse()
        .unwrap(),
      Expr::Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: Box::new(Expr::Compound(vec![
          Expr::Number(1.0),
          Expr::Number(2.0),
          Expr::Number(3.0),
          Expr::RefParam("a"),
          Expr::RefParam("b"),
          Expr::RefParam("c"),
          Expr::RefParam("d"),
        ]))
      }
    );
  }

  #[test]
  fn test_call() {
    assert_eq!(
      Parser::new("(function 1 2 3 4)").parse().unwrap(),
      Expr::Call {
        name: "function",
        args: vec![
          Expr::Number(1.0),
          Expr::Number(2.0),
          Expr::Number(3.0),
          Expr::Number(4.0),
        ]
      }
    );
  }

  #[test]
  fn test_call_ret() {
    assert_eq!(
      Parser::new("($output (function 1 2 3 4))").parse().unwrap(),
      Expr::CallRet {
        var: "output",
        name: "function",
        args: vec![
          Expr::Number(1.0),
          Expr::Number(2.0),
          Expr::Number(3.0),
          Expr::Number(4.0),
        ]
      }
    );
  }

  #[test]
  pub fn test_parse_errors_chal() {
    println!(
      "{:#?}",
      Parser::new(include_str!("../../data/errors.chal"))
        .parse()
        .unwrap()
    );
  }

  // #[test]
  // pub fn test_parse_fizzbuzz_chal() {
  //   assert_eq!(
  //     Parser::new(include_str!("../../data/fizzbuzz.chal"))
  //       .parse()
  //       .unwrap(),
  //     Block(vec![
  //       Expr::Assign {
  //         ident: "counter",
  //         expr: Box::new(Expr::Number(0.0))
  //       },
  //       Expr::Call {
  //         name: "recursiveIncr",
  //         args: vec![Expr::Number(100.0)]
  //       },
  //       Expr::Function {
  //         name: "recursiveIncr",
  //         params: vec!["max"],
  //         body: Box::new(Expr::Compound(vec![Expr::Call {
  //           name: "print",
  //           args: vec![Expr::Call {
  //             name: "fizzbuzz",
  //             args: vec![Expr::RefVar("counter")]
  //           }]
  //         }]))
  //       } // (var counter 0)

  //         // #entry point
  //         // (
  //         //     (recursiveIncr 100)
  //         // )

  //         // (fun recursiveIncr (max)
  //         //    (
  //         //         (print (fizzbuzz $counter))

  //         //         (
  //         //             if (equal $counter max)
  //         //             $counter
  //         //             (recursiveIncr (++ $counter) max)
  //         //         )
  //         //     )
  //         // )

  //         // (fun fizzbuzz (value)
  //         //     (
  //         //         (if (equal (* 15 (/ 15 value)) value)
  //         //             "Fizzbuzz"
  //         //             (if (equal (* 5 (/ 5 value)) value)
  //         //                 "Buzz"
  //         //                 (if (equal (* 3 (/ 3 value)) value)
  //         //                     "Fizz"
  //         //                      value
  //         //                 )
  //         //             )
  //         //         )
  //         //     )
  //         // )
  //     ])
  //   );
  // }

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

  // #[test]
  // #[cfg_attr(miri, ignore)]
  // pub fn test_parse_stress() {
  //   let merged = concat!(
  //     include_str!("../../data/errors.chal"),
  //     include_str!("../../data/fizzbuzz.chal"),
  //     include_str!("../../data/math.chal"),
  //     include_str!("../../data/recursion.chal"),
  //     include_str!("../../data/string.chal"),
  //     include_str!("../../data/whitespace.chal"),
  //   )
  //   .repeat(1_000);

  //   Parser::new(&merged).parse().unwrap();
  // }
}
