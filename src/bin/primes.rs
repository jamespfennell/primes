use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let range = 1..=100;
    let mut ns: Vec<usize> = range.into_iter().map(|i| vec![i; 3]).flatten().collect();
    let mut rng = thread_rng();
    ns.shuffle(&mut rng);
    let data: Vec<(usize, u128)> = ns
        .into_iter()
        .map(|i| {
            let step = 10000;
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
