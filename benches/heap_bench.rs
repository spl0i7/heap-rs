use criterion::{criterion_group, criterion_main, Criterion};
use heap::Heap;
use heap::HeapKind::MinHeap;
use rand::RngCore;

fn criterion_benchmark(c: &mut Criterion) {
    let mut min_heap = Heap::new(MinHeap);
    let mut r = rand::thread_rng();
    for _ in 0..10000 {
        min_heap.insert(r.next_u32());
    }

    c.bench_function("heap insert 10000", |b| b.iter(||
        for _ in 0..1000 {
            min_heap.pop();
        }
    ));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
