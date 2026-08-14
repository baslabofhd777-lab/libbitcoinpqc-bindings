# Join: fixture SoT status verify (read-only)

**Date:** 2026-08-01
**Repo:** `/home/hunter/Projects/surmount/libbitcoinpqc-bindings`
**Mode:** read-only verification of living fixtures vs bitcoin/bips#2220, SoT inventory, remaining gaps.

## Executive answer

**Yes for #2220 on the living PQC construction fixture and hermetic multi-lang full equality, with caveats.**

- Living `p2mr_pqc_construction.json` carries the #2220 fixes (`fb` control-block first byte for the `0xfa` leaf; three-leaf `leafHashes` in DFS order).
- Hermetic Rust / Python / Node run **full** expected-value equality on **both** construction JSONs (not presence-only).
- #2220 work in this repo was **fixtures + tests** (and load-path SoT rewire), **not** product crypto algorithm code under `src/`.
- Classic `p2mr_construction.json` was already leafVersion-correct for the different-version case; #2220 did not rewrite it.
- Caveats: workshop Rust still not default-CI-compiled; macOS is GHA-only vs local `just ci`; stale TO-DO text remains in the PQC fixture objective; older joins still describe presence-only.

---

## 1) bitcoin/bips#2220 incorporation

### Living file

`/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json`

Provenance (living docs): jeanpablojp/bips@`1baf0e71…` path
`bip-0360/ref-impl/common/tests/data/p2mr_pqc_construction.json`, related to
https://github.com/bitcoin/bips/pull/2220
(see `tests/vectors/p2mr/README.md`, prior joins `pr-2220-fixture-context.md` / `fixture-sot-impl.md` / `pqc-full-checks-impl.md`).

### Check A: `p2mr_different_version_leaves` control block starts with `fb` (not `c1`)

In living PQC fixture, leaf id 1 has `"leafVersion": 250` (`0xfa`). Expected control blocks:

```text
scriptPathControlBlocks[0] = c1f224a923cd0021ab202ab139cc56802ddb92dcfc172b9212261a539df79a112a
scriptPathControlBlocks[1] = fb3bb0db8c6adcd87330a4a8c91be0fe1b23da3c151b6f2fb4f269429c43b8d8bc
```

- First byte of index 1 is **`fb`** = `0xfa | 1` (not legacy wrong `c1…`).
- Classic fixture already had `fb…` for its different-version leaf
  (`p2mr_construction.json` lines ~88–90: `fb8ad69e…`).

### Check B: three-leaf `leafHashes` order matches DFS

Trees are nested left-first: `[leaf0, [leaf1, leaf2]]`.

Hermetic collectors walk children left-to-right (DFS order of leaves):

```97:107:tests/p2mr_construction.rs
fn collect_leaf_hashes(tree: &Value) -> Vec<[u8; 32]> {
    if tree.is_object() {
        return vec![tapleaf_hash(&leaf_script_bytes(tree), leaf_version(tree))];
    }
    let arr = tree.as_array().expect("branch");
    let mut out = Vec::new();
    for sub in arr {
        out.extend(collect_leaf_hashes(sub));
    }
    out
}
```

