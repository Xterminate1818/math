use std::collections::HashMap;

use crate::{
  error::{MathError, MathResult},
  functions::Function,
  number::Number,
  variables::Variable,
};

#[derive(Debug, Clone)]
pub struct Context {
  vars: HashMap<String, Variable>,
  funcs: HashMap<String, Function>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      vars: HashMap::new(),
      funcs: HashMap::new(),
    }
  }

  pub fn with(
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
  ) -> Self {
    Self {
      vars: variables,
      funcs: functions,
    }
  }

  pub fn assign_variable(&mut self, name: String, value: Number) {
    self.vars.insert(name, Variable(value));
  }

  pub fn read_variable(&self, name: String) -> MathResult {
    match self.vars.get(&name) {
      Some(var) => Ok(var.0.clone()),
      None => Err(MathError::UndefinedVariable(name)),
    }
  }

  pub fn assign_function(&mut self, name: String, function: Function) {
    self.funcs.insert(name, function);
  }

  pub fn compute_function(
    &self,
    name: String,
    operand: MathResult,
  ) -> MathResult {
    match self.funcs.get(&name) {
      Some(func) => func.compute(operand),
      None => Err(MathError::UndefinedFunction(name)),
    }
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::with(Variable::default_set(), Function::default_set())
  }
}
