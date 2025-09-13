//! Benchmarks for the example course demonstrating performance measurement.

use courses_000_example::{
    binary_search, binary_search_optimized, linear_search, linear_search_naive,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_search_algorithms(c: &mut Criterion) {
    let sizes = vec![10, 100, 1000, 10000];

    let mut group = c.benchmark_group("search_algorithms");

    for size in sizes {
        let data: Vec<i32> = (0..size).collect();
        let target = size / 2; // Middle element

        group.bench_with_input(BenchmarkId::new("linear_naive", size), &size, |b, _| {
            b.iter(|| linear_search_naive(&data, &target))
        });

        group.bench_with_input(BenchmarkId::new("linear_idiomatic", size), &size, |b, _| {
            b.iter(|| linear_search(&data, &target))
        });

        group.bench_with_input(BenchmarkId::new("binary_search", size), &size, |b, _| {
            b.iter(|| binary_search(&data, target))
        });

        group.bench_with_input(BenchmarkId::new("binary_optimized", size), &size, |b, _| {
            b.iter(|| binary_search_optimized(&data, target))
        });
    }

    group.finish();
}

fn bench_worst_case_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("worst_case_search");

    let sizes = vec![100, 1000, 10000];

    for size in sizes {
        let data: Vec<i32> = (0..size).collect();
        let target = size + 1; // Not in array (worst case)

        group.bench_with_input(BenchmarkId::new("linear_worst", size), &size, |b, _| {
            b.iter(|| linear_search(&data, &target))
        });

        group.bench_with_input(BenchmarkId::new("binary_worst", size), &size, |b, _| {
            b.iter(|| binary_search(&data, target))
        });
    }

    group.finish();
}

fn bench_containers(c: &mut Criterion) {
    use courses_000_example::generics_and_traits::Container;

    let mut group = c.benchmark_group("containers");

    let sizes = vec![10, 100, 1000];

    for size in sizes {
        let data: Vec<i32> = (0..size).collect();
        let container = Container::from_vec(data.clone());

        group.bench_with_input(
            BenchmarkId::new("container_find", size),
            &size,
            |b, &size| {
                let target = size / 2;
                b.iter(|| container.find(&target))
            },
        );

        group.bench_with_input(BenchmarkId::new("vec_position", size), &size, |b, &size| {
            let target = size / 2;
            b.iter(|| data.iter().position(|&x| x == target))
        });
    }

    group.finish();
}

fn bench_error_handling(c: &mut Criterion) {
    use courses_000_example::error_handling::{find_with_option, find_with_result};

    let data: Vec<i32> = (0..1000).collect();

    c.bench_function("find_with_option_success", |b| {
        b.iter(|| find_with_option(&data, 500))
    });

    c.bench_function("find_with_result_success", |b| {
        b.iter(|| find_with_result(&data, 500))
    });

    c.bench_function("find_with_option_failure", |b| {
        b.iter(|| find_with_option(&data, 1001))
    });

    c.bench_function("find_with_result_failure", |b| {
        b.iter(|| find_with_result(&data, 1001))
    });
}

criterion_group!(
    benches,
    bench_search_algorithms,
    bench_worst_case_search,
    bench_containers,
    bench_error_handling
);
criterion_main!(benches);
