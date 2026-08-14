# Join: BIP-360 → BIP 360 display-form scrub

**Status:** done
**Scope:** human-facing prose only; surgical per-file edits (no bulk replace)

## What changed

Exact substring `BIP-360` → `BIP 360` in prose (docs, comments, print/console strings). Left alone: identifiers (`BIP360_tests`), `BIP-0360` / `bip-0360` paths, `BIP-340`, denylist patterns.

## Files touched (11)

### Main tree (6)
| File | Sites |
|------|--------|
| `scripts/sync-golden-vectors.py` | docstring |
| `python/p2mr/__init__.py` | module docstring |
| `python/p2mr/p2mr.py` | comment, print, docstring (3) |
| `python/tests/test_p2mr_construction.py` | module docstring |
| `tests/p2mr_construction.rs` | module doc comment |
| `wasm/examples/test-npm-pqc-package.js` | console.log prose |

### Workshop (5)
| File | Sites |
|------|--------|
| `examples/bip360-workshop/python/p2mr.py` | comment, print, docstring (4) |
| `examples/bip360-workshop/js/src/test-npm-pqc-package.js` | console.log prose |
| `examples/bip360-workshop/rust/docs/p2mr-end-to-end.adoc` | 2 prose hits |
| `examples/bip360-workshop/rust/docs/p2mr-signet-workshop.adoc` | 5 prose hits |

## Intentionally not touched
- `*.md` (already done upstream)
- `libbitcoinpqc/justfile` and `libbitcoinpqc/flake.nix` (denylist must keep literal `BIP-360`)
- Generated vectors, paths, identifiers

## Verify

```text
rg -n 'BIP-360' --glob '!.git/**' --glob '!target/**' --glob '!node_modules/**'
```

**Remaining hits (expected only):**
```
libbitcoinpqc/justfile:60:        -e 'BIP-360' \
libbitcoinpqc/flake.nix:117:              -e 'BIP-360' \
```

**Python tests:** `python3 -m unittest tests.test_p2mr_construction -v` from `python/` → 3 passed (OK). Print output now shows `BIP 360 Test Vector …` / `BIP 360 tests passed`.

No git commit.
