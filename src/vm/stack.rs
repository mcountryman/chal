use super::{error::VmResult, types::Value};
#[derive(Debug, Clone)]
pub struct Stack {
  pos: usize,
  items: Vec<Value>,
}

impl Stack {
  pub fn new(size: usize) -> Self {
    Self {
      pos: 0,
      items: vec![Value::Null; size],
    }
  }

  pub fn pop(&mut self) -> VmResult<Value> {
    println!(
      "pop() - pos: {}, item: {:?}",
      self.pos, self.items[self.pos]
    );

    if self.pos == 0 {
      todo!()
    }

    let actual = self.pos - 1;
    let item = self.items[actual].clone();
    self.items[actual] = Value::Null;
    self.pos -= 1;

    Ok(item)
  }

  pub fn clear(&mut self, size: usize) {
    self.pos = self.pos.saturating_sub(size);
  }

  pub fn is_empty(&mut self) -> bool {
    self.pos == 0
  }

  pub fn push(&mut self, value: Value) -> VmResult<()> {
    println!("push({:?}) - pos: {}", value, self.pos);

    if self.pos >= self.items.len() {
      todo!()
    }

    self.items[self.pos] = value;
    self.pos += 1;

    Ok(())
  }

  pub fn push_top(&mut self, value: Value) -> VmResult<()> {
    let top = self.items.len();
    self.items[top] = value;

    Ok(())
  }

  pub fn pop_top(&mut self) -> VmResult<Value> {
    let top = self.items.len();
    let item = self.items[top].clone();

    Ok(item)
  }
}
