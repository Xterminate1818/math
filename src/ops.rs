use std::{collections::HashSet, hash::Hash};

use crate::Number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Associativity {
  Left,
  Right,
}

#[derive(Debug, Clone)]
pub struct Operator {
  pub symbol: char,
  pub associative: bool,
  pub associativity: Associativity,
  pub precedence: usize,
  pub operation_func: fn(a: Number, b: Number) -> Result<Number, String>,
}

impl Hash for Operator {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.symbol.hash(state)
  }
}

// Needs to agree with hash
impl PartialEq for Operator {
  fn eq(&self, other: &Self) -> bool {
    self.symbol == other.symbol
  }
}

// Needs to agree with PartialEq
impl Eq for Operator {
}

impl Operator {
  pub fn perform(&self, a: Number, b: Number) -> Result<Number, String> {
    (self.operation_func)(a, b)
  }
}

pub type OperatorSet = HashSet<Operator>;

pub fn default_operators() -> OperatorSet {
  let mut set = HashSet::new();
  set.insert(OP_ADD);
  set.insert(OP_SUB);
  set.insert(OP_MULT);
  set.insert(OP_DIV);
  set.insert(OP_RAISE);
  set
}

fn check_result(num: Number) -> Result<Number, String> {
  if num.is_nan() {
    Err("NAN".to_string())
  } else if num.is_infinite() {
    Err("INFINITY".to_string())
  } else {
    Ok(num)
  }
}

fn _op_add(a: Number, b: Number) -> Result<Number, String> {
  check_result(a + b)
}

fn _op_sub(a: Number, b: Number) -> Result<Number, String> {
  check_result(a - b)
}

fn _op_mult(a: Number, b: Number) -> Result<Number, String> {
  check_result(a * b)
}

fn _op_div(a: Number, b: Number) -> Result<Number, String> {
  check_result(a / b)
}

fn _op_raise(a: Number, b: Number) -> Result<Number, String> {
  check_result(a.powf(b))
}

pub const OP_ADD: Operator = Operator {
  symbol: '+',
  associative: true,
  associativity: Associativity::Left,
  precedence: 0,
  operation_func: _op_add,
};

pub const OP_SUB: Operator = Operator {
  symbol: '-',
  associative: false,
  associativity: Associativity::Left,
  precedence: 0,
  operation_func: _op_sub,
};

pub const OP_MULT: Operator = Operator {
  symbol: '*',
  associative: true,
  associativity: Associativity::Left,
  precedence: 1,
  operation_func: _op_mult,
};

pub const OP_DIV: Operator = Operator {
  symbol: '/',
  associative: false,
  associativity: Associativity::Left,
  precedence: 1,
  operation_func: _op_div,
};

pub const OP_RAISE: Operator = Operator {
  symbol: '^',
  associative: false,
  associativity: Associativity::Right,
  precedence: 2,
  operation_func: _op_raise,
};
