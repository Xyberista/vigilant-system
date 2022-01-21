use criterion::{criterion_group, criterion_main, Criterion};

use lib::*;

fn bench_score(c: &mut Criterion) {
    let path = "./input/e_elaborate.in.txt";
    let (clients, addable, _removeable): (Vec<Client>, Ing, Ing) = parse_input(&path).unwrap();
    c.bench_function("score", |b| b.iter(|| score(&clients, &addable)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(250);
    targets = bench_score
);
criterion_main!(benches);
