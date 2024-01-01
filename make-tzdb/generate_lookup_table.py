#!/usr/bin/env python3

from logging import basicConfig, INFO
from shutil import which
from subprocess import PIPE, Popen
from sys import stdin, stdout
from typing import Optional
from re import match


def convert(stdin, stdout):
    gperf = which('gperf')
    if not gperf:
        raise Exception('No gperf')
    gperf = Popen(
        ['gperf', '--ignore-case', '--readonly-tables', '--struct-type', '--global-table', '-m', '1000'],
        stdin=stdin, stdout=PIPE,
    )
    with gperf:
        stdin, _ = gperf.communicate(timeout=120)
    match gperf.returncode:
        case 0:
            pass
        case code:
            raise Exception(f'gperf returned {code!r}')
    stdin = str(stdin, 'UTF-8', 'strict')

    total_keywords: Optional[int] = None
    min_word_length: Optional[int] = None
    max_word_length: Optional[int] = None
    min_hash_value: Optional[int] = None
    max_hash_value: Optional[int] = None
    duplicates: Optional[int] = None
    maximum_key_range: Optional[int] = None
    asso_values: list[int] = []
    hash_init: Optional[str] = []
    hash_switch_fst_idx: Optional[int] = None
    hash_switch_idx: Optional[int|str] = None
    hash_switch: dict[int|str, tuple[str, Optional[str]]] = {}
    table: list[Optional[tuple[str, str]]] = []

    state = 'start'
    line: str
    for line in stdin.splitlines():
        line = line.strip()
        if not line:
            continue

        match state:
            case 'start':
                match [s.strip() for s in line.split()]:
                    case ('#define', 'TOTAL_KEYWORDS', s):
                        total_keywords = int(s)
                        continue
                    case ('#define', 'MIN_WORD_LENGTH', s):
                        min_word_length = int(s)
                        continue
                    case ('#define', 'MAX_WORD_LENGTH', s):
                        max_word_length = int(s)
                        continue
                    case ('#define', 'MIN_HASH_VALUE', s):
                        min_hash_value = int(s)
                        continue
                    case ('#define', 'MAX_HASH_VALUE', s):
                        max_hash_value = int(s)
                        continue

                m = match(r'/\* maximum key range = (\d+), duplicates = (\d+) \*/', line)
                if m:
                    maximum_key_range, duplicates = m.groups()
                    maximum_key_range = int(maximum_key_range)
                    duplicates = int(duplicates)
                    state = 'pre_asso_values'

            case 'pre_asso_values':
                if 'asso_values[] =' in line:
                    state = 'asso_values'

            case 'asso_values':
                if '}' in line:
                    state = 'pre_hash_switch'
                elif '{' not in line:
                    s = (s.strip() for s in line.split(','))
                    asso_values.extend(int(s) for s in s if s)

            case 'pre_hash_switch':
                m = match(r'register unsigned int hval = ([^;]+);', line)
                if m:
                    hash_init, = m.groups()
                    state = 'hash_switch'
                    continue

            case 'hash_switch':
                if line == 'default:':
                    hash_switch_idx = 'default'
                    continue

                m = match(r'(return)?[^\)]+\)str\[([^\]]+)*\](?:([\+\-]\d+))?\];', line)
                if m:
                    ret, idx, offs = m.groups()
                    if ret:
                        hash_switch_idx = 'finally'
                    assert hash_switch_idx is not None 
                    hash_switch[hash_switch_idx] = idx, offs
                    hash_switch_idx = None
                    if ret:
                        state = 'pre_wordlist'
                    continue

                m = match(r'case (\d+):', line)
                if m:
                    hash_switch_idx, = m.groups()
                    hash_switch_idx = int(hash_switch_idx)
                    if hash_switch_fst_idx is None:
                        hash_switch_fst_idx = hash_switch_idx
                    continue

            case 'pre_wordlist':
                if line.endswith('wordlist[] ='):
                    state = 'wordlist'

            case 'wordlist':
                if line == '};':
                    state = 'done'
                    break

                m = match(r'{"([^"]+)", "([^"]+)"}', line)
                if m:
                    name, canon = m.groups()
                    table.append((name, canon))
                    continue

                for _ in range(line.count('{""}')):
                    table.append(None)

    assert duplicates == 0

    print('// GENERATED FILE', file=stdout)
    print('// ALL CHANGES MADE IN THIS FOLDER WILL BE LOST!', file=stdout)
    print(file=stdout)
    print('use tz::TimeZoneRef;', file=stdout)
    print(file=stdout)
    print('use crate::eq_ignore_ascii_case;', file=stdout)
    print('use super::raw_tzdata;', file=stdout)
    print('use super::tzdata;', file=stdout)
    print(file=stdout)

    print('#[derive(Clone, Copy)]', file=stdout)
    print('#[repr(u16)]', file=stdout)
    print('pub(crate) enum Index {', file=stdout)
    idx = 0
    for entry in table:
        match entry:
            case (name, canon):
                print(f'    V{idx} = {idx},', file=stdout)
                idx += 1
    entry_count = idx
    print('}', file=stdout)
    print(file=stdout)

    print(f'const WORDLIST: [Option<Index>; {len(table)}] = [', file=stdout)
    idx = 0
    for entry in table:
        match entry:
            case (name, canon):
                print(f'    Some(Index::V{idx}),', file=stdout)
                idx += 1
            case _:
                print('    None,', file=stdout)
    print('];', file=stdout)
    print(file=stdout)

    print(f'const NAMES: [&[u8]; {entry_count}] = [', file=stdout)
    for entry in table:
        match entry:
            case (name, canon):
                print(f'    b"{name}",', file=stdout)
    print('];', file=stdout)
    print(file=stdout)

    print(f'pub(crate) const TIME_ZONES: [&TimeZoneRef<\'static>; {entry_count}] = [', file=stdout)
    for entry in table:
        match entry:
            case (name, canon):
                print(f'    &tzdata::{canon},', file=stdout)
    print('];', file=stdout)
    print(file=stdout)

    print(f'pub(crate) const RAW_TIME_ZONES: [&[u8]; {entry_count}] = [', file=stdout)
    for entry in table:
        match entry:
            case (name, canon):
                print(f'    raw_tzdata::{canon},', file=stdout)
    print('];', file=stdout)
    print(file=stdout)

    asso_values.pop()
    print(f'const ASSO_VALUES: [u16; 257] = [', file=stdout)
    for asso_value in asso_values:
        print(f'    {asso_value},', file=stdout)
    print(f'{max_hash_value + 1}];', file=stdout)
    print(file=stdout)

    print('pub(crate) const fn find_key(s: &[u8]) -> Option<Index> {', file=stdout)
    print('    let len = s.len();', file=stdout)
    print(f'    if !matches!(len, {min_word_length}..={max_word_length}) {{', file=stdout)
    print('        return None;', file=stdout)
    print('    }', file=stdout)
    print(file=stdout)

    def hash_add(idx, offs):
        value = f's[{idx}] as usize'
        if offs:
            if offs.startswith('+'):
                value = f'({value}).wrapping_add({offs[1:]})'
            elif offs.startswith('-'):
                value = f'({value}).wrapping_sub({offs[1:]})'
            else:
                raise Exception(f'offs? {offs!r}')
        return f'key = key.wrapping_add(ASSO_VALUES[{value}] as usize);'

    match hash_init:
        case 'len':
            print('    let mut key: usize = len;', file=stdout)
        case _:
            raise Exception(f'hash_init == {hash_init!r} != "len"')
    match hash_switch.get('finally'):
        case (idx, offs):
            print(f'    {hash_add(idx, offs)}', file=stdout)
    for item in reversed(hash_switch.items()):
        match item:
            case (int(key), (idx, offs)):
                print(f'    if len >= {key} {{', file=stdout)
                print(f'        {hash_add(idx, offs)}', file=stdout)
                print('    }', file=stdout)
    match hash_switch.get('default'):
        case (idx, offs):
            print(f'    if len > {hash_switch_fst_idx} {{', file=stdout)
            print(f'        {hash_add(idx, offs)}', file=stdout)
            print('    }', file=stdout)
    print(file=stdout)

    print(f'    if key > {max_hash_value} {{', file=stdout)
    print('        return None;', file=stdout)
    print('    }', file=stdout)
    print('    let key = match WORDLIST[key] {', file=stdout)
    print('        Some(key) => key,', file=stdout)
    print('        None => return None,', file=stdout)
    print('    };', file=stdout)
    print('    if !eq_ignore_ascii_case(s, NAMES[key as u16 as usize]) {', file=stdout)
    print('        return None;', file=stdout)
    print('    }', file=stdout)
    print(file=stdout)
    print('    Some(key)', file=stdout)
    print('}', file=stdout)
    print(file=stdout)


if __name__ == '__main__':
    basicConfig(level=INFO)
    convert(stdin, stdout)
