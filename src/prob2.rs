/// Even Fibonacci numbers
///
/// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
///
///     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

use common::Problem;

pub fn next_fib(a: Option<i32>, b: Option<i32>) -> i32 {
  match a {
    Some(x) => {
      match b {
        Some(y) => {
          x + y
        },
        None => {
          assert_eq!(0, x);
          1
        }
      }
    },
    None => 0
  }
}

pub fn even_sum(n: i32) -> i32 {
  let mut prev1 = None;
  let mut prev2 = None;
  let mut f = next_fib(prev1, prev2);
  let mut sum = f;

  while f <= n {
    if f % 2 == 0 {
      sum += f;
    }
    prev2 = prev1;
    prev1 = Some(f);
    f = next_fib(prev1, prev2);
  }

  sum
}

pub struct Problem2 {}

impl Problem for Problem2 {
  fn solve(&self) {
    // Get inputs

    let n = 4_000_000;

    println!("sum: {}", even_sum(n));
  }
}