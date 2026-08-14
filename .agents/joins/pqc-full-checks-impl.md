# Join: full expected-value checks for `p2mr_pqc_construction.json`

## Goal

Stop treating PQC construction as presence-only. Require the same full checks
as classic `p2mr_construction.json` in hermetic Rust, Python, and Node.

## What was red / gap

- **Product gap (real):** hermetic tests only asserted fixture presence /
  `version == 1` / non-empty `test_vectors` for pqc:
  - Rust: `p2mr_pqc_fixture_present`
  - Python: `test_p2mr_pqc_fixture_present`
  - Node: `p2mr_pqc_construction.json is preserved`
- **Algorithm red:** none observed after strengthening. Shared pure construction
  runners already honor `leafVersion`, control-block first byte (`ver | 1`),
  merkle root / scriptPubKey / bech32m, and error vectors (missing
  `scriptTree` / snake_case `script_tree` empty). Updated fixture
  (jeanpablojp@1baf0e71 / PR #2220 leaf-version control-block fix) already
  matches the runners.

TDD framing: the missing requirement *was* the red (presence-only allowed a
wrong algorithm to pass CI). Strengthened tests + green proves equality.

## What changed

| Path | Change |
|------|--------|
| `tests/p2mr_construction.rs` | `p2mr_pqc_construction_vectors` calls shared `load_and_run` (full checks) |
| `python/tests/test_p2mr_construction.py` | `test_p2mr_pqc_construction_json` via `_run_fixture`; helper asserts both files |
| `nodejs/tests/p2mr_construction.test.ts` | shared `runFixture` for classic + pqc |
| `tests/vectors/p2mr/README.md` | both fixtures required with full expected-value checks |
| module doc on hermetic Rust test | notes full checks for both fixtures |

No product algorithm edits. No assert weakening. Workshop left alone.

## Commands + results

```text
cargo test --test p2mr_construction
# ok — p2mr_construction_vectors, p2mr_pqc_construction_vectors

cd python && python3 -m unittest tests.test_p2mr_construction -v
# ok — 3 tests (classic, pqc full, BIP360 helper both paths)

cd nodejs && npx jest tests/p2mr_construction.test.ts
# PASS — p2mr_construction.json, p2mr_pqc_construction.json
```

Notable green vector: `p2mr_different_version_leaves` control blocks start with
`c1` / `fb` (`0xc0|1` / `0xfa|1`), matching #2220 leaf-version control-block
byte.

## Remaining gaps

- Workshop Rust (`examples/bip360-workshop/rust/tests/p2mr_pqc_construction.rs`)
  already has fuller stack checks via rust-bitcoin; not run here (may not
  compile in default workspace).
- Pure refs ignore `priv_key` / `asm` / OP_SUBSTR semantics (construction only;
  same as classic). No claim those are spent/verified.
- PQC fixture objective text still has a stale TO-DO about ignoring leaf
  version; runners and expected values *do* honor leafVersion (fixture data is
  the SoT for CI).
