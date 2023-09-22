use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wrapping {
  Paren,
  Curly,
  Square,
}

#[derive(Clone, PartialEq)]
pub enum Lexeme {
  Number(String),
  Identifier(String),
  Special(char),
  LeftWrap(Wrapping),
  RightWrap(Wrapping),
  Unknown(char),
}

impl std::fmt::Debug for Lexeme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Lexeme::Number(s) => write!(f, "Number::{}", s),
      Lexeme::Identifier(s) => write!(f, "Ident::{}", s),
      Lexeme::Special(c) => write!(f, "Special::{}", c),
      Lexeme::LeftWrap(w) => write!(
        f,
        "LWrap::{}",
        match w {
          Wrapping::Paren => '(',
          Wrapping::Curly => '{',
          Wrapping::Square => '[',
        }
      ),

      Lexeme::RightWrap(w) => write!(
        f,
        "RWrap::{}",
        match w {
          Wrapping::Paren => ')',
          Wrapping::Curly => '}',
          Wrapping::Square => ']',
        }
      ),
      Lexeme::Unknown(c) => write!(f, "?::{}", c),
    }
  }
}

/// Matches to grouping lexeme if applicable
#[inline]
fn match_wrapping_char(c: char) -> Option<Lexeme> {
  Some(match c {
    '(' => Lexeme::LeftWrap(Wrapping::Paren),
    ')' => Lexeme::RightWrap(Wrapping::Paren),
    '{' => Lexeme::LeftWrap(Wrapping::Curly),
    '}' => Lexeme::RightWrap(Wrapping::Curly),
    '[' => Lexeme::LeftWrap(Wrapping::Square),
    ']' => Lexeme::RightWrap(Wrapping::Square),
    _ => return None,
  })
}

/// Collects contiguous chars using closure
#[inline]
fn grab_while(
  char_stream: &mut Peekable<Chars<'_>>,
  closure: impl Fn(&char) -> bool,
) -> String {
  let mut word: Vec<char> = vec![];
  while let Some(next) = char_stream.peek() {
    if !closure(next) {
      break;
    }
    word.push(next.clone());
    char_stream.next();
  }
  word.into_iter().collect::<String>()
}

pub fn lex(input: String) -> Vec<Lexeme> {
  let mut lexemes = vec![];
  let mut char_stream = input.chars().peekable();
  while let Some(next) = char_stream.peek() {
    let next = next.clone();
    // Operator
    if Lexeme::SPECIAL_CHARS.contains(&next) {
      lexemes.push(Lexeme::Special(next));
    }
    // Identifier
    else if next.is_ascii_alphabetic() {
      let word = grab_while(&mut char_stream, |c| c.is_alphanumeric());
      lexemes.push(Lexeme::Identifier(word));
      continue;
    }
    // Number
    else if next.is_ascii_digit() {
      let word =
        grab_while(&mut char_stream, |c| c.is_ascii_digit() || *c == '.');
      lexemes.push(Lexeme::Number(word));
      continue;
    }
    // Wrapping
    else if let Some(grp) = match_wrapping_char(next) {
      lexemes.push(grp);
    }
    // Whitespace
    else if next.is_whitespace() {
      // Skip
    }
    // Unrecognized char
    else {
      lexemes.push(Lexeme::Unknown(next));
    }
    char_stream.next();
  }
  lexemes
}

impl Lexeme {
  const SPECIAL_CHARS: [char; 19] = [
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '=', '|', ':',
    '/', '?', '<', '>',
  ];
}
