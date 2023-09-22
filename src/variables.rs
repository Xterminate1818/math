use std::{collections::HashMap, f64::consts::*};

use crate::number::Number;

#[derive(Clone, Copy, Debug)]
pub struct Variable(pub Number);

impl Variable {
  pub const fn new(real: f64, imaginary: f64) -> Self {
    Self(Number::new(real, imaginary))
  }

  pub fn default_set() -> HashMap<String, Variable> {
    let mut map = HashMap::new();
    map.insert("pi".to_string(), _PI_VAR);
    map.insert("e".to_string(), _E_VAR);
    map.insert("tau".to_string(), _TAU_VAR);
    map.insert("i".to_string(), _I_VAR);
    map
  }
}

const _PI_VAR: Variable = Variable::new(PI, 0.0);
const _E_VAR: Variable = Variable::new(E, 0.0);
const _TAU_VAR: Variable = Variable::new(TAU, 0.0);
const _I_VAR: Variable = Variable::new(0.0, 1.0);
