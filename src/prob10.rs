/// Summation of primes
///
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.

use common::Problem;
use common::simple_sieve;

pub fn find_answer(n: usize) -> usize {
  simple_sieve(n).iter().sum()
}


