//! A collection of prime number generators.
use std::collections::HashMap;

/// Prime number generator using the algorithm described in [1].
///
/// ```
/// # use primes::*;
/// let mut generator: Generator1 = Default::default();
/// assert_eq![generator.next(), Some(2)];
/// assert_eq![generator.next(), Some(3)];
/// assert_eq![generator.next(), Some(5)];
/// ```
///
/// [1] https://eli.thegreenplace.net/2023/my-favorite-prime-number-generator/
pub struct Generator1 {
    d: HashMap<u64, Vec<u64>>,
    q: u64,
}

impl Default for Generator1 {
    fn default() -> Self {
        Self {
            d: Default::default(),
            q: 2,
        }
    }
}

impl Iterator for Generator1 {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.d.remove(&self.q) {
                None => {
                    self.d.insert(self.q * self.q, vec![self.q]);
                    self.q += 1;
                    return Some(self.q - 1);
                }
                Some(v) => {
                    for p in v {
                        self.d.entry(p + self.q).or_insert(vec![]).push(p);
                    }
                    self.q += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_thousandth_prime() {
        let mut generator: Generator1 = Default::default();
        assert_eq![generator.nth(1000 - 1), Some(7919)];
    }
}
