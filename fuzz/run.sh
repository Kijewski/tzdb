#!/bin/bash
set -exuo pipefail
cd "${0%/*}"
export RUSTC_BOOTSTRAP=1
nice \
    cargo fuzz run tzdb-fuzz-libfuzzer --features="libfuzzer-sys" \
    --debug-assertions --release --jobs 16 -- \
    -max_total_time=180 -max_len=40 -timeout=5ms
