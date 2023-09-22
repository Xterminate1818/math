use std::collections::HashMap;

use crate::{
  context::Context,
  error::MathResult,
  number::{sanitize_result, Number},
  parse::Ast,
};

#[derive(Debug, Clone)]
pub enum Function {
  SoftwareDefined(fn(Number) -> MathResult),
  AstDefined(Ast, String),
}

impl Function {
  pub fn compute(&self, operand: MathResult) -> MathResult {
    match self {
      Function::SoftwareDefined(func) => (func)(operand?),
      Function::AstDefined(ast, var_name) => {
        let mut ctx = Context::new();
        ctx.assign_variable(var_name.to_string(), operand?);
        ast.evaluate(&ctx)
      },
    }
  }

  pub fn default_set() -> HashMap<String, Function> {
    let mut map = HashMap::new();
    map.insert("sin".to_string(), _SIN_FUNC);
    map.insert("cos".to_string(), _COS_FUNC);
    map.insert("tan".to_string(), _TAN_FUNC);
    map.insert("csc".to_string(), _CSC_FUNC);
    map.insert("sec".to_string(), _SEC_FUNC);
    map.insert("cot".to_string(), _COT_FUNC);

    map.insert("arcsin".to_string(), _ASIN_FUNC);
    map.insert("arccos".to_string(), _ACOS_FUNC);
    map.insert("arctan".to_string(), _ATAN_FUNC);

    map.insert("ln".to_string(), _LN_FUNC);
    map.insert("abs".to_string(), _ABS_FUNC);

    map.insert("sqrt".to_string(), _SQRT_FUNC);
    map
  }
}

// trig
const _SIN_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.sin()));

const _COS_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.cos()));

const _TAN_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.tan()));

const _CSC_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.sin().inv()));

const _SEC_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.cos().inv()));

const _COT_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.tan().inv()));

// inverse trig
const _ASIN_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.asin()));

const _ACOS_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.acos()));

const _ATAN_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.atan()));

// log
const _LN_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.ln()));

// abs
const _ABS_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.norm().into()));

// sqrt
const _SQRT_FUNC: Function =
  Function::SoftwareDefined(|n| sanitize_result(n.sqrt()));
