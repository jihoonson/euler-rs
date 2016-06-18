#![feature(iter_arith)]
extern crate num;

pub mod common;
pub mod prob1;
pub mod prob2;
pub mod prob3;
pub mod prob5;
pub mod prob6;
pub mod prob7;
pub mod prob8;
pub mod prob9;
pub mod prob10;

#[cfg(test)]
mod tests {

  #[test]
  fn test_probl1() {
    use prob1::*;

    assert_eq!(18, sum_of_mul(10, 3));
    assert_eq!(23, sum_of_mul_of_vals(10, 3, 5));
    assert_eq!(233168, sum_of_mul_of_vals(1000, 3, 5));
  }

  #[test]
  fn test_probl2() {
    use prob2::*;

    assert_eq!(0, next_fib(None, None));
    assert_eq!(1, next_fib(Some(0), None));
    assert_eq!(1, next_fib(Some(0), Some(1)));
    assert_eq!(2, next_fib(Some(1), Some(1)));
  }

  #[test]
  fn test_probl3() {
    use prob3::*;

    assert_eq!(Some(29), find_answer(13195));
    assert_eq!(Some(6857), find_answer(600851475143));
  }

  #[test]
  fn test_probl5() {
    use prob5::*;

    assert_eq!(2520, find_answer(10));
    assert_eq!(232792560, find_answer(20));
  }

  #[test]
  fn test_probl6() {
    use prob6::*;

    assert_eq!(385, sum_of_squares(10));
    assert_eq!(3025, square_of_sum(10));
    assert_eq!(2640, find_answer(10));
  }

  #[test]
  fn test_probl7() {
    use prob7::*;

    assert_eq!(13, find_answer(6));
    assert_eq!(104743, find_answer(10001));
  }

  #[test]
  fn test_probl8() {
    use prob8::*;

    assert_eq!(5832, find_answer(4));
    assert_eq!(23514624000, find_answer(13));
  }

  #[test]
  fn test_probl9() {
    use prob9::*;

    let (a, b, c) = find_answer();
    assert_eq!(1000, a + b + c);
    assert_eq!(c * c, a * a + b * b);
  }

  #[test]
  fn test_probl10() {
    use prob10::*;

    assert_eq!(17, find_answer(10));
    assert_eq!(77, find_answer(20));
    assert_eq!(142913828922, find_answer(2_000_000));
  }
}
