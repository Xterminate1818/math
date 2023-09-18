// use crate::lex::*;
//
// /// Level of nested parens
// /// ((())) = 3
// pub type ParenLevel = usize;
//
// #[derive(Debug, Clone)]
// enum AstNode {
//   Leaf {
//     val: f64,
//     paren: ParenLevel,
//   },
//   BinOp {
//     op: BinOp,
//     paren: ParenLevel,
//     left: Box<AstNode>,
//     right: Box<AstNode>,
//   },
//   Empty,
// }
//
// impl AstNode {
//   fn paren_level(&self) -> ParenLevel {
//     *match self {
//       Self::Leaf { paren: pa, .. } => pa,
//       Self::BinOp { paren: pa, .. } => pa,
//       Self::Empty => &EVAL_LAST,
//     }
//   }
//
//   fn is_empty(&self) -> bool {
//     match self {
//       Self::Empty => true,
//       _ => false,
//     }
//   }
//
//   fn children(&self) -> Option<(&Box<AstNode>,
// &Box<AstNode>)> {     match self {
//       Self::Leaf { .. } | Self::Empty => None,
//       Self::BinOp { left, right, .. } => Some((&left,
// &right)),     }
//   }
//
//   fn children_mut(&mut self) -> Option<(&mut
// Box<AstNode>, &mut Box<AstNode>)> {     match self {
//       Self::Leaf { .. } | Self::Empty => None,
//       Self::BinOp {
//         ref mut left,
//         ref mut right,
//         ..
//       } => Some((left, right)),
//     }
//   }
//
//   fn solve(&self) -> Result<f64, ()> {
//     match self {
//       AstNode::Leaf { val: n, .. } => return Ok(*n),
//       AstNode::BinOp {
//         op, left, right, ..
//       } => {
//         let a = left.solve()?;
//         let b = right.solve()?;
//         return Ok(op.perform(a, b));
//       },
//       AstNode::Empty => Err(()),
//     }
//   }
// }
//
// impl Into<EvalOrder> for &AstNode {
//   fn into(self) -> EvalOrder {
//     match self {
//       AstNode::Leaf { .. } | AstNode::Empty => EVAL_LAST,
//       AstNode::BinOp { op, .. } => op.into(),
//     }
//   }
// }
//
// impl PartialEq for AstNode {
//   fn eq(&self, other: &Self) -> bool {
//     match self {
//       AstNode::Leaf { paren: pa1, .. } => match other {
//         AstNode::Leaf { paren: pa2, .. } => pa1 == pa2,
//         _ => false,
//       },
//       AstNode::BinOp {
//         op: op1,
//         paren: pa1,
//         ..
//       } => match other {
//         AstNode::BinOp {
//           op: op2,
//           paren: pa2,
//           ..
//         } => (op1 == op2) && (pa1 == pa2),
//         _ => false,
//       },
//       AstNode::Empty => match other {
//         AstNode::Empty => true,
//         _ => false,
//       },
//     }
//   }
// }
//
// impl PartialOrd for AstNode {
//   fn partial_cmp(&self, other: &Self) ->
// Option<std::cmp::Ordering> {     let paren_nesting1 =
// self.paren_level();     let paren_nesting2 =
// other.paren_level();     if paren_nesting1 !=
// paren_nesting2 {       return
// paren_nesting1.partial_cmp(&paren_nesting2);     }
//
//     let order1: EvalOrder = self.into();
//     let order2: EvalOrder = other.into();
//     order1.partial_cmp(&order2)
//   }
// }
//
// #[derive(Debug)]
// pub struct AST(Box<AstNode>);
//
// impl AST {
//   pub fn new(sequence: Vec<Token>) -> Self {
//     let mut this = Self(Box::new(AstNode::Empty));
//     let mut paren: ParenLevel = 0;
//     for token in sequence {
//       let node = match token {
//         Token::Num(val) => AstNode::Leaf { val, paren },
//         Token::BinOp(op) => AstNode::BinOp {
//           op,
//           paren,
//           left: Box::new(AstNode::Empty),
//           right: Box::new(AstNode::Empty),
//         },
//         Token::LParen => {
//           paren += 1;
//           continue;
//         },
//         Token::RParen => {
//           paren -= 1;
//           continue;
//         },
//         Token::EOE => break,
//       };
//       this.push(Box::new(node));
//     }
//     this
//   }
//
//   fn push(&mut self, mut node: Box<AstNode>) {
//     let mut ptr = &mut self.0;
//     loop {
//       if ptr.is_empty() {
//         // Ptr is a placeholder, replace with node
//         *ptr = node;
//         break;
//       } else if node <= *ptr {
//         // Node has lower precedence, node becomes parent
// of ptr         let (left, right) =
// node.children_mut().unwrap();         *right =
// Box::new(AstNode::Empty);         *left = ptr.clone();
//         *ptr = node;
//         break;
//       } else {
//         // Node has higher precedence, continue down tree
//         let (_, right) = ptr.children_mut().unwrap();
//         ptr = right;
//       }
//     }
//   }
//
//   pub fn evaluate(&self) -> Result<f64, ()> {
//     self.0.solve()
//   }
// }
