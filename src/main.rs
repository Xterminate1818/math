pub mod better_parse;
pub mod lex;
pub mod ops;
pub mod parse;
pub mod tok;

pub type Number = f64;

fn main() {
  let input = "3x+4(5)".to_string();
  let ls = lex::lex(input);
  println!("{ls:?}");
  let ts = tok::tokenize(ls).unwrap();
  println!("{ts:?}");
}
