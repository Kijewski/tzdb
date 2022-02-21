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

use std::collections::HashMap;
use std::env::args;
use std::fmt::Write as _;
use std::fs::read_dir;
use std::io::Write as _;

use convert_case::{Case, Casing};
use tz::TimeZone;

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

    let mut files = HashMap::<Vec<u8>, Vec<_>>::new();

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
                files
                    .entry(bytes)
                    .or_default()
                    .push((name.to_owned(), to_pascal(name)));
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
                    let name = format!("{}/{}", &folder, name);
                    let pascal = to_pascal(&name);
                    files.entry(bytes).or_default().push((name, pascal));
                }
            }
        }
    }

    let mut files = files
        .into_iter()
        .map(|(bytes, names)| {
            let canon = names
                .iter()
                .map(|(_, pascal)| pascal.as_str())
                .min_by(|l, r| l.cmp(r))
                .unwrap()
                .to_owned();
            (bytes, canon, names)
        })
        .collect::<Vec<_>>();
    files.sort_by(|l, r| l.1.cmp(&r.1));

    let mut names_and_indices = files
        .iter()
        .enumerate()
        .flat_map(|(index, (_, canon, names))| {
            let canon = canon.as_str();
            names
                .iter()
                .map(move |(name, pascal)| (index, name.as_str(), pascal.as_str(), canon))
        })
        .collect::<Vec<_>>();
    names_and_indices.sort_by(|l, r| l.2.cmp(r.2));

    let mut f = String::new();

    writeln!(
        f,
        r#"// SPDX-License-Identifier: MIT-0

// GENERATED FILE
// ALL CHANGES MADE IN THIS FILE WILL BE LOST!

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
use phf::phf_map;
use tinystr::TinyAsciiStr;
use tz::TimeZone;

use crate::DbTimeZone;
"#
    )?;

    writeln!(f, "/// All defined time zones statically accessible")?;
    writeln!(f, "#[allow(non_upper_case_globals)]")?;
    writeln!(f, "pub mod time_zone {{")?;
    writeln!(f, "    use super::*;")?;
    for (index, name, pascal, canon) in &names_and_indices {
        writeln!(f)?;
        writeln!(f, "    /// {},", name)?;
        if pascal == canon {
            writeln!(f, "    pub static {}: &DbTimeZone = &DbTimeZone {{", canon)?;
            writeln!(f, "        index: {},", index)?;
            writeln!(f, "        name: {:?},", name)?;
            writeln!(f, "        debug_name: {:?},", canon)?;
            writeln!(f, "        bytes: bytes::{},", canon)?;
            writeln!(f, "        parsed: &parsed::{},", canon)?;
            writeln!(f, "    }};")?;
        } else {
            writeln!(f, "    pub static {}: &DbTimeZone = {};", pascal, canon)?;
        }
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(
        f,
        "pub(crate) fn tz_by_name(s: &str) -> Option<&'static DbTimeZone> {{"
    )?;
    writeln!(
        f,
        "    let s: TinyAsciiStr<{}> = s.parse().ok()?;",
        names_and_indices.iter().map(|t| t.1.len()).max().unwrap(),
    )?;
    writeln!(f, "    let s = s.to_ascii_lowercase();")?;
    writeln!(f, "    Some(*TIME_ZONES_BY_NAME.get(&s)?)")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(
        f,
        "static TIME_ZONES_BY_NAME: phf::Map<&'static str, &'static DbTimeZone> = phf_map!("
    )?;
    for (_, name, _, canon) in &names_and_indices {
        writeln!(
            f,
            "    {:?} => time_zone::{},",
            name.to_ascii_lowercase(),
            canon,
        )?;
    }
    writeln!(f, ");")?;
    writeln!(f)?;

    writeln!(
        f,
        "pub(crate) static TIME_ZONES_LIST: [(&str, &DbTimeZone); {}] = [",
        names_and_indices.len()
    )?;
    for (_, name, _, canon) in &names_and_indices {
        writeln!(f, "    ({:?}, time_zone::{}),", name, canon)?;
    }
    writeln!(f, "];")?;
    writeln!(f)?;

    writeln!(f, "#[allow(non_upper_case_globals)]")?;
    writeln!(f, "pub(crate) mod parsed {{")?;
    writeln!(f, "    use super::*;")?;
    writeln!(f)?;
    for (_, canon, _) in &files {
        writeln!(
            f,
            "    pub(crate) static {}: OnceBox<TimeZone> = OnceBox::new();",
            canon
        )?;
    }
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(f, "#[allow(non_upper_case_globals)]")?;
    writeln!(f, "pub(crate) mod bytes {{")?;
    for (bytes, canon, _) in &files {
        writeln!(f, "    pub(crate) const {}: &[u8] = &{:?};", canon, bytes)?;
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

fn to_pascal(name: &str) -> String {
    name.replace('/', " ")
        .replace("GMT+", " GMT plus ")
        .replace("GMT-", " GMT minus ")
        .to_case(Case::Pascal)
}
