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

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::args;
use std::fmt::Write as _;
use std::fs::read_dir;
use std::io::Write as _;

use convert_case::{Case, Casing};
use itertools::Itertools;
use tz::TimeZone;

struct TzName {
    /// to_pascal("Europe/Belfast")
    canon: String,
    /// "Europe/Guernsey"
    full: String,
    /// Some(to_pascal("Europe"))
    major: Option<String>,
    /// to_pascal("Guernsey")
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
    fn new(folder: Option<&str>, name: &str) -> TzName {
        let full_name = match folder {
            Some(folder) => format!("{}/{}", folder, name),
            None => name.to_owned(),
        };
        Self {
            canon: "".to_owned(),
            full: full_name,
            major: folder.map(|s| prepare_casing(s).to_case(Case::Snake)),
            minor: prepare_casing(name).to_case(Case::UpperSnake),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut args = args().into_iter().fuse();
    args.next();

    let target = args.next().unwrap_or_else(|| "/dev/stdout".to_owned());

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

    let mut entries_by_bytes = HashMap::<Vec<u8>, Vec<TzName>>::new();

    let mut folders = vec![];
    for entry in read_dir(&base_path)?.filter_map(|f| f.ok()) {
        let name = entry.file_name();
        let name = match name.to_str() {
            Some(name) if !name.contains('.') => name,
            _ => continue,
        };
        if entry.file_type().map(|f| f.is_dir()).unwrap_or_default() {
            folders.push(name.to_owned());
            continue;
        }
        if let Ok(bytes) = std::fs::read(format!("{}/{}", &base_path, name)) {
            if TimeZone::from_tz_data(&bytes).is_ok() {
                let tz_entry = TzName::new(None, name);
                entries_by_bytes.entry(bytes).or_default().push(tz_entry);
            }
        }
    }
    for folder in folders {
        for entry in read_dir(format!("{}/{}", base_path, folder))?.filter_map(|f| f.ok()) {
            let name = entry.file_name();
            let name = match name.to_str() {
                Some(name) if !name.contains('.') => name,
                _ => continue,
            };
            if let Ok(bytes) = std::fs::read(format!("{}/{}/{}", &base_path, &folder, name)) {
                if TimeZone::from_tz_data(&bytes).is_ok() {
                    let tz_entry = TzName::new(Some(folder.as_str()), name);
                    entries_by_bytes.entry(bytes).or_default().push(tz_entry);
                }
            }
        }
    }
    for entries in entries_by_bytes.values_mut() {
        entries.sort();
        let canon = prepare_casing(&entries.first().unwrap().full).to_case(Case::UpperSnake);
        for entry in entries {
            entry.canon = canon.clone();
        }
    }

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

    let count: usize = entries_by_major
        .iter()
        .map(|(_, entries)| entries.len())
        .sum();

    let mut f = String::new();

    writeln!(
        f,
        r#"// SPDX-License-Identifier: MIT-0
//
// GENERATED FILE
// ALL CHANGES MADE IN THIS FILE WILL BE LOST!
//
// MIT No Attribution
//
// Copyright 2022 René Kijewski <crates.io@k6i.de>
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

use once_cell::race::OnceBox;
use tz::TimeZone;

use crate::DbTimeZone;
"#
    )?;

    writeln!(f, "/// All defined time zones statically accessible")?;
    writeln!(f, "pub mod time_zone {{")?;
    writeln!(f, "    use super::*;")?;
    for (folder, entries) in &entries_by_major {
        writeln!(f)?;
        if let Some(folder) = folder {
            writeln!(f, "/// {}", folder)?;
            writeln!(f, "pub mod {} {{", folder)?;
            writeln!(f, "    use super::*;")?;
            writeln!(f)?;
        }
        for entry in entries {
            writeln!(f, "    /// {},", entry.full)?;
            writeln!(
                f,
                "pub static {}: &DbTimeZone = &tzdata::{};",
                entry.minor, entry.canon,
            )?;
        }
        if folder.is_some() {
            writeln!(f, "}}")?;
        }
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(f, r#"#[cfg(feature = "by-name")]"#)?;
    writeln!(
        f,
        "pub(crate) fn tz_by_name(s: &str) -> Option<&'static DbTimeZone> {{"
    )?;
    writeln!(
        f,
        "    Some(*TIME_ZONES_BY_NAME.get(crate::Lower32([0u128; 2]).for_str(s)?)?)"
    )?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    let mut phf = phf_codegen::Map::new();
    for entries in entries_by_bytes.values() {
        for entry in entries {
            phf.entry(
                entry.full.to_ascii_lowercase(),
                &format!("&tzdata::{}", entry.canon),
            );
        }
    }
    writeln!(f, r#"#[cfg(feature = "by-name")]"#)?;
    writeln!(
        f,
        "static TIME_ZONES_BY_NAME: phf::Map<&'static str, &'static DbTimeZone> = {};",
        phf.build(),
    )?;
    writeln!(f)?;

    writeln!(f, r#"#[cfg(feature = "list")]"#)?;
    writeln!(
        f,
        "pub(crate) static TIME_ZONES_LIST: [(&str, &DbTimeZone); {}] = [",
        count,
    )?;
    for (_, entries) in entries_by_major.iter() {
        for entry in entries {
            writeln!(f, "({:?}, &tzdata::{}),", entry.full, entry.canon)?;
        }
    }
    writeln!(f, "];")?;
    writeln!(f)?;

    writeln!(f, "mod tzdata {{")?;
    writeln!(f, "    use super::*;")?;
    for (index, entries) in entries_by_bytes.values().enumerate() {
        let entry = &entries[0];
        writeln!(f)?;
        writeln!(
            f,
            "pub(crate) static {}: DbTimeZone = DbTimeZone {{",
            &entry.canon
        )?;
        writeln!(f, "    index: {},", index)?;
        writeln!(f, "    name: {:?},", &entry.full)?;
        writeln!(f, "    debug_name: {:?},", &entry.canon)?;
        writeln!(f, "    bytes: &bytes::{},", &entry.canon)?;
        writeln!(f, "    parsed: &parsed::{},", &entry.canon)?;
        writeln!(f, "}};")?;
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(f, "pub(crate) mod parsed {{")?;
    writeln!(f, "    use super::*;")?;
    writeln!(f)?;
    for entries in entries_by_bytes.values() {
        writeln!(
            f,
            "pub(crate) static {}: OnceBox<TimeZone> = OnceBox::new();",
            &entries[0].canon,
        )?;
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(f, "pub(crate) mod bytes {{")?;
    for (bytes, entries) in &entries_by_bytes {
        writeln!(
            f,
            "pub(crate) const {}: [u8; {}] = {:?};",
            &entries[0].canon,
            bytes.len(),
            bytes,
        )?;
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(target)?
        .write_all(f.as_bytes())?;

    Ok(())
}

fn prepare_casing(name: &str) -> String {
    name.replace('/', " ")
        .replace("GMT+", " GMT plus ")
        .replace("GMT-", " GMT minus ")
}
