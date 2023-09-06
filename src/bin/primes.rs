use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).expect("must provide a command").as_str();
    match command {
        "ops" => ops(),
        "timings1" => timings::<primes::Generator1>(),
        "timings3" => timings::<primes::Generator3>(),
        _ => panic!("command must be one of: ops, timings1, timings3'"),
    }
}

fn timings<T: Iterator<Item=u64> + Default>() {
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
        let mut generator: T = Default::default();
        generator.nth(n);
        let t = start.elapsed().as_micros();
        println!("{},{}", n, t);
        eprintln!("{}/{} n={}, {}s", index+1, num_samples, n, start.elapsed().as_secs());
    });
}

fn ops() {
    let mut generator: primes::Generator3 = Default::default();
    let max = 100_000_000;
    for n in 1..=max {
        let p = generator.next().unwrap();
        let num_ops = generator.num_ops();
        if n % 10_000 == 0 {
            eprintln!("n={}/{}, p={}", n, max, p);
            println!("{},{}", n, num_ops);
        }
    }
}
