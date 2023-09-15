/// Evaluation order (ignoring parens)
pub type EvalOrder = usize;
pub const EVAL_FIRST: EvalOrder = 0;
pub const EVAL_LAST: EvalOrder = std::usize::MAX;

/// Binary Operator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
}

impl Into<EvalOrder> for &BinOp {
  fn into(self) -> EvalOrder {
    match self {
      BinOp::Add => 1,
      BinOp::Sub => 1,
      BinOp::Mul => 2,
      BinOp::Div => 2,
    }
  }
}

impl BinOp {
  pub fn perform(&self, a: f64, b: f64) -> f64 {
    match self {
      BinOp::Add => a + b,
      BinOp::Sub => a - b,
      BinOp::Mul => a * b,
      BinOp::Div => a / b,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
  Num(f64),
  BinOp(BinOp),
  LParen,
  RParen,
  EOE,
}

fn parse_substr(s: &str, start: usize, end: usize) -> Token {
  let number_slice = &s[start..end];
  match number_slice.parse::<f64>() {
    Ok(n) => Token::Num(n),
    Err(_) => panic!("Invalid number: {}", number_slice),
  }
}

pub fn lex(input: String) -> Vec<Token> {
  let mut tokens = vec![];
  let mut start_of_num: Option<usize> = None;
  for (index, c) in input.char_indices() {
    let is_num = (c >= '0' && c <= '9') || c == '.';
    match start_of_num {
      Some(start) => {
        if is_num {
          continue;
        } else {
          // Finished parsing number
          let t = parse_substr(&input, start, index);
          tokens.push(t);
          start_of_num = None;
        }
      },
      None => {
        start_of_num = Some(index);
        continue;
      },
    }
    // Token is a symbol
    match c {
      '+' => tokens.push(Token::BinOp(BinOp::Add)),
      '-' => tokens.push(Token::BinOp(BinOp::Sub)),
      '*' => tokens.push(Token::BinOp(BinOp::Mul)),
      '/' => tokens.push(Token::BinOp(BinOp::Div)),
      _ => {
        panic!("Invalid character: {}", c)
      },
    }
  }
  if let Some(start) = start_of_num {
    tokens.push(parse_substr(&input, start, input.len()));
  }
  tokens.push(Token::EOE);
  tokens
}
