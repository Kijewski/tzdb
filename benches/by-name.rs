use std::convert::TryInto;
use std::time::{Duration, Instant};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use tzdb::{raw_tz_by_name, TZ_NAMES};

fn benchmark_by_name(c: &mut Criterion) {
    let mut names: Vec<(String, usize)> = TZ_NAMES
        .iter()
        .flat_map(|&name| {
            let raw_len = raw_tz_by_name(name).unwrap().len();
            let upper = name.to_uppercase();
            let lower = name.to_lowercase();
            let inverted = name
                .chars()
                .map(|c| match c {
                    'A'..='Z' => c.to_ascii_lowercase(),
                    'a'..='z' => c.to_ascii_uppercase(),
                    c => c,
                })
                .collect();
            let spongebob1 = name
                .chars()
                .enumerate()
                .map(|(i, c)| match i % 2 == 0 {
                    false => c.to_ascii_lowercase(),
                    true => c.to_ascii_uppercase(),
                })
                .collect();
            let spongebob2 = name
                .chars()
                .enumerate()
                .map(|(i, c)| match i % 2 == 0 {
                    true => c.to_ascii_lowercase(),
                    false => c.to_ascii_uppercase(),
                })
                .collect();
            [
                (name.to_owned(), raw_len),
                (upper, raw_len),
                (lower, raw_len),
                (inverted, raw_len),
                (spongebob1, raw_len),
                (spongebob2, raw_len),
            ]
        })
        .collect();

    c.bench_function("tzdb::raw_tz_by_name", |b| {
        b.iter_custom(|iters| {
            let mut nanos = 0;
            for i in 0..iters {
                names.shuffle(&mut SmallRng::seed_from_u64(i));

                let start = Instant::now();
                let names = black_box(&*names);
                for &(ref name, raw_len) in names {
                    assert_eq!(raw_len, crate::raw_tz_by_name(name).unwrap().len());
                }
                nanos += start.elapsed().as_nanos();
            }
            Duration::from_nanos(
                nanos
                    .try_into()
                    .expect("Did the test take 584 years to finish?"),
            )
        })
    });
}

criterion_group!(benches, benchmark_by_name);
criterion_main!(benches);
