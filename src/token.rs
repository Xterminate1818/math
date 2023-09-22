use crate::{
  lexer::{Lexeme, Wrapping},
  number::Number,
  operator::{Operation, OperatorSet},
};

pub type WrappingLevel = usize;

#[derive(Debug, Clone)]
pub enum TokenError {
  UnmatchedWrapping,
  BadNumber,
  IllegalSymbol,
}

#[derive(Clone)]
pub enum Token {
  Constant(Number),
  Variable(String),
  Function(String),
  Operator {
    op: Operation,
    wrapping: WrappingLevel,
  },
}

impl Token {
  pub fn is_numeric(&self) -> bool {
    match self {
      Token::Constant(_) | Token::Variable(_) => true,
      _ => false,
    }
  }
}

impl std::fmt::Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Token::Constant(n) => write!(f, "const::{n}"),
      Token::Variable(s) => write!(f, "var::{s}"),
      Token::Function(s) => write!(f, "func::{s}"),
      Token::Operator { op, .. } => write!(f, "op::{op}"),
    }
  }
}

pub fn tokenize(
  op_set: OperatorSet,
  lexemes: Vec<Lexeme>,
) -> Result<Vec<Token>, TokenError> {
  let mut wrappings: Vec<Wrapping> = vec![];
  let mut tokens = vec![];
  for index in 0..lexemes.len() {
    match &lexemes[index] {
      // Parse number
      Lexeme::Number(s) => {
        let n = match s.parse::<Number>() {
          Ok(n) => n,
          Err(_) => return Err(TokenError::BadNumber),
        };
        if tokens.last().is_some_and(|t: &Token| t.is_numeric()) {
          tokens.push(Token::Operator {
            op: op_set.implicit().clone(),
            wrapping: wrappings.len(),
          });
        }
        tokens.push(Token::Constant(n));
      },
      // Discriminate between functions and vars
      Lexeme::Identifier(s) => match lexemes.get(index + 1) {
        Some(Lexeme::LeftWrap(_)) => {
          // Insert implicit op
          if tokens.last().is_some_and(|t: &Token| t.is_numeric()) {
            tokens.push(Token::Operator {
              op: op_set.implicit().clone(),
              wrapping: wrappings.len(),
            });
          }
          tokens.push(Token::Function(s.clone()));
        },
        _ => {
          // Insert implicit op
          if tokens.last().is_some_and(|t: &Token| t.is_numeric()) {
            tokens.push(Token::Operator {
              op: op_set.implicit().clone(),
              wrapping: wrappings.len(),
            });
          }
          tokens.push(Token::Variable(s.clone()));
        },
      },
      // Insert operator
      Lexeme::Special(c) => {
        let op = match op_set.get(c) {
          Some(op) => op.clone(),
          None => return Err(TokenError::IllegalSymbol),
        };
        tokens.push(Token::Operator {
          op,
          wrapping: wrappings.len(),
        })
      },
      // Incrememnt wrap
      Lexeme::LeftWrap(w) => {
        // Not redundant! Makes sure wrapping level is correct
        match tokens.last() {
          Some(Token::Variable(_)) | Some(Token::Constant(_)) => {
            tokens.push(Token::Operator {
              op: op_set.implicit().clone(),
              wrapping: wrappings.len(),
            });
          },
          _ => {},
        }
        wrappings.push(*w);
      },
      Lexeme::RightWrap(w) => {
        // Check validity of closing wrap
        if wrappings.pop() != Some(*w) {
          return Err(TokenError::UnmatchedWrapping);
        }
      },
      Lexeme::Unknown(_) => return Err(TokenError::IllegalSymbol),
    }
  }
  if !wrappings.is_empty() {
    return Err(TokenError::UnmatchedWrapping);
  }
  Ok(tokens)
}
