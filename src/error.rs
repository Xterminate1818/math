use thiserror::Error;

use crate::number::Number;

#[derive(Error, Debug)]
pub enum MathError {
  #[error("Unknown error")]
  Undefined,
  #[error("Division by zero is undefined")]
  DivisionByZero,
  #[error("Result too big to compute")]
  TooBig,
  #[error("The {0} operation is undefined")]
  UndefinedOperation(String),
  #[error("The variable {0} is undefined")]
  UndefinedVariable(String),
  #[error("The function {0}() is undefined")]
  UndefinedFunction(String),
}

pub type MathResult = Result<Number, MathError>;
