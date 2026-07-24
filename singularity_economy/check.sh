#!/usr/bin/env bash
# One-command verification of the whole project: buck2 targets, cargo suites
# (golden parity + proptest), and the cross-model conclusion locks.
set -euo pipefail
cd "$(dirname "$0")"
echo "== buck2 =="
buck2 test //rust/singularity-econ:core_test //:test_model //:test_model_v2 \
           //:test_valuation //:test_scenarios_v2
echo "== cargo (parity + properties) =="
cargo test --manifest-path rust/singularity-econ/Cargo.toml --quiet
echo "== ALL GREEN =="
