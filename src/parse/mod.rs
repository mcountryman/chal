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
}

impl<'buf> Parser<'buf> {
  pub fn new(buf: &'buf str) -> Self {
    Self {
      tokens: Tokenizer::new(buf),
    }
  }

  pub fn parse(&mut self) -> ParseResult<'buf, Expr<'buf>> {
    let mut depth = 0;

    Ok(self.next_expr(&mut depth, false)?.unwrap_or(Expr::Noop))
  }

  fn next_expr(
    &mut self,
    depth: &mut usize,
    mut first: bool,
  ) -> ParseResult<'buf, Option<Expr<'buf>>> {
    let mut exprs = Vec::new();

    loop {
      if let Some(expr) = self.next_expr_imp(depth, &mut first)? {
        exprs.push(expr);
      }

      if *depth == 0 {
        break;
      }
    }

    Ok(match exprs.len() {
      0 => None,
      1 => Some(exprs[0].clone()),
      _ => Some(Expr::Compound(exprs)),
    })
  }

  fn next_expr_imp(
    &mut self,
    depth: &mut usize,
    first: &mut bool,
  ) -> ParseResult<'buf, Option<Expr<'buf>>> {
    Ok(match self.tokens.next().transpose()? {
      Some(token) => {
        println!("token: {:?}", token);
        match token.kind {
          TokenKind::LParen => {
            *depth += 1;
            *first = true;

            None
          }
          TokenKind::RParen if *depth > 0 => {
            *depth -= 1;
            *first = false;

            None
          }

          TokenKind::Ident("var") if *depth > 0 && *first => Some(Expr::Assign {
            ident: self.next_ident()?,
            expr: self
              .next_expr(depth, false)?
              .map(Box::new)
              .ok_or_else(|| ParseError::expected_var_expr(self.tokens.span()))?,
          }),

          TokenKind::Ident("fun") if *depth > 0 && *first => Some(Expr::Function {
            name: self.next_ident()?,
            params: Vec::new(),
            body: self
              .next_expr(depth, false)?
              .map(Box::new)
              .ok_or_else(|| ParseError::expected_func_body(self.tokens.span()))?,
          }),

          TokenKind::Number(value) => Some(Expr::Number(value)),
          TokenKind::String(value) => Some(Expr::String(value)),

          _ => return Err(ParseError::unexpected_token(token)),
        }
      }
      None => None,
    })
  }

  fn next_var(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.tokens.next().transpose()? {
      Some(token) => match token.kind {
        TokenKind::Var(var) => Ok(var),
        _ => Err(ParseError::expected_ident(self.tokens.span())),
      },
      _ => Err(ParseError::expected_ident(self.tokens.span())),
    }
  }

  fn next_ident(&mut self) -> ParseResult<'buf, &'buf str> {
    match self.tokens.next().transpose()? {
      Some(token) => match token.kind {
        TokenKind::Ident(ident) => Ok(ident),
        _ => Err(ParseError::expected_ident(self.tokens.span())),
      },
      _ => Err(ParseError::expected_ident(self.tokens.span())),
    }
  }

  fn next_params(&mut self) -> ParseResult<'buf, &'buf str> {}
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
      Parser::new("(func function (a b c d) 1)").parse().unwrap(),
      Expr::Function {
        name: "function",
        params: vec!["a", "b", "c", "d"],
        body: Box::new(Expr::Number(1.0))
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
