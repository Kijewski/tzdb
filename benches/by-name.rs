use std::convert::TryInto;
use std::time::{Duration, Instant};

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use tzdb::{raw_tz_by_name, TZ_NAMES};

fn benchmark_by_name(c: &mut criterion::Criterion) {
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
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect();
            let spongebob2 = name
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 1 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
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
                let names = criterion::black_box(&*names);
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
        });
    });
}

fn main() {
    #[cfg(not(miri))]
    {
        criterion::criterion_group!(benches, benchmark_by_name);
        benches();

        criterion::Criterion::default()
            .configure_from_args()
            .final_summary();
    }
}
