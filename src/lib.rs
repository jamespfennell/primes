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

/// Prime number generator using the algorithm described in [1], but without using a hash map.
///
/// ```
/// # use primes::*;
/// let mut generator: Generator3 = Default::default();
/// assert_eq![generator.next(), Some(2)];
/// assert_eq![generator.next(), Some(3)];
/// assert_eq![generator.next(), Some(5)];
/// ```
///
/// [1] https://eli.thegreenplace.net/2023/my-favorite-prime-number-generator/
pub struct Generator3 {
    d: Vec<(u64, u64)>,
    buf: Vec<(u64, u64)>,
    q: u64,
}

impl Default for Generator3 {
    fn default() -> Self {
        Self {
            d: Default::default(),
            buf: Default::default(),
            q: 2,
        }
    }
}

impl Iterator for Generator3 {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            for (c, p) in &self.d {
                if *c != self.q {
                    break;
                }
                self.buf.push((c + p, *p));
            }
            self.q += 1;
            if self.buf.is_empty() {
                let p = self.q - 1;
                self.d.push((p * p, p));
                return Some(p);
            }
            let mut dest = 0;
            let mut src_d = self.buf.len();
            let mut src_buf = 0;
            while src_buf < self.buf.len() {
                if src_d == self.d.len() || self.buf[src_buf] < self.d[src_d] {
                    self.d[dest] = self.buf[src_buf];
                    dest += 1;
                    src_buf += 1;
                } else {
                    self.d[dest] = self.d[src_d];
                    dest += 1;
                    src_d += 1;
                }
            }
            self.buf.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_thousandth_prime_1() {
        let mut generator: Generator1 = Default::default();
        assert_eq![generator.nth(1000 - 1), Some(7919)];
    }
    #[test]
    fn the_thousandth_prime_3() {
        let mut generator: Generator3 = Default::default();
        assert_eq![generator.nth(1000 - 1), Some(7919)];
    }
}
