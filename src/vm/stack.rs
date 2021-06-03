use super::error::VmResult;

#[derive(Debug, Clone)]
pub struct Stack<T, const SIZE: usize> {
  pos: usize,
  items: [T; SIZE],
}

impl<T: Copy + Default, const SIZE: usize> Stack<T, SIZE> {
  pub fn new() -> Self {
    Self {
      pos: 0,
      items: [T::default(); SIZE],
    }
  }

  pub fn pop(&mut self) -> VmResult<T> {
    if self.pos == 0 {
      todo!()
    }

    self.pos -= 1;
    let item = self.items[self.pos];

    Ok(item)
  }

  pub fn clear(&mut self, size: usize) {
    self.pos = self.pos.saturating_sub(size);
  }

  pub fn push(&mut self, value: T) -> VmResult<()> {
    if self.pos >= self.items.len() {
      todo!()
    }

    self.items[self.pos] = value;
    self.pos += 1;

    Ok(())
  }
}

impl<T: Copy + Default, const SIZE: usize> Default for Stack<T, SIZE> {
  fn default() -> Self {
    Self::new()
  }
}
