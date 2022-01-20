use criterion::{criterion_group, criterion_main, Criterion};
use std::io;
use std::io::prelude::*;

use practice::*;

fn bench_score(c: &mut Criterion) {}

criterion_group!(benches, bench_score);
criterion_main!(benches);
