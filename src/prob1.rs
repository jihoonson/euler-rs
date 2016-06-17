/// Multiples of 3 and 5
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.

use common::Problem;
use num::Integer;

pub fn sum_of_mul(n: i32, k: i32) -> i32 {
  let max : i32 = (n as f32 / k as f32).ceil() as i32;
  k * (max - 1) * max / 2
}

pub fn sum_of_mul_of_vals(n: i32, k: i32, m: i32) -> i32 {
  sum_of_mul(n, k) + sum_of_mul(n, m) - sum_of_mul(n, k.lcm(&m))
}

pub struct Problem1 {}

impl Problem for Problem1 {
  fn solve(&self) {
    // Get inputs
  }
}