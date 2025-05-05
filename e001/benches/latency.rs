use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use e001::orderbook::{OrderBook, Side};
use rust_decimal::Decimal;

use e001::btree::BTreeBook;
use e001::hashmap::HashMapBook;
use e001::hybrid::HybridBook;

fn setup_book<T: OrderBook>(mut book: T) -> T {
    for i in 0..1000 {
        book.insert(Side::Bid, Decimal::from(1000 - i), Decimal::from(i + 1));
    }
    book
}

// Insert benchmark
fn bench_insert<T: OrderBook>(c: &mut Criterion, name: &str, mut make_book: impl FnMut() -> T) {
    let quantity = Decimal::from(10);

    c.bench_function(&format!("{} insert", name), |b| {
        b.iter_batched(
            || setup_book(make_book()),
            |mut book| {
                // Insert in the middle of the book
                for i in 0..100 {
                    book.insert(Side::Bid, Decimal::from(1000 + i), quantity);
                }
                black_box(book);
            },
            BatchSize::SmallInput,
        )
    });
}

// Modify benchmark
fn bench_modify<T: OrderBook + Clone>(
    c: &mut Criterion,
    name: &str,
    mut make_book: impl FnMut() -> T,
) {
    let quantity = Decimal::from(10);

    let book = setup_book(make_book());

    c.bench_function(&format!("{} modify", name), |b| {
        b.iter_batched(
            || book.clone(),
            |mut book| {
                for i in 0..100 {
                    book.insert(Side::Bid, Decimal::from(100 + i), quantity);
                }
                black_box(book);
            },
            BatchSize::SmallInput,
        )
    });
}

// Delete benchmark
fn bench_delete<T: OrderBook>(c: &mut Criterion, name: &str, mut make_book: impl FnMut() -> T) {
    c.bench_function(&format!("{} delete", name), |b| {
        b.iter_batched(
            || setup_book(make_book()),
            |mut book| {
                // Delete middle of the book
                for i in 0..100 {
                    book.delete(Side::Bid, Decimal::from(100 + i));
                }
                black_box(book);
            },
            BatchSize::SmallInput,
        )
    });
}

// Top benchmark
fn bench_top<T: OrderBook>(c: &mut Criterion, name: &str, mut make_book: impl FnMut() -> T) {
    c.bench_function(&format!("{} top", name), |b| {
        b.iter_batched(
            || setup_book(make_book()),
            |book| {
                for _ in 0..100 {
                    book.top();
                }
                black_box(book);
            },
            BatchSize::SmallInput,
        )
    });
}

// Bids benchmark
fn bench_bids<T: OrderBook>(c: &mut Criterion, name: &str, mut make_book: impl FnMut() -> T) {
    c.bench_function(&format!("{} bids", name), |b| {
        b.iter_batched(
            || setup_book(make_book()),
            |book| {
                for _ in 0..100 {
                    let _: Vec<_> = book.bids().collect();
                }

                black_box(book);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_all<T: OrderBook + Clone>(
    c: &mut Criterion,
    name: &str,
    mut make_book: impl FnMut() -> T,
) {
    bench_insert(c, name, &mut make_book);
    bench_modify(c, name, &mut make_book);
    bench_delete(c, name, &mut make_book);
    bench_top(c, name, &mut make_book);
    bench_bids(c, name, &mut make_book);
}

// Register all benchmarks for each implementation
fn criterion_benchmark(c: &mut Criterion) {
    bench_all(c, "BTreeBook", BTreeBook::new);
    bench_all(c, "HashMapBook", HashMapBook::new);
    bench_all(c, "HybridBook", HybridBook::new);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
