# Join: `BIP-360` hyphen inventory (display form)

**Repo:** `/home/hunter/Projects/surmount/libbitcoinpqc-bindings`
**Date:** 2026-08-01
**Scope:** exact string `BIP-360` outside markdown; operator prefers human-facing `BIP 360` (space). Markdown already clean.
**Mode:** read-only inventory; no files changed.

## Counts

| Form | Count (rg whole tree; excludes .git noise; includes build/node_modules only if matched) |
|------|----------------------------------------------------------------------------------------|
| `BIP-360` | **23** hits in **14** files (none under `target/`, `node_modules/`, `build/`, `*.md`) |
| `BIP 360` | **35** hits (already-correct space form; mostly README/plan + a few runtime strings) |
| `BIP-360` in `*.md` | **0** (operator already fixed markdown) |

Notes:

- Path/slug forms like `bip360-workshop`, `bip360_test_vectors`, `BIP360_tests`, URL path `bip-0360`, and print `BIP-0360` are **out of scope** for this exact-string inventory (different tokens).
- No test asserts on the literal `"BIP-360"` (no `assert`/`expect`/`toContain` on that string).

---

## Category summary

| Category | Hits | Action theme |
|----------|------|--------------|
| Human-facing prose (main tree) | 9 | **safe rename** → `BIP 360` |
| Human-facing prose (workshop snapshot) | 12 | **careful** (archival ref-impl) or safe rename if scrubbing workshop text |
| CI / linter denylist patterns | 2 | **leave** (must match forbidden form) |
| Code identifiers / assert constants | 0 | n/a |
| Generated vectors / build artifacts | 0 | n/a |
| Submodule project content (`*.md`/`*.h`/`*.c`/…) | 0 live hits | already clean; only denylist |

---

## 1. CI / linter patterns (leave)

These **forbid** `BIP-360` in scanned sources. The hyphen **must stay** inside the `-e` pattern. They are not human display copy.

| Path | Line | Context | Action |
|------|------|---------|--------|
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/libbitcoinpqc/justfile` | 60 | `worktree-naming` recipe: `rg … -e 'BIP-360' \` among denylist (`QuBit`, `P2QRH`, `P2TSH`, …) | **leave** — pattern is the ban list |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/libbitcoinpqc/flake.nix` | 117 | `checks.naming` Nix derivation: same `-e 'BIP-360'` denylist | **leave** |

**Scan globs (submodule only):** `*.md`, `*.h`, `*.c`, `*.yml`, `*.yaml`, `CMakeLists.txt`, `build.sh`, `Makefile`; excludes `dilithium/**`, `sphincsplus/**` (and justfile also excludes `build/**`, `result/**`).

**Inherited by bindings flake:** root `flake.nix` `checks` inherits `upstreamChecks.naming` (so submodule naming gate runs under bindings CI). Root `justfile` has **no** `BIP-360` naming scrub for Python/Rust/JS/adoc.

Comment already correct: `libbitcoinpqc/flake.nix:100` says `BIP 360` (space) in prose about the gate.

---

## 2. Main tree — human-facing prose (safe rename)

Hand-maintained. No CI denylist currently covers these extensions in the main tree. Renaming display strings/docs will not break tests (prints only / module docs).

| Path | Line | Short context | Action |
|------|------|---------------|--------|
| `scripts/sync-golden-vectors.py` | 17 | Module docstring: “does not touch BIP-360 P2MR construction fixtures” | **safe rename** → `BIP 360` |
| `python/p2mr/__init__.py` | 1 | Package docstring: `"""BIP-360 P2MR construction reference…` | **safe rename** |
| `python/p2mr/p2mr.py` | 263 | Section comment: `# BIP-360 Test Code` | **safe rename** |
| `python/p2mr/p2mr.py` | 328 | `print(f"\nBIP-360 Test Vector {test_num}…")` | **safe rename** (stdout only; not asserted) |
| `python/p2mr/p2mr.py` | 432 | Docstring: `"""Run BIP-360 P2MR construction test vectors…` | **safe rename** |
| `python/p2mr/p2mr.py` | 437 | `print(… "BIP-360 tests passed successfully.")` | **safe rename** |
| `python/tests/test_p2mr_construction.py` | 1 | Module docstring: `"""BIP-360 P2MR construction e2e…` | **safe rename** |
| `tests/p2mr_construction.rs` | 1 | Crate/module doc: `//! Hermetic BIP-360 P2MR construction e2e…` | **safe rename** |
| `wasm/examples/test-npm-pqc-package.js` | 145 | `console.log('… designed for BIP-360 and the Bitcoin QuBit soft fork.')` | **safe rename** for `BIP-360`; **also** note stale `QuBit` wording (same line) |

Related identifiers on same paths (**leave** — not display form):

- `python/p2mr/p2mr.py` / workshop: function name `BIP360_tests` (no hyphen).
- `python/tests/test_p2mr_construction.py:53`: method `test_bip360_tests_helper` (no hyphen).
- Print also uses `BIP-0360` (zero-padded BIP number style) at `p2mr.py:433` — separate style choice, not `BIP-360`.

---

