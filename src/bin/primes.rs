fn main() {
    let range = 0..200;
    let data: Vec<(usize, u128)> = range
        .into_iter()
        .map(|i| {
            let step = 5000;
            let n = i * step;
            let start = std::time::Instant::now();
            let mut generator: primes::Generator3 = Default::default();
            generator.nth(n);
            (n, start.elapsed().as_micros())
        })
        .collect();
    for (n, t) in data {
        println!("{},{}", n, t);
    }
}
