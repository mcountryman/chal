pub mod error;
pub mod instr;
pub mod stack;
pub mod types;

use self::{
  error::VmResult,
  instr::Instruction,
  stack::Stack,
  types::{Step, Value},
};
use std::{cmp, collections::HashMap, rc::Rc};

type BuiltIn = dyn Fn() -> VmResult<Value>;
type BuiltInRc = Rc<BuiltIn>;

macro_rules! jmp_if {
  ($to:ident, $stack:expr, $a:ident $condition:tt $b:ident) => {{
    let a = $stack.pop()?;
    let b = $stack.pop()?;

    if a $condition b {
      Ok(Step::Jmp($to))
    } else {
      Ok(Step::Next)
    }}
  };
}

macro_rules! run_op {
  ($stack:expr, $a:ident $op:tt $b:ident) => {
    match ($stack.pop()?, $stack.pop()?) {
      (Value::Number($a), Value::Number($b)) => {
        $stack.push(Value::Number($a $op $b))?;
        Ok(Step::Next)
      }
      _ => todo!(),
    }
  };
  ($stack:expr, $a:ident.$op:tt($b:ident)) => {
    match ($stack.pop()?, $stack.pop()?) {
      (Value::Number($a), Value::Number($b)) => {
        $stack.push(Value::Number($a.$op($b)))?;
        Ok(Step::Next)
      }
      _ => todo!(),
    }
  };
}

macro_rules! run_int_op {
  ($stack:expr, $a:ident $op:tt $b:ident) => {
    match ($stack.pop()?, $stack.pop()?) {
      (Value::Number(a), Value::Number(b)) => {
        let $a = a as u64;
        let $b = b as u64;
        let c = ($a $op $b) as f64;

        $stack.push(Value::Number(c))?;
        Ok(Step::Next)
      }
      _ => todo!(),
    }
  };
}

pub struct VirtualMachine<'script> {
  pc: usize,
  stack: Stack,
  script: &'script [Instruction<'script>],
  locals: Vec<Value>,
  builtins: HashMap<String, BuiltInRc>,
}

impl<'script> VirtualMachine<'script> {
  pub fn new(script: &'script [Instruction<'script>]) -> Self {
    Self {
      pc: 0,
      stack: Stack::new(255),
      script,
      locals: vec![Value::Null; 255],
      builtins: HashMap::new(),
    }
  }

  pub fn builtin<F>(mut self, name: &str, f: F) -> Self
  where
    F: 'static + Fn() -> VmResult<Value>,
  {
    self.builtins.insert(name.to_string(), Rc::new(f));
    self
  }

  pub fn run(&mut self) -> VmResult<()> {
    while self.pc < self.script.len() {
      match self.run_next()? {
        Step::Next => self.pc += 1,
        Step::Jmp(to) => {
          let to = (((self.pc + 1) as isize) + to) as usize;
          let to = cmp::min(to, self.script.len());

          self.pc = to;
        }
        Step::JmpAbs(to) => {
          let to = cmp::min(to, self.script.len());

          self.pc = to;
        }
      }
    }

    Ok(())
  }

  fn run_next(&mut self) -> VmResult<Step> {
    match self.script[self.pc] {
      Instruction::Nop => Ok(Step::Next),

      Instruction::LdNull => self.run_ld(Value::Null),
      Instruction::LdTrue => self.run_ld(true),
      Instruction::LdFalse => self.run_ld(false),
      Instruction::LdF64(value) => self.run_ld(value),
      Instruction::LdStr(value) => self.run_ld(value),
      Instruction::LdAddr(value) => self.run_ld(value),
      Instruction::LdImport(value) => self.run_ldimport(value),

      Instruction::StLoc(local) => self.run_stloc(local),
      Instruction::LdLoc(local) => self.run_ldloc(local),

      Instruction::Jmp(to) => Ok(Step::Jmp(to)),
      Instruction::JmpEq(to) => jmp_if!(to, self.stack, a == b),
      Instruction::JmpNEq(to) => jmp_if!(to, self.stack, a != b),
      Instruction::JmpLt(to) => jmp_if!(to, self.stack, a < b),
      Instruction::JmpLtEq(to) => jmp_if!(to, self.stack, a <= b),
      Instruction::JmpGt(to) => jmp_if!(to, self.stack, a > b),
      Instruction::JmpGtEq(to) => jmp_if!(to, self.stack, a >= b),

      Instruction::Call => self.run_call(),
      Instruction::Ret => self.run_ret(),

      Instruction::Add => run_op!(self.stack, a + b),
      Instruction::Sub => run_op!(self.stack, a - b),
      Instruction::Mul => run_op!(self.stack, a * b),
      Instruction::Div => run_op!(self.stack, a / b),
      Instruction::Mod => run_op!(self.stack, a % b),
      Instruction::Pow => run_op!(self.stack, a.powf(b)),

      Instruction::BOr => run_int_op!(self.stack, a | b),
      Instruction::BAnd => run_int_op!(self.stack, a & b),
      Instruction::BLShift => run_int_op!(self.stack, a << b),
      Instruction::BRShift => run_int_op!(self.stack, a >> b),
    }
  }

  fn run_ld<V: Into<Value>>(&mut self, value: V) -> VmResult<Step> {
    self.stack.push(value.into())?;

    Ok(Step::Next)
  }

  fn run_ldimport(&mut self, value: &str) -> VmResult<Step> {
    match self.builtins.get(value) {
      Some(builtin) => self.stack.push(Value::BuiltIn(builtin.clone()))?,
      None => todo!(),
    };

    Ok(Step::Next)
  }

  fn run_ldloc(&mut self, local: u8) -> VmResult<Step> {
    self.stack.push(self.locals[local as usize].clone())?;

    Ok(Step::Next)
  }

  fn run_stloc(&mut self, local: u8) -> VmResult<Step> {
    self.locals[local as usize] = self.stack.pop()?;

    Ok(Step::Next)
  }

  fn run_call(&mut self) -> VmResult<Step> {
    self.stack.push_top(Value::Addr(self.pc + 1))?;

    let addr = self.stack.pop()?;
    let addr = match addr {
      Value::Addr(addr) => addr,
      _ => todo!(),
    };

    Ok(Step::JmpAbs(addr))
  }

  fn run_ret(&mut self) -> VmResult<Step> {
    let addr = self.stack.pop_top()?;
    let addr = match addr {
      Value::Addr(addr) => addr,
      _ => todo!(),
    };

    Ok(Step::JmpAbs(addr))
  }
}

