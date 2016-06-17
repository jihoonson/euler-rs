/// 10001st prime
///
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
/// What is the 10001st prime number?

use common::Problem;

pub fn find_answer(n: usize) -> i32 {
  let mut primes: Vec<i32> = vec![2, 3, 5, 7 ,11, 13];

  if n < primes.len() {
    return primes[n];
  }

  let mut cand = 14;

  while primes.len() < n {
    if primes.iter().all(|p| cand % *p != 0) {
      primes.push(cand);
    }

    cand += 1;
  }

  *primes.last().unwrap()
}