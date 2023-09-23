use thiserror::Error;

use crate::number::Number;

#[derive(Error, Debug)]
pub enum MathError {
  // Tokenizer errors
  #[error("Unrecognized symbol")]
  IllegalSymbol,
  #[error("Unmatched wrappings")]
  UnmatchedWrapping,
  #[error("Cannot parse number")]
  BadNumber,
  // Computation errors
  #[error("Division by zero is undefined")]
  DivisionByZero,
  #[error("Result too big to compute")]
  TooBig,
  #[error("No input")]
  NoInput,
  #[error("The {0} operation is undefined")]
  UndefinedOperation(String),
  #[error("The variable {0} is undefined")]
  UndefinedVariable(String),
  #[error("The function {0}() is undefined")]
  UndefinedFunction(String),
  // Unknown
  #[error("Unknown error")]
  Undefined,
}

pub type MathResult = Result<Number, MathError>;
