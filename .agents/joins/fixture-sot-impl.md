# Join: single SoT for P2MR construction fixtures

**Status:** done
**Date:** 2026-07-31

## Goal

One load path for `p2mr_construction.json` and `p2mr_pqc_construction.json`:
`tests/vectors/p2mr/fixtures/`. No symlinks. No workshop second copy.

## Map (loaders after rewire)

| Consumer | Path resolution |
|----------|-----------------|
| Hermetic Rust `tests/p2mr_construction.rs` | `CARGO_MANIFEST_DIR` + `tests/vectors/p2mr/fixtures` (unchanged) |
| Python `python/p2mr/p2mr.py` | `default_fixtures_dir()` → repo root + fixtures (unchanged) |
| Python tests `python/tests/test_p2mr_construction.py` | via `default_fixtures_dir()` (unchanged) |
| Node `nodejs/tests/p2mr_construction.test.ts` | `__dirname` + `../../tests/vectors/p2mr/fixtures` (unchanged) |
| Workshop Rust `examples/bip360-workshop/rust/tests/p2mr_construction.rs` | `include_str!("../../../../tests/vectors/p2mr/fixtures/p2mr_construction.json")` |
| Workshop Rust `.../p2mr_pqc_construction.rs` | same dir, `p2mr_pqc_construction.json` |
| Workshop Python `examples/bip360-workshop/python/p2mr.py` | `default_fixtures_dir()` → repo root + fixtures |

## Files changed

- **Updated content:** `tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json`
  Source: https://raw.githubusercontent.com/jeanpablojp/bips/1baf0e71bb25445fa348c3cd6b92f68adac34e8b/bip-0360/ref-impl/common/tests/data/p2mr_pqc_construction.json
  md5 `db18860b…` → `5c4824db…` (same size 14418 B; content differs). Classic `p2mr_construction.json` left as-is.
- **Rewired:**
  - `examples/bip360-workshop/rust/tests/p2mr_construction.rs`
  - `examples/bip360-workshop/rust/tests/p2mr_pqc_construction.rs`
  - `examples/bip360-workshop/python/p2mr.py`
- **Deleted:**
  - `examples/bip360-workshop/common/tests/data/p2mr_construction.json`
  - `examples/bip360-workshop/common/tests/data/p2mr_pqc_construction.json`
  Empty `common/tests/data` (and parent `tests`) removed. `common/utils/` kept.
- **Docs:**
  - `tests/vectors/p2mr/README.md` — single load path; pqc provenance (jeanpablojp/bips@1baf0e71, related to bitcoin/bips#2220); consumers list
  - `examples/bip360-workshop/PROVENANCE.md` — workshop is not a second fixture home

## Tests run

| Command | Result |
|---------|--------|
| `cargo test --test p2mr_construction` | **ok** — 2 passed (`p2mr_construction_vectors`, `p2mr_pqc_fixture_present`) |
| `python3 -m unittest tests.test_p2mr_construction -v` (in `python/`) | **ok** — 3 passed |
| `python3 p2mr.py` (workshop python) | **ok** — 9/9 classic vectors |
| `npm test -- --testPathPattern=p2mr_construction` (in `nodejs/`) | **ok** — 2 passed |
| Workshop Rust `cargo test --test p2mr_construction --test p2mr_pqc_construction --no-run` | **blocked** — package thinks it is in root workspace but is not a member (`Cargo.toml` workspace membership / private registry stack). Paths rewired correctly; compile of those tests not exercised here. |

Presence-only policy for pqc left unchanged (no full equality on pure-Python/Node/hermetic refs).

## Notes

- No symlinks created.
- No git commit.
- No fetch/sync scripts added.
- `plan.md` still describes the historical snapshot plan (dual path language); living docs above are the SoT for current layout. Did not rewrite the whole plan file.
