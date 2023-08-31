use criterion::{criterion_group, criterion_main, Criterion};
use primes::*;

pub fn generator_1(c: &mut Criterion) {
    c.bench_function("generator1", |b| {
        b.iter(|| {
            let mut generator: Generator1 = Default::default();
            generator.nth(10000);
        })
    });
}

pub fn generator_2(c: &mut Criterion) {
    c.bench_function("generator2", |b| {
        b.iter(|| {
            let mut generator: Generator2 = Default::default();
            generator.nth(10000);
        })
    });
}

pub fn generator_3(c: &mut Criterion) {
    c.bench_function("generator3", |b| {
        b.iter(|| {
            let mut generator: Generator3 = Default::default();
            generator.nth(10000);
        })
    });
}

criterion_group!(benches, generator_1, generator_2, generator_3);
criterion_main!(benches);
