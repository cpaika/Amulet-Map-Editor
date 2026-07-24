#!/usr/bin/env bash
# One-command verification: buck2 (dependency-free core + behavior + book
# suites) and cargo (snapshot regression + proptest + full features).
set -euo pipefail
cd "$(dirname "$0")"
echo "== buck2 =="
buck2 test //rust/singularity-econ:core_test //rust/singularity-econ:behavior \
           //rust/singularity-econ:book
echo "== cargo (snapshots + properties) =="
cargo test --manifest-path rust/singularity-econ/Cargo.toml --quiet
echo "== ALL GREEN =="
