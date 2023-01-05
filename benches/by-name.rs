use std::convert::TryInto;
use std::time::{Duration, Instant};

use rand::rngs::SmallRng;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::SeedableRng;
use tzdb::{raw_tz_by_name, TZ_NAMES};

fn benchmark_by_name(c: &mut criterion::Criterion) {
    // collect all names with "random" capitalization
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

    // insert a bunch of unknown names
    for idx in 0..100 {
        let mut rng = SmallRng::seed_from_u64(idx);

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
        let raw_len = crate::raw_tz_by_name(&raw_name).unwrap_or_default().len();
        names.push((raw_name, raw_len));
    }

    // benchmark per name lookup time
    c.bench_function("tzdb::raw_tz_by_name", |b| {
        b.iter_custom(|iters| {
            let mut nanos = 0;
            for i in 0..iters {
                names.shuffle(&mut SmallRng::seed_from_u64(i));

                let start = Instant::now();
                let names = criterion::black_box(&*names);
                for &(ref name, raw_len) in names {
                    assert_eq!(
                        raw_len,
                        crate::raw_tz_by_name(name).unwrap_or_default().len(),
                    );
                }
                nanos += start.elapsed().as_nanos();
            }
            Duration::from_nanos(
                (nanos / names.len() as u128)
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
