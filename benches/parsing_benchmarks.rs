//! Performance benchmarks for FLEX parsing

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ib_flex::parse_activity_flex;

fn benchmark_minimal_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_minimal.xml");

    c.bench_function("parse_minimal_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_options_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_options.xml");

    c.bench_function("parse_options_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_futures_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_futures.xml");

    c.bench_function("parse_futures_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_forex_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_forex.xml");

    c.bench_function("parse_forex_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_bonds_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_bonds.xml");

    c.bench_function("parse_bonds_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_corporate_actions_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_corporate_actions.xml");

    c.bench_function("parse_corporate_actions_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_cash_transactions_parsing(c: &mut Criterion) {
    let xml = include_str!("../tests/fixtures/activity_cash.xml");

    c.bench_function("parse_cash_statement", |b| {
        b.iter(|| {
            let result = parse_activity_flex(black_box(xml));
            assert!(result.is_ok());
            result.unwrap()
        })
    });
}

fn benchmark_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");

    // Minimal (1 trade)
    let xml_1 = include_str!("../tests/fixtures/activity_minimal.xml");
    group.bench_with_input(BenchmarkId::from_parameter(1), &xml_1, |b, xml| {
        b.iter(|| parse_activity_flex(black_box(xml)))
    });

    // Options (4 trades)
    let xml_4 = include_str!("../tests/fixtures/activity_options.xml");
    group.bench_with_input(BenchmarkId::from_parameter(4), &xml_4, |b, xml| {
        b.iter(|| parse_activity_flex(black_box(xml)))
    });

    // Corporate actions (8 actions + 15 cash txns)
    let xml_23 = include_str!("../tests/fixtures/activity_cash.xml");
    group.bench_with_input(BenchmarkId::from_parameter(15), &xml_23, |b, xml| {
        b.iter(|| parse_activity_flex(black_box(xml)))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_minimal_parsing,
    benchmark_options_parsing,
    benchmark_futures_parsing,
    benchmark_forex_parsing,
    benchmark_bonds_parsing,
    benchmark_corporate_actions_parsing,
    benchmark_cash_transactions_parsing,
    benchmark_scalability
);
criterion_main!(benches);
