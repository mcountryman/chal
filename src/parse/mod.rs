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
    Ok(Block(self.next_exprs(true)?))
  }

  fn next_expr(&mut self, first: bool) -> ParseResult<'buf, Option<Expr<'buf>>> {
    match self.peek_token()? {
      Some(x) if x.is_right_paren() => {
        self.next_token()?;
        return Ok(None);
      }
      _ => {}
    };

    let expr = match self.next_token()? {
      Some(token) => match token.kind {
        TokenKind::Number(value) => Some(Expr::Number(value)),
        TokenKind::String(value) => Some(Expr::String(value)),

        TokenKind::LParen => self.next_expr(first)?,
        TokenKind::Ident("if") => Some(Expr::If {
          condition: self
            .next_expr(first)?
            .ok_or_else(|| ParseError::expected_if_condition(self.tokens.span()))
            .map(Box::new)?,

          body: self
            .next_expr(first)?
            .ok_or_else(|| ParseError::expected_if_body(self.tokens.span()))
            .map(Box::new)?,

          fallthrough: self.next_expr(first)?.map(Box::new),
        }),
        TokenKind::Ident("var") => Some(Expr::Assign {
          ident: self.next_ident()?,
          expr: self
            .next_expr(first)?
            .ok_or_else(|| ParseError::expected_var_expr(self.tokens.span()))
            .map(Box::new)?,
        }),
        TokenKind::Ident("func") => Some(Expr::Function {
          name: self.next_ident()?,
          params: self.next_parameters()?,
          body: self
            .next_expr(first)?
            .ok_or_else(|| ParseError::expected_func_body(self.tokens.span()))
            .map(Box::new)?,
        }),
        TokenKind::Ident(name) => Some({
          if first {
            Expr::RefParam(name)
          } else {
            Expr::Call {
              name,
              args: self.next_exprs(false)?,
            }
          }
        }),

        TokenKind::Var(var) => Some(if first {
          Expr::RefVar(var)
        } else {
          self.next_if_or_else(
            |token| token.is_left_paren(),
            ParseError::expected_left_paren,
          )?;

          let name = self.next_ident()?;
          let args = self.next_exprs(false)?;

          self.next_if_or_else(
            |token| token.is_right_paren(),
            ParseError::expected_right_paren,
          )?;

          Expr::CallRet { var, name, args }
        }),

        TokenKind::Add => Some(self.next_binary_op(BinaryOperator::Add)?),
        TokenKind::Sub => Some(self.next_binary_op(BinaryOperator::Sub)?),
        TokenKind::Mul => Some(self.next_binary_op(BinaryOperator::Mul)?),
        TokenKind::Div => Some(self.next_binary_op(BinaryOperator::Div)?),
        TokenKind::Pow => Some(self.next_binary_op(BinaryOperator::Pow)?),
        TokenKind::Mod => Some(self.next_binary_op(BinaryOperator::Mod)?),
        TokenKind::BOr => Some(self.next_binary_op(BinaryOperator::BOr)?),
        TokenKind::BAnd => Some(self.next_binary_op(BinaryOperator::BAnd)?),
        TokenKind::BLShift => Some(self.next_binary_op(BinaryOperator::BLShift)?),
        TokenKind::BRShift => Some(self.next_binary_op(BinaryOperator::BRShift)?),
        TokenKind::Lt => Some(self.next_binary_op(BinaryOperator::Lt)?),
        TokenKind::LtEq => Some(self.next_binary_op(BinaryOperator::LtEq)?),
        TokenKind::Gt => Some(self.next_binary_op(BinaryOperator::Gt)?),
        TokenKind::GtEq => Some(self.next_binary_op(BinaryOperator::GtEq)?),

        TokenKind::BNot => Some(self.next_unary_op(UnaryOperator::BNot)?),
        TokenKind::AddInc => Some(self.next_unary_op(UnaryOperator::AddInc)?),
        TokenKind::SubInc => Some(self.next_unary_op(UnaryOperator::SubInc)?),

        _ => return Err(ParseError::unexpected_token(token)),
      },
      None => None,
    };

    if in_paren {
      self.next_if_or_else(Token::is_right_paren, ParseError::expected_right_paren)?;
    }

    Ok(expr)
  }

  fn next_unary_op(&mut self, op: UnaryOperator) -> ParseResult<'buf, Expr<'buf>> {
    Ok(Expr::UnaryOp {
      op,
      expr: self
        .next_expr(false)?
        .ok_or_else(|| ParseError::expected_op_operand(self.tokens.span()))
        .map(Box::new)?,
    })
  }

  fn next_binary_op(&mut self, op: BinaryOperator) -> ParseResult<'buf, Expr<'buf>> {
    Ok(Expr::BinaryOp {
      op,
      lhs: self
        .next_expr(false)?
        .ok_or_else(|| ParseError::expected_op_lhs(self.tokens.span()))
        .map(Box::new)?,
      rhs: self
        .next_expr(false)?
        .ok_or_else(|| ParseError::expected_op_rhs(self.tokens.span()))
        .map(Box::new)?,
    })
  }

  fn next_exprs(&mut self, mut first: bool) -> ParseResult<'buf, Vec<Expr<'buf>>> {
    let mut exprs = Vec::new();

    while let Some(expr) = self.next_expr(first)? {
      first = false;
      exprs.push(expr)
    }

    Ok(exprs)
  }

  fn next_parameters(&mut self) -> ParseResult<'buf, Vec<&'buf str>> {
    let mut parameters = Vec::new();

    self.next_if_or_else(
      |token| token.is_left_paren(),
      ParseError::expected_left_paren,
    )?;

    while let Some(token) = self.peek_token()? {
      match token.kind {
        TokenKind::Ident(name) => {
          self.next_token()?;
          parameters.push(name);
        }
        _ => break,
      }
    }

    self.next_if_or_else(
      |token| token.is_right_paren(),
      ParseError::expected_right_paren,
    )?;

    Ok(parameters)
  }

  fn next_ident(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.next_token()? {
      Some(token) => match token.kind {
        TokenKind::Ident(ident) => Ok(ident),
        _ => Err(ParseError::expected_ident(self.tokens.span())),
      },
      None => Err(ParseError::expected_ident(self.tokens.span())),
    }
  }

  fn next_var(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.next_token()? {
      Some(token) => match token.kind {
        TokenKind::Var(ident) => Ok(ident),
        _ => Err(ParseError::expected_ident(self.tokens.span())),
      },
      None => Err(ParseError::expected_ident(self.tokens.span())),
    }
  }

  fn next_if_or_else<F, E>(&mut self, f: F, or: E) -> ParseResult<'buf, Token<'buf>>
  where
    F: Fn(&Token<'buf>) -> bool,
    E: Fn(Span<'buf>) -> ParseError<'buf>,
  {
    match self.next_token()? {
      Some(token) if f(&token) => Ok(token),
      Some(token) => Err(or(token.span)),
      None => Err(or(self.tokens.span())),
    }
  }

  fn next_token(&mut self) -> ParseResult<'buf, Option<Token<'buf>>> {
    match self.peeked.take() {
      Some(peeked) => Ok(peeked),
      None => Ok(self.tokens.next().transpose()?),
    }
  }

  fn peek_token(&mut self) -> ParseResult<'buf, &Option<Token<'buf>>> {
    if self.peeked.is_none() {
      self.peeked = Some(self.tokens.next().transpose()?);
    }

    match self.peeked.as_ref() {
      Some(v) => Ok(v),
      // SAFETY: a `None` variant for `self` would have been replaced by a `Some`
      // variant in the code above.
      None => panic!("Not possible"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Expr, Parser};
  use std::borrow::Cow;

  #[test]
  fn test_next_expr_parse_var() {
    assert_eq!(
      Parser::new("$variable").next_expr(true).unwrap().unwrap(),
      Expr::RefVar("variable")
    );
  }

  #[test]
  fn test_next_expr_parse_ident() {
    assert_eq!(
      Parser::new("variable").next_expr(true).unwrap().unwrap(),
      Expr::RefParam("variable")
    );
  }

  #[test]
  fn test_next_expr_parse_number() {
    assert_eq!(
      Parser::new("69420").next_expr(true).unwrap().unwrap(),
      Expr::Number(69420.0)
    );
  }

  #[test]
  fn test_next_expr_parse_string() {
    assert_eq!(
      Parser::new("\"string\"").next_expr(true).unwrap().unwrap(),
      Expr::String(Cow::from("string"))
    );
  }

  #[test]
  fn test_next_expr_nested() {
    assert_eq!(
      Parser::new("((((((\"string\"))))))")
        .next_expr(true)
        .unwrap()
        .unwrap(),
      Expr::String(Cow::from("string"))
    );
  }

  #[test]
  fn test_next_expr_compund() {
    assert_eq!(
      Parser::new("(1 2)").next_expr(true).unwrap().unwrap(),
      Expr::Compound(vec![Expr::Number(1.0), Expr::Number(2.0)])
    );
  }

  #[test]
  fn test_next_expr_parse_assign() {
    assert_eq!(
      Parser::new("(var variable 1)")
        .next_expr(true)
        .unwrap()
        .unwrap(),
      Expr::Assign {
        ident: "variable",
        expr: Box::new(Expr::Number(1.0))
      }
    );
  }

  #[test]
  fn test_next_expr_if() {
    assert_eq!(
      Parser::new("(if $variable 1 0)")
        .next_expr(true)
        .unwrap()
        .unwrap(),
      Expr::If {
        condition: Box::new(Expr::RefVar("variable")),
        body: Box::new(Expr::Number(1.0)),
        fallthrough: Some(Box::new(Expr::Number(0.0)))
      }
    );

    assert_eq!(
      Parser::new("(if $variable 1)")
        .next_expr(true)
        .unwrap()
        .unwrap(),
      Expr::If {
        condition: Box::new(Expr::RefVar("variable")),
        body: Box::new(Expr::Number(1.0)),
        fallthrough: None
      }
    );
  }

  #[test]
  fn test_next_expr_func() {
    assert_eq!(
      Parser::new("(func function (a b c d) 1)")
        .next_expr(true)
        .unwrap()
        .unwrap(),
      Expr::Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: Box::new(Expr::Number(1.0))
      }
    );
  }

  #[test]
  fn test_next_expr_call() {
    assert_eq!(
      Parser::new("(function 1 2 3 4)")
        .next_expr(true)
        .unwrap()
        .unwrap(),
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
  fn test_next_expr_call_ret() {
    assert_eq!(
      Parser::new("($output (function 1 2 3 4))")
        .next_expr(true)
        .unwrap()
        .unwrap(),
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

  // #[test]
  // pub fn test_parse_errors_chal() {
  //   Parser::new(include_str!("../../data/errors.chal"))
  //     .parse()
  //     .unwrap();
  // }

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
