use criterion::{Criterion, criterion_group, criterion_main};
use e002::fp::Fp;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::hint::black_box;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

const TEST_DATA: &[u8] = br#"{"lastUpdateId":7488596254027,"E":1746977566204,"T":1746977566198,"bids":[["104276.90","10.023"],["104276.80","0.032"],["104276.30","0.002"],["104276.00","0.030"],["104275.80","0.002"],["104275.70","0.392"],["104275.60","0.002"],["104275.40","0.002"],["104275.30","0.004"],["104275.00","0.002"],["104274.90","0.122"],["104274.70","0.002"],["104274.60","0.038"],["104274.40","0.007"],["104273.80","0.030"],["104273.40","0.002"],["104273.20","0.022"],["104273.00","0.001"],["104272.90","0.002"],["104272.80","0.340"],["104272.70","0.240"],["104272.40","0.197"],["104272.30","0.280"],["104272.20","0.004"],["104272.10","0.154"],["104272.00","0.637"],["104271.70","0.002"],["104271.60","0.388"],["104271.30","0.160"],["104271.20","0.225"],["104271.10","0.154"],["104271.00","0.320"],["104270.90","0.002"],["104270.80","0.194"],["104270.70","0.002"],["104270.60","0.120"],["104270.40","0.040"],["104270.30","0.018"],["104270.20","0.359"],["104270.10","0.873"],["104270.00","0.367"],["104269.80","1.026"],["104269.70","1.304"],["104269.60","0.002"],["104269.50","0.001"],["104269.30","0.001"],["104269.00","0.005"],["104268.90","0.003"],["104268.80","0.122"],["104268.70","0.002"]],"asks":[["104277.00","15.341"],["104277.10","0.010"],["104277.30","0.001"],["104277.40","0.002"],["104277.50","0.379"],["104277.60","1.080"],["104277.70","0.076"],["104278.30","0.004"],["104278.40","0.001"],["104278.70","0.003"],["104279.30","0.002"],["104279.40","0.002"],["104279.70","0.122"],["104279.80","1.061"],["104279.90","0.002"],["104280.00","0.002"],["104280.40","0.004"],["104281.20","0.038"],["104281.40","0.038"],["104281.50","0.004"],["104281.60","0.001"],["104281.80","0.002"],["104281.90","0.168"],["104282.50","0.004"],["104282.70","0.121"],["104283.10","0.002"],["104283.20","0.327"],["104283.50","0.004"],["104283.60","0.002"],["104284.00","0.040"],["104284.10","0.002"],["104284.20","0.400"],["104284.30","0.002"],["104284.60","0.004"],["104284.70","0.004"],["104284.80","0.039"],["104284.90","0.010"],["104285.60","0.004"],["104285.90","0.002"],["104286.00","0.003"],["104286.10","0.002"],["104286.20","0.460"],["104286.30","0.200"],["104286.40","1.230"],["104286.60","0.040"],["104286.70","0.002"],["104286.90","0.003"],["104287.00","0.002"],["104287.10","0.001"],["104287.20","0.038"]]}"#;

#[derive(Debug, Deserialize)]
pub struct OrderBookV1 {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub tx_time: u64,

    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub struct OrderBookV2<'a> {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub tx_time: u64,

    #[serde(bound(deserialize = "'de: 'a"))]
    pub bids: Vec<(&'a [u8], &'a [u8])>,

    #[serde(bound(deserialize = "'de: 'a"))]
    pub asks: Vec<(&'a [u8], &'a [u8])>,
}

#[derive(Debug, Deserialize)]
pub struct OrderBookV3 {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub tx_time: u64,

    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Deserialize)]
pub struct OrderBookV4 {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub tx_time: u64,

    pub bids: Vec<(Fp<2>, Fp<3>)>,
    pub asks: Vec<(Fp<2>, Fp<3>)>,
}

// // Insert benchmark
fn bench_serde(c: &mut Criterion) {
    c.bench_function("serde_v1", |b| {
        b.iter(|| {
            let res: OrderBookV1 = serde_json::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("serde_v2", |b| {
        b.iter(|| {
            let res: OrderBookV2 = serde_json::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("serde_v3", |b| {
        b.iter(|| {
            let res: OrderBookV3 = serde_json::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("serde_v4", |b| {
        b.iter(|| {
            let res: OrderBookV4 = serde_json::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });
}

fn bench_sonic(c: &mut Criterion) {
    c.bench_function("sonic_v1", |b| {
        b.iter(|| {
            let res: OrderBookV1 = sonic_rs::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("sonic_v2", |b| {
        b.iter(|| {
            let res: OrderBookV2 = sonic_rs::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("sonic_v3", |b| {
        b.iter(|| {
            let res: OrderBookV3 = sonic_rs::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });

    c.bench_function("sonic_v4", |b| {
        b.iter(|| {
            let res: OrderBookV4 = sonic_rs::from_slice(&TEST_DATA).unwrap();
            black_box(res);
        });
    });
}

fn bench_simd_json(c: &mut Criterion) {
    c.bench_function("simd_json_v1", |b| {
        b.iter(|| {
            let mut data = TEST_DATA.to_vec();
            let res: OrderBookV1 = simd_json::from_slice(&mut data).unwrap();
            black_box(res);
        });
    });

    c.bench_function("simd_json_v2", |b| {
        b.iter(|| {
            let mut data = TEST_DATA.to_vec();
            let res: OrderBookV2 = simd_json::from_slice(&mut data).unwrap();
            black_box(res);
        });
    });

    c.bench_function("simd_json_v3", |b| {
        b.iter(|| {
            let mut data = TEST_DATA.to_vec();
            let res: OrderBookV3 = simd_json::from_slice(&mut data).unwrap();
            black_box(res);
        });
    });

    c.bench_function("simd_json_v4", |b| {
        b.iter(|| {
            let mut data = TEST_DATA.to_vec();
            let res: OrderBookV4 = simd_json::from_slice(&mut data).unwrap();
            black_box(res);
        });
    });
}

criterion_group!(benches, bench_serde, bench_sonic, bench_simd_json);
criterion_main!(benches);