Living fixture order for both three-leaf vectors is leaf id 0, then 1, then 2 (not pre-#2220 swapped 0↔2). Full equality asserts derived DFS hashes equal `intermediary.leafHashes`, so a swapped list would fail CI.

| Vector | `leafHashes[0]` prefix | Middle | `leafHashes[2]` prefix |
|--------|------------------------|--------|-------------------------|
| `p2mr_three_leaf_complex` | `b2a5304f…` | `837ef667…` | `0840c39e…` |
| `p2mr_three_leaf_alternative` | `ddb521a4…` | `dcef3ce8…` | `52e9326c…` |

### Check C: hermetic FULL equality on both JSON files

| Lang | Path | Both fixtures? | Full equality? |
|------|------|----------------|----------------|
| Rust | `tests/p2mr_construction.rs` | `load_and_run` for classic + pqc | Yes: leafHashes, merkleRoot, scriptPubKey, bip350Address, scriptPathControlBlocks |
| Python | `python/tests/test_p2mr_construction.py` + `python/p2mr/p2mr.py` | `test_p2mr_construction_json` + `test_p2mr_pqc_construction_json` + helper | Yes via `run_single_test` |
| Node | `nodejs/tests/p2mr_construction.test.ts` | both `runFixture(...)` | Yes (same fields) |

Module docs and `tests/vectors/p2mr/README.md` state both fixtures are **required** with the same full expected-value checks (not presence-only).

Prior green evidence (join `pqc-full-checks-impl.md`):
`cargo test --test p2mr_construction`, Python unittest, Node jest all ok after full-check promotion. This verify pass did not re-run tests (read-only).

### Product algorithm vs fixtures/tests (honest)

| Layer | Changed for #2220? |
|-------|--------------------|
| Living `p2mr_pqc_construction.json` | **Yes** (content from jeanpablojp@1baf0e71) |
| Hermetic tests (Rust/Python/Node) | **Yes** (pqc promoted from presence-only to full `load_and_run` / `_run_fixture` / `runFixture`) |
| Workshop load paths | **Yes** (include_str / default_fixtures_dir rewired to living SoT; workshop JSON copies deleted) |
| Product crate `src/` (bitcoinpqc keys/sign/verify) | **No** P2MR construction code; `rg` finds no p2mr/tapleaf/control-block in `src/` |
| Pure construction runners (test helpers / `python/p2mr`) | Already leafVersion-aware (`leafVersion \| 1`); **not** a new crypto algorithm for #2220 |

**Honest summary:** #2220 in BIPs is a **published-vector fix**. This repo adopted those vector bytes and closed the multi-lang bar gap so wrong vectors cannot hide behind presence-only. No product signature-algorithm change was required for the #2220 byte/order bugs.

---

## 2) Single source of truth inventory

### A. P2MR construction JSON (script trees / control blocks)

| Path | Role |
|------|------|
| `tests/vectors/p2mr/fixtures/p2mr_construction.json` | **Living SoT** classic (#2202-era OP_SUBSTR import) |
| `tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json` | **Living SoT** PQC (#2220-aligned @1baf0e71) |

**No second construction JSON home.** Workshop copies under
`examples/bip360-workshop/common/tests/data/` are **gone**
(deleted per `fixture-sot-impl.md`; `common/` now only `utils/`).

| Consumer | Resolution |
|----------|------------|
| Hermetic Rust | `CARGO_MANIFEST_DIR` + `tests/vectors/p2mr/fixtures` |
| Hermetic Python | `python/p2mr/p2mr.py` `default_fixtures_dir()` → repo `tests/vectors/p2mr/fixtures` |
| Node | `__dirname/../../tests/vectors/p2mr/fixtures` |
| Workshop Rust tests | `include_str!("../../../../tests/vectors/p2mr/fixtures/…")` |
| Workshop Python | `default_fixtures_dir()` → same living path (classic only in `__main__`) |

Docs: `tests/vectors/p2mr/README.md`, `examples/bip360-workshop/PROVENANCE.md`.
**Not** generated by `scripts/sync-golden-vectors.py`.

### B. Algorithm golden vectors (sign/verify)

| Kind | Path | Role |
|------|------|------|
| Canonical JSON | `tests/vectors/fixtures/secp256k1_bip340_row0.json` | SoT |
| Canonical JSON | `tests/vectors/fixtures/ml_dsa_44_golden.json` | SoT |
| Canonical JSON | `tests/vectors/fixtures/slh_dsa_sha2_golden_vectors.json` | SoT |
| Generated Rust | `tests/vectors/rust/*_golden_vectors.rs` | from sync |
| Generated Python | `tests/vectors/python/*_golden_vectors.py` | from sync |
| Generated Node | `tests/vectors/nodejs/*` | from sync |
| Generated WASM JS | `tests/vectors/wasm/*` | from sync |
| Generated C headers | `libbitcoinpqc/tests/vectors/*.h` | from sync (submodule) |

**Sync:** `scripts/sync-golden-vectors.py` (+ shell wrapper), `make sync-vectors`, `just vectors-in-sync`.
Docs: `tests/vectors/README.md`.

**Pair relationship:** JSON is source; language artifacts are **generated dual homes**, must match after sync (CI job `vectors-in-sync`). Not intentional independent content.

### C. Workshop leftovers (not fixture JSON SoT)

| Path | Role |
|------|------|
| `examples/bip360-workshop/` | Archival workshop / spend / docs / private-registry Rust |
| `examples/bip360-workshop/python/p2mr.py` | **Code** copy of construction ref (loads living fixtures); not a JSON home |
| `examples/bip360-workshop/rust/` | rust-bitcoin/kellnr stack; tests include_str living JSON |
| `examples/bip360-workshop/js/` | examples / package, not construction vector SoT |
| `examples/bip360-workshop/common/utils/` | docker/drawio/signet utils only |

### D. Other / empty / non-SoT

| Path | Note |
|------|------|
| `tests/p2mr/` | Empty directory (no modules) |
| `plan.md` | Historical dual-path language (`common/tests/data` → living); **stale vs living layout** |
| Older joins `fixture-sot-inventory.md`, `fixture-sot-impl.md` | Still describe presence-only pqc and/or workshop dual JSON; **superseded** by current code + this join |
| `nodejs/coverage/`, `node_modules/`, `build/`, `target/` | Build/cache junk, not vector SoT |
| Upstream algo internals (`libbitcoinpqc/dilithium`, `sphincsplus/vectors.py`) | Algorithm vendor trees; not BIP-360 construction fixtures |

### Pair summary

| Pair | Relationship |
|------|----------------|
| Living P2MR JSON vs workshop `common/tests/data` | Workshop copies **removed**; single SoT |
| Algorithm JSON vs rust/python/nodejs/wasm/C headers | **Generated** from JSON via sync script |
| `python/p2mr/p2mr.py` vs workshop `python/p2mr.py` | **Intentional dual code** (binding vs workshop archive); both load same living fixtures; workshop `__main__` only runs classic |
| Classic vs PQC construction JSON | Two living files, same checks, different vector sets / provenance |

---

## 3) Remaining gaps (honest)

1. **Workshop Rust not compiled in default CI**
   Package `p2mr-ref` uses private `kellnr-denver-space` registry deps; not a member of root workspace (`Cargo.toml` members: `"."`, `"fuzz"` only). Path rewire is correct; stack not hermetic.

2. **macOS CI exists in GHA, not local `just ci`**
   `.github/workflows/ci.yml` has `macos-latest` on `build-matrix` and `python-test` (and node if present). Local `just check`/`ci` is Linux-shaped (see `just-check-vs-ci.md`).

3. **Stale TO-DO text in living PQC fixture**
   `p2mr_different_version_leaves` objective still says
   `TO-DO: currently ignores given leaf version and over-rides…`
   Runners and expected values **do** honor `leafVersion` (including `fb`). Prose is wrong relative to data + tests.

4. **Stale documentation / joins**
   - `fixture-sot-inventory.md` / early `fixture-sot-impl.md`: presence-only pqc, dual workshop JSON.
   - `plan.md` still maps workshop `common/tests/data` as if copies remain.

5. **Workshop Python CLI only runs classic**
   Hermetic `python/p2mr` `__main__` runs both files; workshop `p2mr.py` `__main__` only classic.

6. **Classic construction not #2220 / not jeanpablojp rewrite**
   Still #2202-era OP_SUBSTR + priv_key style. Intentional for multi-lang bar stability; not a parity claim with upstream classic rewrite.

7. **Empty `tests/p2mr/`**
   Plan once suggested modules there; hermetic integration is `tests/p2mr_construction.rs` instead.

8. **No automated fetch of BIP construction JSON**
   Hand-maintained under `tests/vectors/p2mr/fixtures/`; no pin-script analogous to algorithm sync.

9. **This verify did not re-execute cargo/python/node**
   Evidence is tree state + prior green joins; re-run before release claims if needed.

---

## Key evidence paths (absolute)

- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_construction.json`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/README.md`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/p2mr_construction.rs`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/python/tests/test_p2mr_construction.py`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/nodejs/tests/p2mr_construction.test.ts`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/scripts/sync-golden-vectors.py`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/examples/bip360-workshop/PROVENANCE.md`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/.agents/joins/pqc-full-checks-impl.md`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/.agents/joins/fixture-sot-impl.md`
- `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/.agents/joins/pr-2220-fixture-context.md`
