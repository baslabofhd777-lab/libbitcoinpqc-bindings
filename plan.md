# libbitcoinpqc-bindings — release 0.4.1 plan (revised)

Consolidated plan for the **wasm-tests** polish pass, plus **BIP 360 P2MR multi-language e2e** sourced from [bitcoin/bips#2202](https://github.com/bitcoin/bips/pull/2202) (notmike-5). Incorporates review of repo-root [`plan.md`](plan.md).

**Canonical bindings home:** [jbride/libbitcoinpqc-bindings](https://github.com/jbride/libbitcoinpqc-bindings)  
(Local `origin` may be a `cryptoquick` fork; publish/docs URLs use **jbride**.)

---

## Context

Ship **0.4.1** so notmike-5 can remove ref-impl from [BIPs PR #2202](https://github.com/bitcoin/bips/pull/2202) and point here. Algorithm golden vectors already live in this repo. **P2MR construction code + vectors currently live only in that PR and will be removed soon** — they must be **copied into this repo first** (full preserve), then **extended** to drive real multi-language e2e against our FFI bindings.

### Preserve, then extend (non-negotiable)

| Source (PR #2202) | Preserve into bindings | Extend for our work |
|-------------------|------------------------|---------------------|
| `common/tests/data/*.json` | **First-class living vectors** under `tests/vectors/p2mr/fixtures/` | Same JSON informs Python / Rust / Node / WASM e2e; update here when BIP 360 construction semantics change |
| `python/p2mr.py` | **Copy** into Python binding tree | Point at shared fixtures; wire CI tests; align with current `bitcoinpqc` Python API where scripts/signatures meet |
| `rust/` p2mr construction + pqc construction tests / helpers | **Copy** into Rust binding tree (tests + any support modules) | Drop/private-registry-gate non-hermetic deps for default CI; extend to use **this** crate’s `bitcoinpqc` (0.4.x, SLH-DSA-SHA2, three algos) instead of kellnr `bitcoinpqc` 0.3.0 |
| `js/` p2mr example + wasm smoke | **Copy** into Node/WASM binding trees | Retarget unscoped **`bitcoinpqc`** / `bitcoinpqc/wasm` **0.4.x**; replace SHAKE naming with SHA2 where still present; drive shared construction fixtures |

**Priority:** import a complete snapshot (attribute notmike-5 + PR SHA in README) **before** the BIPs PR deletes it. Loss of that tree is unacceptable. Hermetic CI adaptations and extensions come immediately after the snapshot lands — not as a reason to leave the original behind.

**kellnr / workshop stack:** preserved under binding folders (e.g. notes, Cargo alternative features, or `examples/bip360-workshop/`) so nothing is lost; default `just ci` runs **extended hermetic** construction + binding e2e, not the private registry.

---

## Goals (0.4.1)

1. **Algorithm vectors home** — single tree under `tests/vectors/` for secp / ML-DSA-44 / SLH-DSA-SHA2 golden artifacts (all binding languages).
2. **Binding test polish** — Python ML/SLH golden tamper; Node/WASM import rewires only.
3. **`just ci`** already mirrors GHA on host (done on branch); keep after rehome.
4. **P2MR multi-language e2e** — **copy** notmike-5’s PR work into the appropriate FFI binding folders, host construction JSON as **living test vectors**, then **extend** runners so they exercise our bindings (not a throwaway port).
5. **Version 0.4.1** + publish to **canonical public names**:
   - npm: [`bitcoinpqc`](https://www.npmjs.com/package/bitcoinpqc) (unscoped — not `@jbride/*`)
   - PyPI: [`bitcoinpqc`](https://pypi.org/project/bitcoinpqc/)
   - crates.io: `bitcoinpqc` (Rust)

## Out of scope

- libbitcoinpqc **upstream feature** work (submodule `053e954` is fine).
- Changing **algorithm** golden fixture crypto bytes (secp / ML-DSA / SLH-DSA JSON).
- Making private `kellnr-denver-space` required for default CI (workshop may be preserved; not a hard CI dep).
- Automating npm/PyPI/crates.io publish (manual checklist only).

---

## Already done (staged on wasm-tests)

| Area | Status |
|------|--------|
| `just ci` | Mirrors blocking GHA jobs; local `vectors-in-sync` path-scoped |
| Rust crypto path | All three algos via libbitcoinpqc FFI; no duplicate secp crate |
| libsecp pin | Node/WASM/`build.rs` → v0.7.1 matching CMake |
| Fuzz | Nightly smoke; tolerant invalid secp secrets |
| Nix | Expanded `flake check` |
| Submodule | `053e954` |

**Hygiene before/with release commits:** decide fate of staged `fuzz/artifacts/keypair_generation/crash-…` (all-zero payload). Prefer **do not ship** crash binaries unless policy keeps artifacts; if kept, document as regression sample for zero-secret keygen.

---

## Version: 0.4.1 + canonical package names

Bump manifests `0.4.0` → `0.4.1` and **align published names** with the public URLs (not `@jbride` scope):

| Artifact | Manifest files | Publish as | Registry URL |
|----------|----------------|------------|--------------|
| Rust | `Cargo.toml` | `bitcoinpqc` | crates.io |
| Python | `python/setup.py`, `python/bitcoinpqc/__init__.py` | `bitcoinpqc` | [pypi.org/project/bitcoinpqc](https://pypi.org/project/bitcoinpqc/) |
| **JS (Node + browser)** | see “One npm package” below | **`bitcoinpqc`** (unscoped) | [npmjs.com/package/bitcoinpqc](https://www.npmjs.com/package/bitcoinpqc) |
| Nix | `flake.nix` | n/a | — |

**Registry state today (for awareness):** unscoped npm `bitcoinpqc` is at **0.1.0**; `@jbride/bitcoinpqc` / `@jbride/bitcoinpqc-wasm` are older scoped publishes; PyPI `bitcoinpqc` is at **0.1.0** (README’s 0.4.0 link is ahead of registry). Ship **0.4.1** as the new latest on the **unscoped / PyPI** names. Confirm npm publish rights on unscoped `bitcoinpqc` before release day.

### Why Node and WASM were separate — and what we do instead

| Tree today | Role |
|------------|------|
| `nodejs/` | **Native** Node addon (`node-gyp`, C++ + C sources) — requires compile/install on the host |
| `wasm/` | **Emscripten** build — works in **browsers** (and Node without a native toolchain) |

That split is build-tech, not product identity. Users should install **one** JS package: **`bitcoinpqc`**.

**0.4.1 approach (recommended):** **single unscoped npm package `bitcoinpqc`** assembled from both trees:

```
bitcoinpqc@0.4.1
  .                 → native Node API (from nodejs/), default for Node
  ./wasm            → WASM/browser entry (from wasm/dist/), via package.json "exports"
  browser field     → prefer WASM entry when bundlers resolve for browser
```

Implementation sketch:

1. Rename `nodejs/package.json` `"name"` from `@jbride/bitcoinpqc` → **`bitcoinpqc`**.
2. Publish script (or monorepo pack step) includes `wasm/dist/*` (or copies into `nodejs/wasm/` / `nodejs/dist-wasm/`) and sets:

   ```json
   "exports": {
     ".": { "node": "./dist/index.js", "browser": "./wasm/index.js", "default": "./dist/index.js" },
     "./wasm": "./wasm/index.js"
   }
   ```

3. **Stop publishing** `@jbride/bitcoinpqc` and `@jbride/bitcoinpqc-wasm` as the primary line (optional one-line deprecation readme on those packages later; not blocking).
4. Update all docs/install snippets: `npm install bitcoinpqc` — not `@jbride/…`.
5. Part D JS examples/tests import `bitcoinpqc` / `bitcoinpqc/wasm` accordingly; BIPs JS that used `@jbride/bitcoinpqc-wasm` retargets here.

Keep **`nodejs/` and `wasm/` as separate source trees** in the repo (different toolchains). Only the **published npm identity** collapses to one package.

**Fallback** if dual-export packaging slips the release: still publish **unscoped `bitcoinpqc`** for the Node native binding as the canonical JS package; ship WASM as the same package’s `./wasm` in a fast follow — do **not** resume `@jbride/*` as the documented default.

---

## Part A — Algorithm vectors home

### Target layout

```
tests/vectors/
  README.md
  fixtures/                 # from tests/fixtures/
    secp256k1_bip340_row0.json
    ml_dsa_44_golden.json
    slh_dsa_sha2_golden_vectors.json
  rust/                     # generated *.rs
  python/                   # generated *_golden_vectors.py
  nodejs/                   # generated *.js + *.d.ts
  wasm/                     # generated *.js
  p2mr/                     # Part D — living BIP 360 construction vectors (not sync-golden-vectors)
    README.md               # provenance (PR #2202 SHA), how languages consume them
    fixtures/
      p2mr_construction.json
      p2mr_pqc_construction.json
```

C headers still: `libbitcoinpqc/tests/vectors/*.h` (sync output only).

P2MR **implementation / e2e code** lives in each FFI tree (Part D), not only under `tests/vectors/`.

### Steps

1. Add `tests/vectors/README.md` (algorithm vs P2MR; regen; BIPs handoff).
2. Move `tests/fixtures/` → `tests/vectors/fixtures/`.
3. Update `scripts/sync-golden-vectors.py` paths + headers.
4. `make sync-vectors` / `just vectors-in-sync` (force `LIBBITCOINPQC_SRC` to submodule for C headers).
5. Rewire imports: `tests/algorithm_tests.rs`, `tests/serialization_tests.rs`, Python/Node/WASM suites.
6. Delete old golden copies under `python/tests/`, `nodejs/tests/`, `wasm/test/`, flat `tests/vectors/*.rs`.
7. Simplify path lists in `justfile`, `flake.nix`; update `Makefile` sync help text.

**Critical files:** `scripts/sync-golden-vectors.py`, `justfile`, `flake.nix`, `Makefile`, test import sites above.

**Risk note:** GHA `vectors-in-sync` remains **full-tree** diff; local just remains **scoped**. Always export `LIBBITCOINPQC_SRC` or use `just vectors-in-sync` so C headers land in the submodule, not a standalone checkout.

---

## Part B — Binding test polish (algorithm)

| Binding | Work |
|---------|------|
| **Python** | After import rewire: add **tamper-on-verify** on ML-DSA and SLH-DSA **golden** tests (today only success path). |
| **Node** | Import rewire only — suite already strong. |
| **WASM** | Import rewire only — golden + e2e + tamper already present. |

Bar: Rust `tests/algorithm_tests.rs` (golden pk/sig/verify, e2e + tamper, bad inputs).

---

## Part C — Version + publish prep

```bash
make sync-vectors   # or just vectors-in-sync
just vectors-in-sync
just ci
```

Manual publish — **canonical public packages only**:

| Order | Command | Artifact |
|-------|---------|----------|
| 1 | `cargo publish` | crates.io `bitcoinpqc` 0.4.1 |
| 2 | `cd python && python -m build && twine upload dist/*` | **[PyPI `bitcoinpqc`](https://pypi.org/project/bitcoinpqc/) 0.4.1** |
| 3 | Assemble dual-export package → `npm publish` from the Node publish root | **[npm `bitcoinpqc`](https://www.npmjs.com/package/bitcoinpqc) 0.4.1** (native + `./wasm`) |

Do **not** document or rely on `@jbride/bitcoinpqc` / `@jbride/bitcoinpqc-wasm` for this release.

Docs after publish:

- README: `npm install bitcoinpqc`, `pip install bitcoinpqc`, cargo crate `bitcoinpqc`.
- Links: [npm bitcoinpqc](https://www.npmjs.com/package/bitcoinpqc), [PyPI bitcoinpqc](https://pypi.org/project/bitcoinpqc/) (version 0.4.1 pages after upload).
- README: `tests/vectors/` (algorithm) + `tests/vectors/p2mr/` (construction).
- Fix repo URLs in package metadata if they still point at obsolete `bitcoin/libbitcoinpqc` paths.

**BIPs paste (correct owner):**

> Algorithm integration tests and golden vectors for BIP 360 tapscript signature algorithms (secp256k1 Schnorr, ML-DSA-44, SLH-DSA-SHA2-128s), plus P2MR construction e2e, live in [jbride/libbitcoinpqc-bindings `tests/vectors/`](https://github.com/jbride/libbitcoinpqc-bindings/tree/main/tests/vectors) (path after merge to main).

Tag `v0.4.1` if that matches how 0.4.0 was released.

---

## Part D — P2MR: preserve BIPs work, host living vectors, extend for bindings

**Urgency:** [bitcoin/bips#2202](https://github.com/bitcoin/bips/pull/2202) will drop this tree soon. Snapshot **must** land here first. Pull PR head via `gh` (or user-provided tarball) at implement time; record **PR number + commit SHA** in `tests/vectors/p2mr/README.md`.

**These are real test vectors** — not disposable demo data. They live under `tests/vectors/p2mr/` and continue to inform construction + multi-lang e2e as BIP 360 evolves. Algorithm goldens (`fixtures/` for secp/ML/SLH) stay separate; P2MR construction is the script-tree / control-block layer on top.

### D.0 Snapshot inventory (copy everything that matters)

From `bip-0360/ref-impl/` on the PR:

| BIPs path | Destination in bindings (preserve) |
|-----------|-------------------------------------|
| `common/tests/data/p2mr_construction.json` | `tests/vectors/p2mr/fixtures/` |
| `common/tests/data/p2mr_pqc_construction.json` | `tests/vectors/p2mr/fixtures/` |
| `python/p2mr.py` (+ any python helpers) | **Python FFI tree** — e.g. `python/p2mr/` or `python/bitcoinpqc/p2mr/` (decide on public vs test-only API at implement; default: importable under `python/` next to package, tested by `python/tests/`) |
| `rust/tests/p2mr_construction.rs`, `p2mr_pqc_construction.rs` | **Rust tree** — e.g. `tests/p2mr/` (integration tests) |
| `rust/src/*` construction-related modules (`data_structures`, helpers used by those tests) | **Rust tree** — e.g. `tests/p2mr/support/` or `src/p2mr/` if we export helpers; prefer test-local modules unless product wants public P2MR API |
| `rust/` spend/workshop extras (docs, signet scripts, kellnr Cargo.toml) | **Preserve** under e.g. `examples/bip360-workshop/rust/` or `docs/bip360/` so nothing is lost; not required for default CI |
| `js/src/p2mr-example.ts` | **Node tree** — e.g. `nodejs/examples/p2mr-example.ts` and/or logic under `nodejs/tests/p2mr/` |
| `js/src/test-npm-pqc-package.js` | **WASM/Node** — fold useful bits into existing wasm tests or keep as example under `wasm/examples/`; retarget current package APIs |

Do **not** leave the only copy on GitHub’s PR branch.

### D.1 Living shared fixtures

- Both JSONs → `tests/vectors/p2mr/fixtures/` (canonical).
- `tests/vectors/p2mr/README.md`: algorithm vs P2MR distinction; provenance; “languages **must** load from here”; how to update when BIP vectors change.
- **Not** generated by `sync-golden-vectors.py` (different domain). Hand-maintained; treated as source of truth for construction e2e going forward.

### D.2 Python binding folder — preserve + extend

1. **Copy** `p2mr.py` into the Python binding area (full file history intent: preserve notmike-5 logic, bech32, merkle, control blocks, `run_single_test`).
2. **Rewire** fixture paths → `tests/vectors/p2mr/fixtures/`.
3. **Wire CI:** `python/tests/test_p2mr_*.py` run both construction vector sets via existing `just python`.
4. **Extend:** where vectors/scripts involve PQC leaves, use **this repo’s** `bitcoinpqc` Python binding (current algorithms, entropy rules) rather than leaving orphan demos. Construction assertions remain the primary bar from the original tests.

### D.3 Rust binding folder — preserve + extend

1. **Copy** construction / pqc construction tests and the support modules they need into the Rust binding tree.
2. **Rewire** to `tests/vectors/p2mr/fixtures/` (`include_str!` or read relative to `CARGO_MANIFEST_DIR`).
3. **Extend for hermetic CI + our crate:**
   - Default `cargo test` / `just rust` must **not** require `kellnr-denver-space`.
   - Replace old registry `bitcoinpqc` 0.3.0 with path/workspace **this crate** where signatures are needed.
   - Construction-only paths: implement or keep pure merkle/control-block logic so tests pass without private `bitcoin-p2mr-pqc` if that crate is unavailable — **but keep the original sources** (e.g. under `examples/bip360-workshop/`) for reference until a public dep exists.
4. Prefer extended tests as first-class integration tests under `tests/p2mr/`.

### D.4 Node / WASM binding folders — preserve + extend

1. **Copy** `p2mr-example.ts` into Node examples/tests; copy any JS helpers needed for construction e2e.
2. **Rewire** fixtures to shared JSON; add Jest (or node test) suite under `nodejs/tests/` that asserts the same construction properties as Python.
3. **Extend:** depend on **in-repo / published `bitcoinpqc` 0.4.x** (native and/or `bitcoinpqc/wasm`); fix SLH-DSA-SHAKE → **SLH-DSA-SHA2** naming; drop obsolete smoke that duplicates `wasm/test/*` once covered.
4. Optional: keep bitcoinjs-lib P2MR example as an **example** if `@jbride/bitcoinjs-lib` is available on npm; do not block CI on private packages.

### D.5 CI wiring

- `just python` / `just rust` / `just nodejs` (and wasm where applicable) run extended P2MR construction e2e against shared fixtures.
- `no-skipped-tests` applies — no `#[ignore]` / `it.skip` for the living vector suites.
- Algorithm `vectors-in-sync` remains algorithm-only; P2MR JSON is committed hand-maintained living data.

### D.6 BIPs follow-up (human)

After snapshot + green CI: comment on #2202 that the work is preserved and extended under [jbride/libbitcoinpqc-bindings](https://github.com/jbride/libbitcoinpqc-bindings) (`tests/vectors/p2mr/` + per-language binding folders); safe to remove from the BIPs PR in favor of links.

---

## Suggested commit / PR ordering

1. **Polish base** — staged FFI/CI/fuzz/pin (drop or justify crash artifact).
2. **Part A** — algorithm vectors rehome (stable `tests/vectors/` layout).
3. **Part D.0–D.1** — **snapshot** BIPs ref-impl + living P2MR fixtures (**do early**; before PR deletion).
4. **Part D.2–D.5** — place into FFI folders, rewire fixtures, extend for our bindings, green CI.
5. **Part B** — Python algorithm golden tamper (can parallel D after A).
6. **Part C** — version 0.4.1 + README + **PyPI/crates/npm publish** + BIPs paste.

If BIPs deletion is imminent, **reorder: D.0 snapshot immediately** (even before Part A), then rehome algorithm vectors.

---

## Risks

| Risk | Mitigation |
|------|------------|
| BIPs PR deletes tree before we copy | **D.0 first** if needed; snapshot commit with PR SHA |
| Incomplete copy (only “minimal port”) | Inventory table in D.0; review checklist vs PR file list |
| kellnr / private deps break CI | Preserve workshop sources; default CI = hermetic extended suite on shared vectors |
| Jest/Node fixture paths | Repo-relative from binding test dirs |
| Python package surface | Document whether `p2mr` is public API or test/example module |
| Vector drift | Living vectors owned here after import; BIPs text may lag — README notes ownership |
| GHA full-tree vs local scoped algorithm vectors | Unchanged; P2MR JSON not part of sync generator |
| Standalone libbitcoinpqc path in sync script | `just vectors-in-sync` forces submodule |
| Scope too large for one PR | Snapshot commit + follow-up extend commits still OK; **do not skip snapshot** |
| No publish rights on unscoped npm `bitcoinpqc` | Verify `npm owner ls bitcoinpqc` before release; transfer/access if needed |
| Dual-export npm pack is fiddly | Keep monorepo trees; one publish script; fallback = unscoped native only + fast-follow `./wasm` |
| Old `@jbride/*` consumers | README migration note; optional deprecation of scoped packages later |

---

## Verification

```bash
just vectors-in-sync
just ci
# Algorithm: no leftover goldens under old paths; tests/fixtures gone
# P2MR: fixtures present under tests/vectors/p2mr/fixtures/
# P2MR: python + rust + node (and wasm as applicable) pass construction e2e on both JSON sets
# Preserve check: PR #2202 ref-impl files accounted for in binding folders or examples/bip360-workshop/
# Manual: tests/vectors/README.md + tests/vectors/p2mr/README.md (provenance SHA)
```

---

## Follow-ups (post-0.4.1)

- Deprecate `@jbride/bitcoinpqc` / `@jbride/bitcoinpqc-wasm` on npm if still visible (README → install `bitcoinpqc`).
- Wire public `@jbride/bitcoinjs-lib` / rust-bitcoin P2MR crates into CI when hermetic on public registries.
- Promote workshop spend/signet material from preserved examples if product needs it.
- macOS CI matrix legs; optional `just ci-benchmark`.

---

## Approval

- [ ] Approve → implement Parts A–D + 0.4.1 bump as above
- [ ] Approve A–C only; defer Part D to 0.4.2
- [ ] Revise further
- [ ] Abandon
