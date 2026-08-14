# Signature algorithm test vectors

This directory holds the known-good **sign and verify** test data for the
signature algorithms used with BIP 360 tapscript: BIP-340 Schnorr, ML-DSA-44,
and SLH-DSA-SHA2-128s. Rust, Python, Node, WASM, and the C library
(`libbitcoinpqc`) all exercise these values in their tests.

This is a different concern from **P2MR construction** (building a script tree,
merkle root, address, and control blocks). Construction fixtures live under
[`p2mr/`](p2mr/). Do not mix the two.

## Where to edit the numbers

The real data lives only in the JSON files under `fixtures/`:

- `fixtures/secp256k1_bip340_row0.json` — BIP-340 row 0 Schnorr
- `fixtures/ml_dsa_44_golden.json` — ML-DSA-44
- `fixtures/slh_dsa_sha2_golden_vectors.json` — SLH-DSA-SHA2-128s

When a signature test vector needs to change, edit the matching JSON file. Do
not hand-edit the language-specific copies described below.

## Why there are also Rust / Python / JS / C files

Each language’s tests prefer native constants (Rust `const` byte slices, Python
modules, JavaScript modules, C headers with `static` arrays) rather than loading
JSON at test time. So we keep those forms in the tree, but they are **produced
by a script**, not maintained as a second independent set of fixtures.

[`scripts/sync-golden-vectors.py`](../../scripts/sync-golden-vectors.py) reads
the JSON under `fixtures/` and rewrites:

- `tests/vectors/rust/`, `python/`, `nodejs/`, and `wasm/` in this repo
- C headers under `libbitcoinpqc/tests/vectors/` (the C library, often present
  here as a git submodule)

Those generated files are committed so CI and local tests can run without an
extra code-generation step. They are still just mirrors of the JSON.

```text
  you edit →  tests/vectors/fixtures/*.json
                    |
                    v
           sync-golden-vectors.py
                    |
                    +--> language copies in this repo
                    +--> C headers in libbitcoinpqc
```

| Please do | Please do not |
|-----------|----------------|
| Change the JSON, then run the sync recipe | Treat a generated `.rs` / `.py` / `.js` / `.h` file as the place to fix a wrong byte |
| Commit the JSON and the regenerated files together | Leave generated files out of date after a JSON change |

CI and `just vectors-in-sync` run the generator and fail if the tree does not
match. If you change only the JSON and forget to sync, the gate fails. If you
change only a generated file, the next sync overwrites it and the gate fails
unless the JSON agrees.

## Directory layout

```
tests/vectors/
  fixtures/     # JSON — edit these when vector data changes
  rust/         # generated from fixtures — do not edit by hand
  python/       # generated from fixtures — do not edit by hand
  nodejs/       # generated from fixtures — do not edit by hand
  wasm/         # generated from fixtures — do not edit by hand
  p2mr/         # P2MR construction fixtures (hand-written JSON; not this script)
```

## How to regenerate

Use the just recipe when you want C headers written into the **submodule** path
that CI checks:

```bash
just vectors-in-sync
```

You can also run:

```bash
make sync-vectors
```

`just vectors-in-sync` sets `LIBBITCOINPQC_SRC` to `./libbitcoinpqc`. By
default, `make sync-vectors` may write C headers to a separate checkout at
`~/Projects/surmount/libbitcoinpqc` if that directory exists on your machine.
Prefer `just` when the change should land in this repo’s submodule.

The C headers only feed **signature** tests inside `libbitcoinpqc`. They are
not used for P2MR construction fixtures.

## Related

- P2MR construction: [`p2mr/README.md`](p2mr/README.md)
- Integration tests for these signature algorithms live in this bindings repo
  (not in the BIPs tree). Background:
  [bitcoin/bips#2202](https://github.com/bitcoin/bips/pull/2202)
