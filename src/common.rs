/// Common library

pub trait Problem {
  fn solve(&self);
}

pub fn simple_sieve(n: usize) -> Vec<usize> {

  if n < 2 {
    return Vec::new();
  }

  let mut is_prime = vec![true; n + 1];
  is_prime[0] = false; // keep positions for 0 and 1 for direct access
  is_prime[1] = false;

  for cand in 2..n + 1 {
    if is_prime[cand] {
      let mut multiple = cand * cand;
      while multiple <= n {
        is_prime[multiple] = false;
        multiple += cand;
      }
    }
  }

  is_prime.iter().enumerate()
      .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
      .collect()
}