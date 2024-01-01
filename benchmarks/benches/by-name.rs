use std::convert::TryInto;
use std::str::FromStr;
use std::time::{Duration, Instant};

use criterion::black_box;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;
use tzdb_data::{find_raw, TZ_NAMES};

fn benchmark_by_name(c: &mut criterion::Criterion) {
    // collect all names with "random" capitalization
    let mut names: Vec<(String, usize)> = TZ_NAMES
        .iter()
        .flat_map(|&name| {
            let raw_len = find_raw(name.as_bytes()).unwrap().len();
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

    // insert a bunch of unknown names
    for idx in 0..100 {
        let mut rng = Xoroshiro128PlusPlus::seed_from_u64(idx);

        let mut continent = *b"abcdefghijklmnopqrstuvwxyz";
        let mut city = *b"abcdefghijklmnopqrstuvwxyz";

        continent.shuffle(&mut rng);
        city.shuffle(&mut rng);

        let continent = &mut continent[..(4..=8).choose(&mut rng).unwrap()];
        let city = &mut city[..(4..=12).choose(&mut rng).unwrap()];

        continent[0] = continent[0].to_ascii_uppercase();
        city[0] = city[0].to_ascii_uppercase();

        let continent = std::str::from_utf8(continent).unwrap();
        let city = std::str::from_utf8(city).unwrap();

        let raw_name = format!("{}/{}", continent, city);
        let raw_len = crate::find_raw(raw_name.as_bytes())
            .unwrap_or_default()
            .len();
        names.push((raw_name, raw_len));
    }

    let orig_names: Vec<_> = names
        .iter()
        .map(|&(ref s, len)| (s.as_str(), len))
        .collect();
    let mut names = Vec::with_capacity(orig_names.len());

    // benchmark per name lookup time

    macro_rules! bench_function {
        ($id:literal($name:pat, $raw_len:pat) $body:expr) => {
            c.bench_function($id, |b| {
                b.iter_custom(|iters| {
                    let mut nanos = 0;
                    for i in 0..iters {
                        names.clear();
                        names.extend_from_slice(orig_names.as_slice());
                        names.shuffle(&mut Xoroshiro128PlusPlus::seed_from_u64(i));
                        let names = black_box(names.as_slice());

                        let start = Instant::now();
                        for &($name, $raw_len) in names {
                            $body
                        }
                        nanos += start.elapsed().as_nanos();
                    }
                    Duration::from_nanos(
                        (nanos / orig_names.len() as u128)
                            .try_into()
                            .expect("Did the test take 584 years to finish?"),
                    )
                });
            });
        };
    }

    bench_function!(
        "tzdb::find_raw"
        (name, raw_len)
        assert_eq!(
            raw_len,
            black_box(crate::find_raw(name.as_bytes())).unwrap_or_default().len(),
        )
    );
    bench_function!(
        "chrono_tz::Tz::from_str"
        (name, _)
        if let Ok(tz) = chrono_tz::Tz::from_str(name) {
            assert!(!black_box(tz.name()).is_empty());
        }
    );
    bench_function!(
        "chrono_tz::Tz::from_str_insensitive"
        (name, _)
        if let Ok(tz) = chrono_tz::Tz::from_str_insensitive(name) {
            assert!(!black_box(tz.name()).is_empty());
        }
    );
}

fn main() {
    #[cfg(not(miri))]
    {
        criterion::criterion_group!(benches, benchmark_by_name);
        benches();

        criterion::Criterion::default()
            .configure_from_args()
            .with_plots()
            .final_summary();
    }
}
