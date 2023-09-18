use std::rc::Rc;

use crate::{alg::Algebra, tok::Token};

#[derive(Debug, Clone)]
pub enum AstNode {
  Some {
    token: Token,
    left: Rc<AstNode>,
    right: Rc<AstNode>,
  },
  None,
}

impl AstNode {
  pub fn children(&self) -> Option<(Rc<AstNode>, Rc<AstNode>)> {
    match self {
      AstNode::Some { left, right, .. } => Some((left.clone(), right.clone())),
      AstNode::None => None,
    }
  }

  pub const fn is_some(&self) -> bool {
    match self {
      AstNode::Some { .. } => true,
      AstNode::None => false,
    }
  }

  pub const fn is_none(&self) -> bool {
    !self.is_some()
  }

  pub fn is_leaf(&self) -> bool {
    match self {
      AstNode::Some { left, right, .. } => left.is_none() && right.is_none(),
      AstNode::None => false,
    }
  }
}

pub struct Ast {
  algebra: Algebra,
  root: Rc<AstNode>,
}

impl Ast {
  pub fn new() -> Self {
    Self {
      algebra: Algebra::default(),
      root: Rc::new(AstNode::None),
    }
  }

  pub fn should_sink(&self, a: &AstNode, b: &AstNode) -> bool {
    // Sink to the bottom if constant, variable, or none
    if a.is_none() {
      return true; // Possibly error or panic
    }
    if let AstNode::Some {
      token: Token::Constant(_) | Token::Variable(_),
      ..
    } = a
    {
      return true;
    }

    let (paren_a, prio_a): (usize, usize) = match a {
      AstNode::Some {
        token:
          Token::Operator {
            symbol,
            wrapping,
            ops,
          },
        ..
      } => todo!(),
      AstNode::Some {
        token: Token::Variable(_) | Token::Constant(_),
        ..
      } => return true,
      AstNode::None => return true,
      _ => todo!(),
    };

    todo!()
  }

  pub fn push(&mut self, node: AstNode) {
    let node = Rc::new(node);
    let mut ptr = &mut self.root;
    loop {
      if ptr.is_none() {
        *ptr = node;
        return;
      }
    }
  }
}
