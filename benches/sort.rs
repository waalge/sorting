use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use rand::{thread_rng, seq::SliceRandom};

use sorting::sort;

fn get_data() -> Vec<usize> {
    let mut src : Vec<usize> = (0..2000000).collect();
    let mut rng = thread_rng();
    src.shuffle(&mut rng);
    src
}

fn sort(c: &mut Criterion) {
    let algos = vec!["merge", "quick"];
    for x in algos {
        c.bench_function(&x, move |b| {
            b.iter_batched(|| get_data(), |data| sort::run(&x,data), BatchSize::SmallInput)
        });
    }
}

criterion_group!(benches, sort);
criterion_main!(benches);
