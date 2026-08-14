# BIP-360 hyphen inventory (CI / submodule / Python tests / adoc)

Scope: literal `BIP-360` (hyphen), ban/allow patterns, submodule content, Python
print-assert risk, workshop `.adoc`. Date: 2026-08-01.

## 1. CI / justfile / flake / lint — literal `BIP-360` as ban pattern

### Must-keep denylist entries (do not “fix” these strings)

| Path | Role | Pattern line |
|------|------|--------------|
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/libbitcoinpqc/justfile` | `worktree-naming` recipe; `just test` / `just ci` runs it | `-e 'BIP-360' \` (~L60) |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/libbitcoinpqc/flake.nix` | `naming-check` flake check; part of `nix flake check` | `-e 'BIP-360' \` (~L117) |

Both denylists also ban: `QuBit`, `P2QRH`/`p2qrh`, `P2TSH`/`p2tsh`,
`cryptoquick/bips`. Scan globs (content only): `*.md`, `*.h`, `*.c`, `*.yml`,
`*.yaml`, `CMakeLists.txt`, `build.sh`, `Makefile`. Excludes
`dilithium/**`, `sphincsplus/**` (and justfile also excludes `build/**`,
`result/**`). Gate scripts themselves are intentionally **not** scanned for
these substrings (comment: “not gate scripts, which list these as denylist
patterns”).

Comment in flake: “Reject deprecated BIP 360 / soft naming…” — ban is
**hyphen form** `BIP-360`, not spaced `BIP 360`.

### Parent bindings repo

| Path | `BIP-360` ban? | Notes |
|------|----------------|-------|
| Root `justfile` | **No** | No BIP-360 / naming scrub |
| Root `flake.nix` | **No denylist of its own** | Inherits submodule check: `inherit (upstreamChecks) … naming …` (~L436). Description string uses **spaced** `BIP 360` (~L125) — allowed |
| `.github/workflows/ci.yml` | **No** | No `BIP-360` / naming grep; jobs are vectors-in-sync, no-skipped-tests, test, etc. |

**Implication:** renaming hyphen → space (or other preferred form) in **bindings**
tree is free of a parent-level naming gate. The **libbitcoinpqc submodule**
still fails CI if any scanned content file contains literal `BIP-360`.

## 2. libbitcoinpqc submodule content files

Grep of submodule project content (`*.md`, `*.h`, `*.c`, etc.):

| File | Form present |
|------|----------------|
| `libbitcoinpqc/README.md` | **`BIP 360`** (space) only — passes ban |
| `libbitcoinpqc/include/libbitcoinpqc/bitcoinpqc.h` | **`BIP 360`** (space) only — passes ban |
| `libbitcoinpqc/justfile` | `BIP-360` **only as denylist pattern** |
| `libbitcoinpqc/flake.nix` | `BIP-360` **only as denylist pattern** |

**No content-file hits for hyphen `BIP-360` inside the submodule.** Naming
scrub is currently clean for this string in content.

## 3. Python print strings vs tests (exact message text)

### Live binding path: `python/p2mr/p2mr.py`

Hyphen user-facing prints:

- L328: `print(f"\nBIP-360 Test Vector {test_num}\n...")`
- L433: `print("\nRunning BIP-0360 Pay-to-Merkle-Root (P2MR) Tests...")` (note: **BIP-0360**, zero-padded)
- L437: `print(f"\n{passed}/{len(test_vectors)} BIP-360 tests passed successfully.")`

Also comments/docstrings: L263 `# BIP-360 Test Code`, L432 docstring.

### Tests: `python/tests/test_p2mr_construction.py`

- Imports `BIP360_tests`, `run_single_test` (symbol names, not message text).
- `test_p2mr_*_json`: asserts fixture version + that `run_single_test` returns
  true for each vector (failures collected by id). **No stdout capture.**
- `test_bip360_tests_helper`: `self.assertEqual(BIP360_tests(...), 0)` — return
  **count only**, not printed strings.

**Verdict: changing Python print strings that contain `BIP-360` will not break
current tests.** No `capsys`/`capfd`/`assertIn` on stdout. Docstring on the
test module also has `BIP-360` (L1) but is not asserted.

Workshop copy `examples/bip360-workshop/python/p2mr.py` has the same print
forms; no dedicated unit test tree there asserting those messages.

## 4. Workshop adoc docs (`examples/bip360-workshop/`)

### Files with literal `BIP-360` (hyphen)

| File | Approx. hits |
|------|----------------|
| `rust/docs/p2mr-signet-workshop.adoc` | L10, L18, L86, L148, L182 |
| `rust/docs/p2mr-end-to-end.adoc` | L72, L116 |

### Related adoc / workshop naming (not always hyphen form)

| File | Notes |
|------|--------|
| `js/README.adoc` | Title uses **`BIP 360`** (space) |
| Other rust docs (`development_notes.adoc`, `p2tr-end-to-end.adoc`, `stack_element_size_performance_tests.adoc`) | No `BIP-360` / `BIP 360` from this grep |

### Same workshop area (non-adoc, for context)

- Path/dir name: `examples/bip360-workshop/`
- Docker users/hosts: `bip360`, `signet.bip360.org`, `faucet.bip360.org`
- Links: `bip-0360` paths, `cryptoquick/bips` (would be **banned inside
  libbitcoinpqc content**, not under bindings workshop)
- JS smoke: `js/src/test-npm-pqc-package.js` and `wasm/examples/...` still say
  `BIP-360` and `QuBit` in console.log

Parent has **no** naming ban over workshop/docs; these are prose/archival only
relative to the C-library gate.

## 5. Other bindings-tree `BIP-360` (hyphen) content (inventory only)

Not CI denylist, safe to reword unless product wants consistency:

- `python/p2mr/{p2mr.py,__init__.py}`, `python/tests/test_p2mr_construction.py` docstring
- `tests/p2mr_construction.rs` module doc
- `scripts/sync-golden-vectors.py` comment
- `wasm/examples/test-npm-pqc-package.js`, workshop JS copy
- Workshop Python + adoc as above

Preferred **spaced** form already common in parent READMEs / Cargo.toml:
`BIP 360`.

## 6. Summary for hyphen rename work

| Area | Action if scrubbing hyphen form |
|------|----------------------------------|
| `libbitcoinpqc/justfile` + `flake.nix` denylist `-e 'BIP-360'` | **Keep** as ban patterns (or intentionally change the ban if policy changes) |
| libbitcoinpqc **content** | Already clean (`BIP 360` only) |
| Parent CI / just / flake | No hyphen ban; flake only re-exports submodule `naming` |
| Python prints | **Safe to change**; tests do not assert message text |
| Workshop adoc | Free prose; 7 lines across two adocs use hyphen form |

**Bottom line:** only must-keep hyphen literals are the two **denylist pattern
strings** under `libbitcoinpqc/`. Content and bindings do not depend on exact
print text for tests; adoc workshop docs are independent archival copy.
