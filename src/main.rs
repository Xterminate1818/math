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

fn main() {
  let input = "asf(0)".to_string();
  println!("input: {}", input);
  let ls = lexer::lex(input);
  println!("lexemes: {ls:?}");
  let ts = token::tokenize(OperatorSet::default(), ls).unwrap();
  println!("tokens: {ts:?}");

  let context = Context::default();

  let tree = Ast::new(ts);
  let result = tree.evaluate(&context);
  println!("{}", result_to_string(result));
}
