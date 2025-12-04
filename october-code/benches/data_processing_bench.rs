use criterion::{black_box, criterion_group, criterion_main, Criterion};
use performance_optimization_demo::{optimized, unoptimized};
use rand::Rng;

fn generate_test_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(-1000..=1000))
        .collect()
}

fn bench_calculate_average(c: &mut Criterion) {
    let data = generate_test_data(10000);
    
    let mut group = c.benchmark_group("calculate_average");
    
    group.bench_function("unoptimized", |b| {
        b.iter(|| unoptimized::calculate_average(black_box(&data)))
    });
    
    group.bench_function("optimized", |b| {
        b.iter(|| optimized::calculate_average(black_box(&data)))
    });
    
    group.finish();
}

fn bench_find_most_frequent(c: &mut Criterion) {
    let data = generate_test_data(10000);
    
    let mut group = c.benchmark_group("find_most_frequent");
    
    group.bench_function("unoptimized", |b| {
        b.iter(|| unoptimized::find_most_frequent(black_box(&data)))
    });
    
    group.bench_function("optimized", |b| {
        b.iter(|| optimized::find_most_frequent(black_box(&data)))
    });
    
    group.finish();
}

fn bench_filter_and_transform(c: &mut Criterion) {
    let data = generate_test_data(10000);
    
    let mut group = c.benchmark_group("filter_and_transform");
    
    group.bench_function("unoptimized", |b| {
        b.iter(|| unoptimized::filter_and_transform(black_box(&data)))
    });
    
    group.bench_function("optimized", |b| {
        b.iter(|| optimized::filter_and_transform(black_box(&data)))
    });
    
    group.finish();
}

fn bench_process_strings(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).collect();
    
    let mut group = c.benchmark_group("process_strings");
    
    group.bench_function("unoptimized", |b| {
        b.iter(|| unoptimized::process_strings(black_box(&data)))
    });
    
    group.bench_function("optimized", |b| {
        b.iter(|| optimized::process_strings(black_box(&data)))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_calculate_average,
    bench_find_most_frequent,
    bench_filter_and_transform,
    bench_process_strings
);
criterion_main!(benches);

