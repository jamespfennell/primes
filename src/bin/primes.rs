use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

fn main() {
    let range = 1..=1000;
    let mut ns: Vec<usize> = range.into_iter().map(|i| vec![i; 3]).flatten().collect();
    let mut rng = thread_rng();
    let index : std::sync::Mutex<usize> = Default::default();
    ns.shuffle(&mut rng);
    let num_samples = ns.len();
    ns.par_iter().for_each(|i| {
        let index = {
            let mut l = index.lock().unwrap();
            let index = *l;
            *l +=1;
            index
        };
        let step = 10000;
        let n = i * step;
        eprintln!("{}/{} n={}", index+1, num_samples, n);
        let start = std::time::Instant::now();
        let mut generator: primes::Generator3 = Default::default();
        generator.nth(n);
        let t = start.elapsed().as_micros();
        println!("{},{}", n, t);
        eprintln!("{}/{} n={}, {}s", index+1, num_samples, n, start.elapsed().as_secs());
    });
}
