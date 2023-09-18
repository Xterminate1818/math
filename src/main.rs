pub mod alg;
pub mod better_parse;
pub mod lex;
pub mod ops;
pub mod parse;
pub mod tok;

pub type Number = f64;

fn main() {
  let input = "a*1(4)".to_string();
  println!("{}", input);
  let ls = lex::lex(input);
  // println!("{ls:?}");
  let ts = tok::tokenize(ls).unwrap();
  println!("{ts:?}");
}
