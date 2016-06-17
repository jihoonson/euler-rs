/// Smallest multiple
///
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use common::Problem;
use num::Integer;

pub fn find_answer(n: i32) -> i32 {
  let mut ans = 1;

  for i in 2..(n + 1) { // include n
    if ans % i != 0 {
      ans *= i / ans.gcd(&i);
    }
  }
  ans
}

pub struct Problem5 {}

impl Problem for Problem5 {
  fn solve(&self) {
    // Get inputs

    let n = 10;


  }
}