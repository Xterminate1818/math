use num::Complex;

use crate::error::{MathError, MathResult};

pub type Number = Complex<f64>;

pub fn round_num(mut num: Number) -> Number {
  if num.re.fract().abs() < f64::EPSILON {
    num.re = num.re.trunc();
  }
  if num.im.fract().abs() < f64::EPSILON {
    num.im = num.im.trunc();
  }
  num
}

pub fn sanitize_result(num: Number) -> MathResult {
  if num.is_nan() {
    Err(MathError::Undefined)
  } else if num.is_infinite() {
    Err(MathError::TooBig)
  } else {
    Ok(round_num(num))
  }
}

pub fn result_to_string(result: MathResult) -> String {
  match result {
    Ok(num) => {
      if num.im == 0.0 {
        return num.re.to_string();
      } else if num.re != 0.0 && num.im != 0.0 {
        return num.to_string();
      } else {
        return format!("{}i", num.im);
      }
    },
    Err(err) => "".to_string(),
  }
}
