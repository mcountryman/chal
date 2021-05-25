use super::ast::{BinaryOp, Block, Call, Expression, Function, If, Reference, Stmt, UnaryOp};

pub trait Visitor {
  type Error;

  fn visit_if_mut(&mut self, node: &If<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_call_mut(&mut self, node: &Call<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_unary_op_mut(&mut self, node: &UnaryOp<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_binary_op_mut(&mut self, node: &BinaryOp<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_function_mut(&mut self, node: &Function<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_reference_mut(&mut self, node: &Reference<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_block_mut(&mut self, node: &Block<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_expression_mut(&mut self, node: &Expression<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_stmt_mut(&mut self, node: &Stmt<'_>) -> Result<(), Self::Error> {
    Ok(())
  }
}

pub trait VisitorMut {
  type Error;

  fn visit_if(&mut self, node: &mut If<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_call(&mut self, node: &mut Call<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_unary_op(&mut self, node: &mut UnaryOp<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_binary_op(&mut self, node: &mut BinaryOp<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_function(&mut self, node: &mut Function<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_reference(&mut self, node: &mut Reference<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_block(&mut self, node: &mut Block<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_expression(&mut self, node: &mut Expression<'_>) -> Result<(), Self::Error> {
    Ok(())
  }

  fn visit_stmt(&mut self, node: &mut Stmt<'_>) -> Result<(), Self::Error> {
    Ok(())
  }
}
