use std::{iter::Peekable, str::Chars};

use super::token::*;

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
      '+' => tokens.push(Token::Add),
      '-' => tokens.push(Token::Sub),
      '*' => tokens.push(Token::Mul),
      '/' => tokens.push(Token::Div),
      _ => {
        panic!("Invalid character: {}", c)
      },
    }
  }
  if let Some(start) = start_of_num {
    tokens.push(parse_substr(&input, start, input.len()));
  }
  tokens
}

enum CharType {
  Number,
  Letter,
  Symbol,
}

impl CharType {
  const SYMBOLS_LIST: [char; 4] = ['+', '-', '*', '/'];

  pub fn of(c: char) -> Option<Self> {
    if c.is_ascii_digit() {
      Some(Self::Number)
    } else if c.is_ascii() {
      Some(Self::Letter)
    } else if Self::SYMBOLS_LIST.contains(&c) {
      Some(Self::Symbol)
    } else {
      None
    }
  }
}

pub struct TokenStream<'a> {
  iterator: Peekable<Chars<'a>>,
}

impl<'a> TokenStream<'a> {
  pub fn next(&mut self) -> Option<Token> {
    let substr = iterator.as_str();
    let token_type = CharType::of(self.iterator.next()?);
    let mut index = 1;
    loop {
      let current_char = self.iterator.peek();
      break;
    }
    todo!();
  }
}
