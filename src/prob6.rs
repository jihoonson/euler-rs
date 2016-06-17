/// Sum square difference
///
/// The sum of the squares of the first ten natural numbers is,
///
///     1^2 + 2^2 + ... + 10^2 = 385
///
/// The square of the sum of the first ten natural numbers is,
///
///     (1 + 2 + ... + 10)^2 = 55^2 = 3025
///
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

use common::Problem;

pub fn sum_of_squares(n: i32) -> i32 {
  (1..n + 1).into_iter().map(|i| i * i).sum()
}

pub fn square_of_sum(n: i32) -> i32 {
  let sum: i32 = (1..n + 1).into_iter().sum();
  sum * sum
}

pub fn find_answer(n: i32) -> i32 {
  (sum_of_squares(n) - square_of_sum(n)).abs()
}

pub struct Problem6 {}

impl Problem for Problem6 {
  fn solve(&self) {
    // Get inputs
  }
}