#[cfg(test)]
mod tests {
  use super::{instr::Instruction, VirtualMachine};
  use crate::vm::types::Value;

  #[test]
  fn test_nop() {
    let mut vm = VirtualMachine::new(&[Instruction::Nop]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
  }

  #[test]
  fn test_ld_null() {
    let mut vm = VirtualMachine::new(&[Instruction::LdNull]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), Value::Null);
  }

  #[test]
  fn test_ld_true() {
    let mut vm = VirtualMachine::new(&[Instruction::LdTrue]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));
  }

  #[test]
  fn test_ld_false() {
    let mut vm = VirtualMachine::new(&[Instruction::LdFalse]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));
  }

  #[test]
  fn test_ld_str() {
    let mut vm = VirtualMachine::new(&[Instruction::LdStr("test")]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), "test".into());
  }

  #[test]
  fn test_ld_f64() {
    let mut vm = VirtualMachine::new(&[Instruction::LdF64(1337.69)]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), 1337.69.into());
  }

  #[test]
  fn test_ld_addr() {
    let mut vm = VirtualMachine::new(&[Instruction::LdAddr(0xdeadbeaf)]);
    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert_eq!(vm.stack.pop().unwrap(), Value::Addr(0xdeadbeaf));
  }

  #[test]
  fn test_ld_import() {
    let mut vm = VirtualMachine::new(&[Instruction::LdImport("printf")])
      //
      .builtin("printf", || Ok(Value::Null));

    vm.run().unwrap();

    assert_eq!(vm.pc, 1);
    assert!(matches!(vm.stack.pop().unwrap(), Value::BuiltIn(_)));
  }

  #[test]
  fn test_ld_loc() {
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::StLoc(0),
      Instruction::LdLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 3);
    assert_eq!(vm.stack.pop().unwrap(), Value::Number(69.420));
  }

  #[test]
  fn test_stloc() {
    let mut vm = VirtualMachine::new(&[Instruction::LdF64(69.420), Instruction::StLoc(0)]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 2);
    assert_eq!(vm.locals[0], Value::Number(69.420));
    assert!(vm.stack.is_empty());
  }

  #[test]
  fn test_jmp_eq() {
    // Test if jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(69.420),
      Instruction::JmpEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.60),
      Instruction::LdF64(69.420),
      Instruction::JmpEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  #[test]
  fn test_jmp_neq() {
    // Test if jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(69.420),
      Instruction::JmpNEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(69.420),
      Instruction::JmpNEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  #[test]
  fn test_jmp_lt() {
    // Test if jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(69.420),
      Instruction::JmpLt(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(420.69),
      Instruction::JmpLt(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  #[test]
  fn test_jmp_gt() {
    // Test if jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(420.69),
      Instruction::JmpGt(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(69.420),
      Instruction::JmpGt(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  #[test]
  fn test_jmp_lt_eq() {
    // Test if jump when less than
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(69.420),
      Instruction::JmpLtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if jump when equal
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(420.69),
      Instruction::JmpLtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(420.69),
      Instruction::JmpLtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  #[test]
  fn test_jmp_gt_eq() {
    // Test if jump when less than
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(69.420),
      Instruction::LdF64(420.69),
      Instruction::JmpGtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if jump when equal
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(420.69),
      Instruction::JmpGtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(-50),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(true));

    // Test if doesn't jump
    let mut vm = VirtualMachine::new(&[
      Instruction::LdF64(420.69),
      Instruction::LdF64(69.420),
      Instruction::JmpGtEq(3),
      Instruction::LdFalse,
      Instruction::StLoc(0),
      Instruction::Jmp(2),
      Instruction::LdTrue,
      Instruction::StLoc(0),
    ]);

    vm.run().unwrap();

    assert_eq!(vm.pc, 8);
    assert_eq!(vm.locals[0], Value::Bool(false));
  }

  macro_rules! {
    () => {

    };
  }
}
