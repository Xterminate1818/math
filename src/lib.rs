use error::MathResult;

use crate::{
  context::Context, number::result_to_string, operator::OperatorSet, parse::Ast,
};

pub mod context;
pub mod error;
pub mod functions;
pub mod lexer;
pub mod number;
pub mod operator;
pub mod parse;
pub mod token;
pub mod variables;

pub fn evaluate(input: String) -> MathResult {
  let ls = lexer::lex(input);
  let ts = token::tokenize(OperatorSet::default(), ls)?;
  let context = Context::default();
  let tree = Ast::new(ts);
  let result = tree.evaluate(&context);
  result
}

pub fn evaluate_to_string(input: String) -> String {
  result_to_string(evaluate(input))
}

// fn main() {
//   let input = "asf(0)".to_string();
//   println!("input: {}", input);
//   let ls = lexer::lex(input);
//   println!("lexemes: {ls:?}");
//   let ts = token::tokenize(OperatorSet::default(),
// ls).unwrap();   println!("tokens: {ts:?}");
//
//   let context = Context::default();
//
//   let tree = Ast::new(ts);
//   let result = tree.evaluate(&context);
//   println!("{}", result_to_string(result));
// }
