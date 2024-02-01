use std::iter::FusedIterator;
use std::str::FromStr;
use std::time::Duration;

use criterion::black_box;
use minstant::Instant;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;
use tzdb_data::{find_raw, TZ_NAMES};

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

fn benchmark_by_name(c: &mut criterion::Criterion) {
    let names = generate_names();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();

    macro_rules! bench_functions {
        ($($ident:ident($name:pat) $body:block)*) => { $({
            fn $ident(limit: u64, names: &[&str]) -> Duration {
                let start = Instant::now();
                for $name in names.iter().cycle().take64(limit) {
                    let _ = black_box($body);
                }
                start.elapsed()
            }

            c.bench_function(
                stringify!($id),
                |b| b.iter_custom(|iters| $ident(iters, black_box(&names))),
            );
        })* };
    }

    bench_functions! {
        tzdb_find_raw(name) {
            crate::find_raw(name.as_bytes())
        }
        chrono_tz_from_str(name) {
            chrono_tz::Tz::from_str(name)
        }
        chrono_tz_from_str_insensitive(name) {
            chrono_tz::Tz::from_str_insensitive(name)
        }
    }
}

fn generate_names() -> Vec<String> {
    let rng = &mut Xoroshiro128PlusPlus::seed_from_u64(2024_02_01);

    let mut names: Vec<_> = TZ_NAMES.iter().map(|s| String::from(*s)).collect();

    // insert 10% unknown names
    for _ in 0..names.len().div_ceil(10) {
        let mut continent = *b"abcdefghijklmnopqrstuvwxyz";
        let mut city = *b"abcdefghijklmnopqrstuvwxyz";

        continent.shuffle(rng);
        city.shuffle(rng);

        let continent = &mut continent[..(4..=8).choose(rng).unwrap()];
        let city = &mut city[..(4..=12).choose(rng).unwrap()];

        continent[0] = continent[0].to_ascii_uppercase();
        city[0] = city[0].to_ascii_uppercase();

        let continent = std::str::from_utf8(continent).unwrap();
        let city = std::str::from_utf8(city).unwrap();

        names.push(format!("{}/{}", continent, city));
    }

    // collect all names with "random" capitalization
    let mut names: Vec<_> = names
        .into_iter()
        .flat_map(|name| {
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
            [name, upper, lower, inverted, spongebob1, spongebob2]
        })
        .collect();

    // randomize order
    names.shuffle(rng);
    names
}

#[derive(Debug, Clone, Copy)]
struct Take64<I> {
    iter: I,
    limit: u64,
}

impl<I: Iterator> Iterator for Take64<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.limit = self.limit.checked_sub(1)?;
        self.iter.next()
    }
}

impl<I: FusedIterator> FusedIterator for Take64<I> {}

trait Take64Ext: Sized {
    fn take64(self, limit: u64) -> Take64<Self> {
        Take64 { iter: self, limit }
    }
}

impl<I: Iterator> Take64Ext for I {}
