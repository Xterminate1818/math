use std::rc::Rc;

use crate::tok::Token;

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

z
