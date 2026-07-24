# Building & Testing

Two build paths, by design:

- **Buck2** builds and tests the first-party core with zero third-party
  dependencies (hermetic-ish, fast, reproducible task graph).
- **Cargo** is the dependency-full path: the CLI binary, golden-parity tests
  against the Python reference, and proptest property suites (those need
  serde_json / proptest, which are cargo-managed).

The Python implementation (`model_v2.py`) is the executable specification;
the Rust crate is the fast typed port, held to 1e-6 golden parity with it.

## Buck2

The buck cell is rooted at `singularity_economy/` (`.buckconfig`), using the
prelude **bundled inside the buck2 binary** (`[external_cells] prelude =
bundled`) so the prelude version always matches your buck2 version.
Toolchains are system toolchains (`toolchains/BUCK`): rustc, python3, cxx.

Install buck2 (prebuilt, any recent release):

```bash
curl -sSL -o buck2.zst \
  https://github.com/facebook/buck2/releases/download/latest/buck2-x86_64-unknown-linux-musl.zst
python3 -c "import zstandard,sys; zstandard.ZstdDecompressor().copy_stream(open('buck2.zst','rb'), open('buck2','wb'))"
chmod +x buck2 && sudo mv buck2 /usr/local/bin/
```

Build and test everything buck knows about:

```bash
cd singularity_economy
buck2 build //rust/singularity-econ:core       # rust core (no third-party deps)
buck2 test  //rust/singularity-econ:core_test  # rust unit tests
buck2 test  //:test_model_v2 //:test_model //:test_valuation   # python suites
```

## Cargo (dependency-full path)

```bash
cd singularity_economy/rust/singularity-econ
cargo test                      # unit + golden parity + proptest (default features)
cargo build --release
./target/release/singularity-econ run          # baseline (JSON)
./target/release/singularity-econ mc 10000 7   # 10k-run Monte Carlo (~30ms)
```

The `serde` cargo feature (default on) gates all serde derives; buck builds
with it off, which is what keeps the buck target third-party-free.

## Golden vectors

`output/golden_v2.json` is generated from the Python reference. Whenever
`model_v2.py` changes behavior intentionally:

```bash
cd singularity_economy
python3 -m unittest test_model_v2            # spec must be green first
python3 - <<'PY'
# regenerate goldens (see git history for the exact block)
PY
cargo test --manifest-path rust/singularity-econ/Cargo.toml   # parity must pass
```

A parity failure after a Python change means the Rust port needs the same
change — the two must move together, goldens are the contract.

## Test inventory

| Suite | Runner | What it proves |
|---|---|---|
| `test_model.py` (24) | buck2 / unittest | v1 invariants, 2026 calibration, comparative statics |
| `test_model_v2.py` (19) | buck2 / unittest | pipeline conservation/delays, v2 invariants, **loop ablations** (each named feedback loop, disabled, changes behavior as theory predicts), rent-duration-vs-gain scaling, credit stress |
| `test_valuation.py` (12) | buck2 / unittest | reverse-DCF roundtrip, pool mapping, scenario ordering |
| `rust unit` (2) | buck2 / cargo | pipeline conservation, baseline sanity |
| `tests/parity.rs` (4) | cargo | Rust == Python on golden vectors (baseline, B1-off, credit-stress, fast-takeoff) at 1e-6 |
| `tests/props.rs` (3×256 cases) | cargo | invariants over the whole parameter space; supply-response monotonicity |
