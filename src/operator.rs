use std::collections::HashMap;

use crate::{
  error::{MathError, MathResult},
  number::{sanitize_result, Number},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
  Left,
  Right,
}

type BinaryOperationFunc = fn(a: Number, b: Number) -> MathResult;
type UnaryOperationFunc = fn(num: Number) -> MathResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
  pub name: &'static str,
  pub associativity: Associativity,
  pub precedence: usize,
  pub binary_func: Option<BinaryOperationFunc>,
  pub unary_func: Option<UnaryOperationFunc>,
}

impl Operation {
  pub fn perform_binary(&self, a: Number, b: Number) -> MathResult {
    match self.binary_func {
      Some(func) => sanitize_result((func)(a, b)?),
      None => Err(MathError::UndefinedOperation(format!(
        "binary {}",
        self.name
      ))),
    }
  }

  pub fn perform_unary(&self, num: Number) -> MathResult {
    match self.unary_func {
      Some(func) => sanitize_result((func)(num)?),
      None => Err(MathError::UndefinedOperation(format!(
        "unary {}",
        self.name
      ))),
    }
  }
}

impl std::fmt::Display for Operation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

pub struct OperatorSet {
  set: HashMap<char, Operation>,
  implicit: Operation,
}

impl OperatorSet {
  pub fn new(operators: Vec<(char, Operation)>, implicit: Operation) -> Self {
    let mut set = HashMap::new();
    for (symbol, op) in operators {
      set.insert(symbol, op);
    }
    Self { set, implicit }
  }

  pub fn get(&self, symbol: &char) -> Option<&Operation> {
    self.set.get(symbol)
  }

  pub fn implicit(&self) -> &Operation {
    &self.implicit
  }
}

impl Default for OperatorSet {
  fn default() -> Self {
    Self::new(
      vec![
        ('+', OP_ADD),
        ('-', OP_SUB),
        ('*', OP_MULT),
        ('/', OP_DIV),
        ('^', OP_EXP),
      ],
      OP_MULT,
    )
  }
}

fn _op_add(a: Number, b: Number) -> MathResult {
  Ok(a + b)
}

fn _op_sub(a: Number, b: Number) -> MathResult {
  Ok(a - b)
}

fn _op_sub_unary(num: Number) -> MathResult {
  Ok(-num)
}

fn _op_mult(a: Number, b: Number) -> MathResult {
  Ok(a * b)
}

fn _op_div(a: Number, b: Number) -> MathResult {
  Ok(a / b)
}

fn _op_exp(a: Number, b: Number) -> MathResult {
  Ok(a.powc(b))
}

pub const OP_ADD: Operation = Operation {
  name: "addition",
  associativity: Associativity::Left,
  precedence: 0,
  binary_func: Some(_op_add),
  unary_func: None,
};

pub const OP_SUB: Operation = Operation {
  name: "subtraction",
  associativity: Associativity::Left,
  precedence: 0,
  binary_func: Some(_op_sub),
  unary_func: Some(_op_sub_unary),
};

pub const OP_MULT: Operation = Operation {
  name: "multiplication",
  associativity: Associativity::Left,
  precedence: 1,
  binary_func: Some(_op_mult),
  unary_func: None,
};

pub const OP_DIV: Operation = Operation {
  name: "division",
  associativity: Associativity::Left,
  precedence: 1,
  binary_func: Some(_op_div),
  unary_func: None,
};

pub const OP_EXP: Operation = Operation {
  name: "exponentiation",
  associativity: Associativity::Right,
  precedence: 2,
  binary_func: Some(_op_exp),
  unary_func: None,
};
