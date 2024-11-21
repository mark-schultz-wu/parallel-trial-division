//! Factoring various semi-primes derived from Mersenne numbers.

const M13: u128 = (1 << 13) - 1;
const M17: u128 = (1 << 17) - 1;
const M19: u128 = (1 << 19) - 1;
const M31: u128 = (1 << 31) - 1;

const N0: u128 = M13 * M17;
const N1: u128 = M13 * M19;
const N2: u128 = M13 * M31;
const N3: u128 = M17 * M19;
const N4: u128 = M17 * M31;
const N5: u128 = M19 * M31;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use parallel_trial_division::{parallel_factorization, serial_factorization};

fn bench_factorization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Factorization");
    for i in [N0, N1, N2, N3, N4, N5].iter() {
        group.bench_with_input(BenchmarkId::new("Serial", i), i, |b, i| {
            b.iter(|| serial_factorization(*i))
        });
        group.bench_with_input(BenchmarkId::new("Parallel", i), i, |b, i| {
            b.iter(|| parallel_factorization(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_factorization);
criterion_main!(benches);
