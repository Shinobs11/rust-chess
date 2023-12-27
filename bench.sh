#!/bin/bash
hash=$(git rev-parse --short HEAD);
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd );
export CRITERION_HOME="$SCRIPT_DIR/bench_results";

cargo bench --bench bench  -- --verbose --save-baseline "bench-$hash"