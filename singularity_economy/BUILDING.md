# Building & Testing

The project is pure Rust (`rust/singularity-econ`). The original Python
reference implementation was retired after the Rust port reached 1e-6
parity on golden vectors — git history preserves it; the Rust crate is now
the specification, and `output/golden_v2.json` is its frozen regression
snapshot.

Two build paths:

- **Buck2** builds and tests everything third-party-free (the `serde` cargo
  feature is off, so the core + behavior + book suites compile with rustc
  alone).
- **Cargo** is the dependency-full path: the CLI, snapshot-regression tests
  (serde_json), and proptest property suites.

## Buck2

Cell rooted at `singularity_economy/` (`.buckconfig`), prelude bundled in
the buck2 binary, system toolchains (`toolchains/BUCK`).

```bash
cd singularity_economy
buck2 test //rust/singularity-econ:core_test \
           //rust/singularity-econ:behavior \
           //rust/singularity-econ:book
```

## Cargo

```bash
cd singularity_economy/rust/singularity-econ
cargo test                                     # everything incl. snapshots + proptest
cargo build --release
./target/release/singularity-econ run          # baseline year-by-year (JSON)
./target/release/singularity-econ book ../../output/financials.json  # trade valuations
./target/release/singularity-econ mc 10000 7   # Monte Carlo distributions (~30ms)
./target/release/singularity-econ sa 20000 11  # Spearman sensitivity study
./target/release/singularity-econ golden       # regenerate regression snapshot
```

Run everything: `./check.sh`

## The snapshot contract

`output/golden_v2.json` freezes the model's behavior across six named cases
(baseline, B1-off, credit-stress, fast-takeoff, capital-bound, power-tight).
A `parity.rs` failure means behavior changed: if intentional, regenerate with
`singularity-econ golden` so the snapshot diff documents the change in
review; if not, it caught a bug.

## Test inventory (47 tests)

| Suite | Runner | What it proves |
|---|---|---|
| lib unit (2) | buck2 + cargo | pipeline conservation, baseline sanity |
| `tests/behavior.rs` (22) | buck2 + cargo | pipeline delays, invariants, 2026 calibration anchors, **loop ablations** (each named feedback loop, disabled, changes behavior as theory predicts), endogenous rent dynamics (silicon decay vs IP/power persistence, glut emergence), timing shifts, starved-recovery |
| `tests/book.rs` (14) | buck2 + cargo | reverse-DCF roundtrip, pool mapping/capture economics, scenario ordering, **trade-book conclusion locks** (semi/power longs positive, wage shorts negative, robotics fails hurdle, fizzle worst for beneficiaries) |
| `tests/parity.rs` (6) | cargo | behavior == frozen snapshot on six cases at 1e-6 |
| `tests/props.rs` (3×256) | cargo | invariants over the whole parameter space; supply-response monotonicity |
