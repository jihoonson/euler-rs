/// Largest palindrome product
///
/// A palindromic number reads the same both ways. The largest palindrome made from the product of
/// two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn find_answer() -> Option<usize> {
  (100..1000).rev().map(|i| {
    match (100..1000).rev().filter_map(|j| {
      let n = i * j;
      if is_palindrome(n) {
        Some(n)
      } else {
        None
      }
    }).max() {
      Some(m) => m,
      None => 0
    }
  }).max()
}

pub fn is_palindrome(num: usize) -> bool {
  let str = num.to_string();
  let half_len = str.len() / 2;

  let first = &str[0..half_len];
  let second = match str.len() % 2 {
    0 => &str[half_len..str.len()],
    _ => &str[half_len + 1..str.len()]
  };

  first.chars().zip(second.chars().rev()).all(|(f, s)| f == s)
}