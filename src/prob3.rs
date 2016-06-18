/// Largest prime factor
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143?

use common::Problem;
use common::simple_sieve;

pub fn find_answer(n: usize) -> Option<usize> {
  let rt = (n as f64).sqrt().ceil() as usize;
  let cands: Vec<usize> = simple_sieve(rt);

  match cands.iter().filter_map(|&cand| {
    match n % cand {
      0 => Some(cand),
      _ => None
    }
  }).collect::<Vec<usize>>().last() {
    Some(&p) => Some(p),
    None => None
  }
}