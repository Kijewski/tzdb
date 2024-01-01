// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2022 René Kijewski <crates.io@k6i.de>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod parse;

use std::cmp::Ordering;
use std::env::{args, var_os};
use std::fmt::Write as _;
use std::fs::{create_dir_all, read, read_to_string, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use itertools::Itertools;
use subprocess::{Popen, PopenConfig, Redirection};
use tz::TimeZone;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct TzName {
    /// "Europe/Belfast"
    canon: String,
    /// "Europe/Guernsey"
    full: String,
    /// Some("europe") // Snake
    major: Option<String>,
    /// "GUERNSEY" // UpperSnake
    minor: String,
}

impl PartialEq for TzName {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for TzName {}

impl PartialOrd for TzName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TzName {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.is_some().cmp(&other.major.is_some()) {
            Ordering::Equal => match self.major.cmp(&other.major) {
                Ordering::Equal => self.minor.cmp(&other.minor),
                r => r,
            },
            r => r,
        }
    }
}

impl TzName {
    fn new(base_path: &Path, path: &Path) -> Option<TzName> {
        let mut path = path.iter().fuse();
        for _ in base_path {
            path.next();
        }
        let a = path.next().and_then(|o| o.to_str());
        let b = path.next().and_then(|o| o.to_str());
        let c = path.next().and_then(|o| o.to_str());
        match [a, b, c] {
            [Some("etc"), ..] => None,
            [Some(a), None, None] => Some(Self {
                canon: "".to_owned(),
                full: a.to_owned(),
                major: None,
                minor: prepare_casing(a).to_case(Case::UpperSnake),
            }),
            [Some(a), Some(b), None] => Some(Self {
                canon: "".to_owned(),
                full: format!("{a}/{b}"),
                major: Some(prepare_casing(a).to_case(Case::Snake)),
                minor: prepare_casing(b).to_case(Case::UpperSnake),
            }),
            [Some(a), Some(b), Some(c)] => Some(Self {
                canon: "".to_owned(),
                full: format!("{a}/{b}/{c}"),
                major: Some(prepare_casing(&format!("{a}/{b}")).to_case(Case::Snake)),
                minor: prepare_casing(c).to_case(Case::UpperSnake),
            }),
            _ => None,
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut args = args().into_iter().fuse();
    let _ = args.next(); // exe path

    let target_dir = PathBuf::from(args.next().unwrap_or_else(|| "tzdb/generated".to_owned()));
    create_dir_all(&target_dir)?;

    let mut base_path = args
        .next()
        .unwrap_or_else(|| "/usr/share/zoneinfo/posix/".to_owned());
    while base_path.ends_with('/') {
        if base_path.len() == 1 {
            break;
        }
        base_path.pop();
    }
    if base_path.is_empty() {
        base_path.push('.');
    }
    if !base_path.ends_with('/') {
        base_path.push('/');
    }
    let base_path = Path::new(&base_path).canonicalize()?;

    let hash_file = args.next().unwrap_or_else(|| "tzdb.tar.lz.sha".to_owned());
    let hash_file = read_to_string(&hash_file)?;
    let (hash, version) = hash_file
        .trim()
        .split_once("  ")
        .ok_or_else(|| anyhow!("Hash file {hash_file:?} malformed."))?;
    let version = version.rsplit_once('/').unwrap_or(("", version)).1;
    let version = version.split_once('.').unwrap_or((version, "")).0;
    let version = version.rsplit_once('-').unwrap_or(("", version)).1;

    let mut entries_by_bytes: IndexMap<Vec<u8>, Vec<TzName>> = IndexMap::new();
    let walkdir = WalkDir::new(&base_path)
        .min_depth(1)
        .contents_first(true)
        .follow_links(true);
    for entry in walkdir {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = base_path.join(entry.path());
        let Ok(bytes) = read(&path) else {
            continue;
        };
        if !TimeZone::from_tz_data(&bytes).is_ok() {
            continue;
        }
        let Some(tz_entry) = TzName::new(&base_path, &path) else {
            continue;
        };
        entries_by_bytes.entry(bytes).or_default().push(tz_entry);
    }
    for entries in entries_by_bytes.values_mut() {
        entries.sort();
        let canon = prepare_casing(&entries.first().unwrap().full).to_case(Case::UpperSnake);
        for entry in entries {
            entry.canon = canon.clone();
        }
    }
    entries_by_bytes.sort_by(|_, l, _, r| l[0].canon.cmp(&r[0].canon));

    let entries_by_major = entries_by_bytes
        .values()
        .flat_map(|entries| entries.iter())
        .map(|tz_entry| (tz_entry.major.as_deref(), tz_entry))
        .sorted_by(|(l, _), (r, _)| match (l, r) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(l), Some(r)) => l.cmp(r),
        })
        .group_by(|(k, _)| k.map(|s| s.to_owned()))
        .into_iter()
        .map(|(major, entries)| {
            let mut entries = entries.map(|(_, e)| e).collect_vec();
            entries.sort();
            (major, entries)
        })
        .collect_vec();

    let max_len = entries_by_major
        .iter()
        .flat_map(|(_, entries)| entries.iter())
        .map(|entry| entry.full.len())
        .max()
        .unwrap();
    assert!(max_len <= 32);

    let mut f = String::new();

    const GENERATED_FILE: &str = r#"// GENERATED FILE
// ALL CHANGES MADE IN THIS FOLDER WILL BE LOST!

"#;

    writeln!(
        f,
        r#"{GENERATED_FILE}
// MIT No Attribution
//
// Copyright 2022-2024 René Kijewski <crates.io@k6i.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![allow(clippy::pedantic)]

#[cfg(all(test, not(miri)))]
mod test_all_names;

pub(crate) mod by_name;
mod raw_tzdata;
mod tzdata;

/// All defined time zones statically accessible
pub mod time_zone;

/// The version of the source Time Zone Database
pub const VERSION: &str = {version:?};

/// The SHA512 hash of the source Time Zone Database (using the "Complete Distribution")
pub const VERSION_HASH: &str = {hash:?};
"#
    )?;

    // generate lookup table
    {
        let mut keywords = String::new();
        writeln!(
            keywords,
            "struct keyword {{ const char* name; const char* canon; }}"
        )?;
        writeln!(keywords, "%%")?;
        for entries in entries_by_bytes.values() {
            for entry in entries {
                writeln!(keywords, "{:?}, {:?}", entry.full, entry.canon)?;
            }
        }
        writeln!(keywords, "%%")?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(target_dir.join("by_name.rs"))?;
        let mut gperf = Popen::create(
            &["/usr/bin/env", "python3", "generate_lookup_table.py"],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::File(file),
                cwd: Some(var_os("CARGO_MANIFEST_DIR").ok_or(anyhow!("!CARGO_MANIFEST_DIR"))?),
                ..PopenConfig::default()
            },
        )?;
        gperf.communicate(Some(&keywords))?;
    }

    // generate exhaustive by-name test
    let mut r = GENERATED_FILE.to_owned();
    writeln!(r, "#[test]")?;
    writeln!(r, "fn test() {{")?;
    writeln!(r, "    use crate::{{find_raw, find_tz, time_zone}};")?;
    writeln!(r)?;
    writeln!(
        r,
        "    const TIME_ZONES: &[(&tz::TimeZoneRef<'static>, &[u8], &[&[u8]])] = &["
    )?;
    for entries in entries_by_bytes.values() {
        for entry in entries {
            let name = match entry.major {
                Some(ref major) => format!("{}::{}", major, &entry.minor),
                None => format!("{}", &entry.minor),
            };
            let raw_name = match entry.major {
                Some(ref major) => format!("{}::RAW_{}", major, &entry.minor),
                None => format!("RAW_{}", &entry.minor),
            };

            writeln!(r, "        (")?;
            writeln!(r, "            &time_zone::{name},")?;
            writeln!(r, "            time_zone::{raw_name},")?;
            writeln!(r, "            &[")?;
            for f in [
                |s: &str| s.to_owned(),
                |s: &str| s.to_ascii_lowercase(),
                |s: &str| s.to_ascii_uppercase(),
                |s: &str| {
                    s.chars()
                        .map(|c| match c {
                            'A'..='Z' => c.to_ascii_lowercase(),
                            'a'..='z' => c.to_ascii_uppercase(),
                            c => c,
                        })
                        .collect()
                },
                |s: &str| {
                    s.chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i % 2 == 0 {
                                c.to_ascii_uppercase()
                            } else {
                                c.to_ascii_lowercase()
                            }
                        })
                        .collect()
                },
                |s: &str| {
                    s.chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i % 2 == 1 {
                                c.to_ascii_uppercase()
                            } else {
                                c.to_ascii_lowercase()
                            }
                        })
                        .collect()
                },
            ] {
                writeln!(r, "                b{:?},", f(&entry.full))?;
            }
            writeln!(r, "            ],")?;
            writeln!(r, "        ),")?;
        }
    }
    writeln!(r, "    ];")?;
    writeln!(r)?;
    writeln!(
        r,
        "    for &(tz, raw, names) in TIME_ZONES {{ for name in names {{",
    )?;
    writeln!(
        r,
        "        assert_eq!(Some(tz), find_tz(name), \"find_tz({{:?}})\", name);",
    )?;
    writeln!(
        r,
        "        assert_eq!(Some(raw), find_raw(name), \"find_raw({{:?}})\", name);",
    )?;
    writeln!(r, "    }} }}")?;
    writeln!(r, "}}")?;
    write_string(r, target_dir.join("test_all_names.rs"))?;

    // all known time zones as reference to (raw_)tzdata
    let mut r = GENERATED_FILE.to_owned();
    for (folder, entries) in &entries_by_major {
        if let Some(folder) = folder {
            let doc = entries[0].full.as_str();
            let doc = match doc.rsplit_once('/') {
                Some((doc, _)) => doc,
                None => doc,
            };
            writeln!(r, "/// {doc}")?;
            writeln!(r, "pub mod {folder} {{")?;
        }
        for entry in entries {
            writeln!(r, "    /// Time zone data for `{:?}`", entry.full)?;
            writeln!(
                r,
                "pub const {}: tz::TimeZoneRef<'static> = crate::generated::tzdata::{};",
                entry.minor, entry.canon,
            )?;
        }

        for entry in entries {
            writeln!(
                r,
                "    /// Raw, unparsed time zone data for `{:?}`",
                entry.full
            )?;
            writeln!(
                r,
                "pub const RAW_{}: &[u8] = crate::generated::raw_tzdata::{};",
                entry.minor, entry.canon,
            )?;
        }

        if folder.is_some() {
            writeln!(r, "}}")?;
        }
    }
    write_string(r, target_dir.join("time_zone.rs"))?;

    // list of known time zone names
    let mut time_zones_list = entries_by_major
        .iter()
        .flat_map(|(_, entries)| entries.iter())
        .map(|entry| entry.full.as_str())
        .collect_vec();
    time_zones_list.sort_by_key(|l| l.to_ascii_lowercase());
    writeln!(f, "/// A list of all known time zones")?;
    writeln!(f, "pub const TZ_NAMES: &[&str] = &[",)?;
    for name in time_zones_list {
        writeln!(f, "    {:?},", name)?;
    }
    writeln!(f, "];")?;
    writeln!(f)?;

    // parsed time zone data by canonical name
    let mut r = GENERATED_FILE.to_owned();
    writeln!(r, "use tz::timezone::RuleDay;")?;
    writeln!(r, "use tz::timezone::TransitionRule;")?;
    writeln!(r, "use tz::TimeZoneRef;")?;
    writeln!(r)?;
    writeln!(r, "use crate::new_alternate_time;")?;
    writeln!(r, "use crate::new_local_time_type;")?;
    writeln!(r, "use crate::new_month_week_day;")?;
    writeln!(r, "use crate::new_time_zone_ref;")?;
    writeln!(r, "use crate::new_transition;")?;
    writeln!(r)?;
    for (bytes, entries) in &entries_by_bytes {
        writeln!(r)?;
        writeln!(
            r,
            "pub(crate) const {}: TimeZoneRef<'static> = {};",
            &entries[0].canon,
            tz_convert(bytes),
        )?;
    }
    write_string(r, target_dir.join("tzdata.rs"))?;

    // raw time zone data by canonical name
    let mut r = GENERATED_FILE.to_owned();
    for (bytes, entries) in &entries_by_bytes {
        writeln!(
            r,
            "pub(crate) const {}: &[u8] = b\"{}\";",
            &entries[0].canon,
            hex_encode(bytes)?,
        )?;
    }
    write_string(r, target_dir.join("raw_tzdata.rs"))?;

    write_string(f, target_dir.join("mod.rs"))?;

    Ok(())
}

fn write_string(mut s: String, f: PathBuf) -> std::io::Result<()> {
    if !s.ends_with("\n") {
        s.push('\n');
    }
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(f)?
        .write_all(s.as_bytes())
}

fn prepare_casing(name: &str) -> String {
    name.replace('/', " ")
        .replace("GMT+", " GMT plus ")
        .replace("GMT-", " GMT minus ")
}

fn tz_convert(bytes: &[u8]) -> crate::parse::TimeZone {
    let tz = TimeZone::from_tz_data(bytes).unwrap();
    let s = format!("{:?}", tz);
    let s = s.replace('{', "(");
    let s = s.replace('}', ")");
    ron::from_str::<crate::parse::TimeZone>(&s).unwrap()
}

fn hex_encode(v: &[u8]) -> Result<String, std::fmt::Error> {
    let mut s = String::with_capacity(v.len() * 4);
    for &b in v {
        if b == 0 {
            s.push_str("\\0");
        } else {
            write!(s, "\\x{b:02x}")?;
        }
    }
    Ok(s)
}
