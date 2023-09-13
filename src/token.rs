#[derive(Debug, Clone, Copy)]
pub enum Token {
  Num(f64),
  Add,
  Sub,
  Mul,
  Div,
  LParen,
  RParen,
  EOL,
}
