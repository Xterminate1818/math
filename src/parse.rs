use crate::lex::*;

pub enum AstNode {
  Leaf(f64),
  BinOp {
    op: BinOp,
    paren_level: Precedence,
    left: Box<AstNode>,
    right: Box<AstNode>,
  },
}

impl PartialEq for AstNode {
    fn eq(&self, other: &Self) -> bool {
        match self {
            AstNode::Leaf(_) => match other {
                AstNode::Leaf(_) => true,
                AstNode::BinOp{..} => false,
            },
            AstNode::BinOp { op: rh_op, paren_level: rh_paren, ..} => match other {
                AstNode::Leaf(_) => false,
                AstNode::BinOp { op: lh_op, paren_level: lh_paren, ..} => (rh_op == lh_op && rh_paren == lh_paren),
            },
        }
    }
}

impl PartialOrd for AstNode {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let prio_1 = match self {
        AstNode::Leaf(_) => BASE_PRECEDENCE,
        AstNode::BinOp { op, paren_level, left, right } => op.into::<Precedence>() + paren_level ,
    }
    todo!()
  }
}
