pub mod lex;
pub mod ops;
pub mod parse;

pub type Number = f64;

fn main() {
  let input = "6*4/12+72/8-9".to_string();
  let ts = lex::lex(input);
  println!("{ts:?}");
  let ast = parse::AST::new(ts);
  let solution = ast.evaluate().unwrap();
  println!("{solution}");
}
