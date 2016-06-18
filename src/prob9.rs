/// Special Pythagorean triplet
///
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
///
///     a^2 + b^2 = c^2
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

use common::Problem;

pub fn find_answer() -> (u64, u64, u64) {
  for a in 1..997 {
    for b in a + 1..1000 - a {
      let c = 1000 - a - b;
      if a * a + b * b == c * c {
        return (a, b, c);
      }
    }
  }

  panic!("Not found");
}

pub struct Problem9 {}

impl Problem for Problem9 {
  fn solve(&self) {
    let (a, b, c) = find_answer();

    println!("Answer: {}", a * b * c);
  }
}