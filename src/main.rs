pub mod lex;
pub mod token;

fn main() {
  let input = "0+1/0".to_string();
  let ts = lex::lex(input);
  println!("{ts:?}");
}
