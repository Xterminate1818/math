use std::cmp::Ordering;

use crate::{
  context::Context,
  error::{MathError, MathResult},
  operator::{Associativity, Operation},
  token::Token,
};

#[derive(Clone)]
pub enum AstNode {
  Some {
    token: Token,
    left: Box<AstNode>,
    right: Box<AstNode>,
  },
  None,
}

impl std::fmt::Debug for AstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AstNode::Some { token, left, right } => {
        write!(f, "{:?}({:?}, {:?})", token, left, right)
      },
      AstNode::None => write!(f, ""),
    }
  }
}

impl AstNode {
  pub fn new(token: Token) -> Self {
    Self::Some {
      token,
      left: Box::new(AstNode::None),
      right: Box::new(AstNode::None),
    }
  }

  pub fn children(&self) -> Option<(&Box<AstNode>, &Box<AstNode>)> {
    match self {
      Self::Some {
        ref left,
        ref right,
        ..
      } => Some((left, right)),
      Self::None => None,
    }
  }

  pub fn children_mut(
    &mut self,
  ) -> Option<(&mut Box<AstNode>, &mut Box<AstNode>)> {
    match self {
      Self::Some {
        ref mut left,
        ref mut right,
        ..
      } => Some((left, right)),
      Self::None => None,
    }
  }

  pub const fn is_some(&self) -> bool {
    match self {
      Self::Some { .. } => true,
      Self::None => false,
    }
  }

  pub const fn is_none(&self) -> bool {
    !self.is_some()
  }

  pub fn is_leaf(&self) -> bool {
    match self {
      Self::Some { left, right, .. } => left.is_none() && right.is_none(),
      Self::None => false,
    }
  }

  fn precedence(&self) -> (usize, usize) {
    match self {
      Self::Some {
        token: Token::Operator { op, wrapping },
        ..
      } => (*wrapping, op.precedence),
      Self::Some {
        token: Token::Function { .. },
        ..
      } => (0, usize::MAX),
      _ => (usize::MAX, usize::MAX),
    }
  }

  pub fn compare(&self, other: &Self) -> Ordering {
    let ordering = self.precedence().cmp(&other.precedence());
    if ordering == Ordering::Equal {
      if let Self::Some {
        token:
          Token::Operator {
            op: Operation { associativity, .. },
            ..
          },
        ..
      } = self
      {
        return match associativity {
          Associativity::Left => Ordering::Less,
          Associativity::Right => Ordering::Greater,
        };
      }
    }
    ordering
  }

  pub fn evaluate(&self, ctx: &Context) -> MathResult {
    let (token, left, right) = match self {
      AstNode::Some { token, left, right } => (token, left, right),
      AstNode::None => return Err(MathError::Undefined),
    };
    match token {
      Token::Constant(c) => Ok(*c),
      Token::Variable(v) => ctx.read_variable(v.to_string()),
      Token::Function(f) => {
        // Check if parsing messed up somewhere
        if left.is_some() {
          return Err(MathError::Undefined);
        }
        ctx.compute_function(f.to_string(), right.evaluate(ctx))
      },
      Token::Operator { op, .. } => {
        match (left.evaluate(ctx), right.evaluate(ctx)) {
          (Ok(a), Ok(b)) => op.perform_binary(a, b),
          (Err(_), Ok(a)) => op.perform_unary(a),
          _ => Err(MathError::Undefined),
        }
      },
    }
  }
}

#[derive(Debug, Clone)]
pub struct Ast {
  pub root: Box<AstNode>,
}

impl Ast {
  pub fn new(tokens: Vec<Token>) -> Self {
    let mut this = Self {
      root: Box::new(AstNode::None),
    };
    for t in tokens {
      this.push(AstNode::new(t));
    }
    this
  }

  fn swim(old: &mut Box<AstNode>, mut new: Box<AstNode>) {
    let (left, right) = new.children_mut().unwrap();
    *right = Box::new(AstNode::None);
    *left = old.clone();
    *old = new;
  }

  fn sink(mut ptr: &mut Box<AstNode>) -> &mut Box<AstNode> {
    let (_, right) = ptr.children_mut().unwrap();
    ptr = right;
    ptr
  }

  pub fn evaluate(&self, ctx: &Context) -> MathResult {
    if let AstNode::None = *self.root {
      return Err(MathError::NoInput);
    }
    self.root.evaluate(ctx)
  }

  pub fn push(&mut self, node: AstNode) {
    if node.is_none() {
      return;
    }
    let node = Box::new(node);
    let mut ptr = &mut self.root;
    loop {
      // Replace
      if ptr.is_none() {
        *ptr = node;
        return;
      }

      match node.compare(ptr) {
        // Swim
        Ordering::Less => {
          Self::swim(ptr, node);
          return;
        },
        // Sink
        Ordering::Equal | Ordering::Greater => {
          ptr = Self::sink(ptr);
        },
      }
    }
  }
}