## 3. Workshop snapshot — human-facing (`examples/bip360-workshop/`)

**Provenance:** archival snapshot from `bitcoin/bips#2202` (`PROVENANCE.md`; commit `3e3e4347…`). Living fixtures live under `tests/vectors/p2mr/`; this tree is workshop/spend material.

| Path | Line | Short context | Action |
|------|------|---------------|--------|
| `examples/bip360-workshop/python/p2mr.py` | 263 | `# BIP-360 Test Code` | **careful** if preserving upstream snapshot wording; else **safe rename** like main `python/p2mr/p2mr.py` |
| same | 328 | print `BIP-360 Test Vector` | same |
| same | 424 | docstring `Run all BIP-360 Test Vectors` | same |
| same | 432 | print `BIP-360 tests passed successfully` | same |
| `examples/bip360-workshop/js/src/test-npm-pqc-package.js` | 145 | console.log BIP-360 + QuBit (near-duplicate of `wasm/examples/…`) | **careful** / safe rename + QuBit scrub |
| `examples/bip360-workshop/rust/docs/p2mr-signet-workshop.adoc` | 10 | Welcome to the BIP-360 / P2MR workshop | **careful** (adoc human prose; workshop may still say P2TSH historically) |
| same | 18 | “alluded to in BIP-360” | **careful** → prefer `BIP 360` if scrubbing display |
| same | 86 | “reference implementation for BIP-360” | **careful** |
| same | 148 | “as per BIP-360” | **careful** |
| same | 182 | “this BIP-360 reference implementation” | **careful** |
| `examples/bip360-workshop/rust/docs/p2mr-end-to-end.adoc` | 72 | “as per BIP-360” | **careful** |
| same | 116 | “this BIP-360 reference implementation” | **careful** |

**Not hits but nearby (leave unless scrubbing URLs/hosts):** `bip-360` URL slugs, `bip360.org`, `signet.bip360.org`, `bip360@…` prompt text, directory name `bip360-workshop`.

Workshop `js/README.adoc` already uses `BIP 360` (space).

---

## 4. Submodule vs main tree

| Tree | `BIP-360` content hits | Notes |
|------|------------------------|--------|
| **Main bindings** | 21 content hits (scripts, python, rust test doc, wasm example, workshop) | No main-tree naming gate on `.py`/`.rs`/`.js`/`.adoc` |
| **`libbitcoinpqc/` submodule** | 0 content hits; 2 denylist pattern lines only | `naming` check would fail if `BIP-360` reappeared in scanned globs |
| **Generated** (`tests/vectors/{rust,python,nodejs,wasm}/`, `*.h` vectors, `dist/`, `target/`) | 0 | `sync-golden-vectors.py` explicitly does not touch P2MR fixtures |

---

## 5. Already-correct `BIP 360` (context)

**35** occurrences of the preferred space form, including:

- Root / package docs: `README.md`, `python/README.md`, `nodejs/README.md`, `wasm/README.md`, `libbitcoinpqc/README.md`, `tests/vectors/**/README.md`, `examples/bip360-workshop/PROVENANCE.md`, rust workshop README
- Metadata: `Cargo.toml` description, root `flake.nix` package description
- Runtime already fixed: `examples/basic.rs`, `wasm/test/test-*.js`, `wasm/index.html`
- C header comment: `libbitcoinpqc/include/libbitcoinpqc/bitcoinpqc.h`
- Plan / internal: `plan.md`

These are the model for remaining human-facing renames.

---

## 6. Recommended scrub order (if implementing later)

1. **Main living sources (safe, high value):**
   `python/p2mr/*`, `python/tests/test_p2mr_construction.py`, `tests/p2mr_construction.rs`, `scripts/sync-golden-vectors.py`, `wasm/examples/test-npm-pqc-package.js` (consider dropping “QuBit soft fork” wording too).
2. **Workshop:** decide snapshot fidelity vs house style; if scrubbing, align `examples/bip360-workshop/python/p2mr.py`, JS example, and rust `*.adoc` prose hits.
3. **Never change:** `libbitcoinpqc/justfile` / `libbitcoinpqc/flake.nix` denylist `-e 'BIP-360'` lines.
4. **Optional follow-up:** extend main-tree naming scrub (if desired) to `.py`, `.rs`, `.js`, `.adoc` so hyphen form cannot regress; keep denylist pattern itself excluded from self-match (same as justfile comment: “not justfile/flake.nix denylist pattern lines”).

---

## Executive table (operator)

| Bucket | Files | Hits | Action |
|--------|------:|-----:|--------|
| Main prose/docs/prints | 6 | 9 | Rename `BIP-360` → `BIP 360` |
| Workshop snapshot | 4 | 12 | Careful (archive) or same rename |
| Submodule CI denylist | 2 | 2 | Leave hyphen in pattern |
| Markdown | — | 0 | Already done |
| Identifiers / asserts / generated | — | 0 | Nothing to do for this string |

**Bottom line:** 23 residual `BIP-360` hits; **2 must stay** (lint denylist); **9 main-tree** safe display renames; **12 workshop** optional/careful. Markdown already at `BIP 360`. No test locks the hyphenated display string.
