use std::collections::HashMap;

use crate::{
  ops::{default_operators, Operator, OperatorSet, OP_MULT},
  Number,
};

fn default_variables() -> HashMap<String, Number> {
  let hm = HashMap::new();
  hm
}

/// Set of rules for evaluating statements
pub struct Algebra {
  pub operators: OperatorSet,
  pub implicit_operator: Option<Operator>,
  pub variables: HashMap<String, Number>,
}

impl Default for Algebra {
  fn default() -> Self {
    Self {
      operators: default_operators(),
      implicit_operator: Some(OP_MULT),
      variables: default_variables(),
    }
  }
}
