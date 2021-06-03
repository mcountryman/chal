pub mod error;
pub mod instr;
pub mod stack;
pub mod types;

use std::collections::HashMap;

use self::{
  error::VmResult,
  instr::Instruction,
  stack::Stack,
  types::{Step, Value},
};

type BuiltIn<'a> = Box<dyn FnMut() -> VmResult<Value<'a>>>;

pub struct VirtualMachine<'script, const STACK: usize> {
  pc: usize,
  stack: Stack<Value<'script>, STACK>,
  script: &'script [Instruction<'script>],
  locals: [Value<'script>; 255],
  builtins: HashMap<String, BuiltIn<'script>>,
}

impl<'script, const STACK: usize> VirtualMachine<'script, STACK> {
  pub fn new(script: &'script [Instruction<'script>]) -> Self {
    VirtualMachine::<'script, STACK> {
      pc: 0,
      stack: Default::default(),
      script,
      locals: [Value::Null; 255],
      builtins: HashMap::new(),
    }
  }

  pub fn builtin<F>(&mut self, name: &str, f: F) -> &mut Self
  where
    F: 'static + FnMut() -> VmResult<Value<'script>>,
  {
    self.builtins.insert(name.to_string(), Box::new(f));
    self
  }

  pub fn run(&mut self) -> VmResult<()> {
    todo!()
  }

  fn step(&mut self) -> VmResult<Step> {
    Ok(match self.script[self.pc] {
      Instruction::Nop => Step::Next,

      Instruction::LdStr(value) => {
        self.stack.push(Value::String(value))?;
        Step::Next
      }

      Instruction::LdF64(value) => {
        self.stack.push(Value::Number(value))?;
        Step::Next
      }

      Instruction::Store(local) => {
        self.locals[local as usize] = self.stack.pop()?;
        Step::Next
      }

      Instruction::JmpEq(to) => {
        if self.stack.pop()? == self.stack.pop()? {
          Step::Jmp(to)
        } else {
          Step::Next
        }
      }

      Instruction::JmpNEq(to) => {
        if self.stack.pop()? != self.stack.pop()? {
          Step::Jmp(to)
        } else {
          Step::Next
        }
      }

      Instruction::Call(to) => Step::Next,

      Instruction::CallVirt(name) => Step::Next,

      Instruction::Ret => Step::Next,

      _ => todo!(),
    })
  }
}
