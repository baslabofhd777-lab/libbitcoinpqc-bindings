# Fixture SoT inventory — P2MR / BIP-0360 construction

Date: 2026-07-31
Scope: `p2mr*_construction*.json` and consumers in this repo.

## Fixture copies (absolute paths)

| Path | Role | Lines | Top-level shape |
|------|------|------:|-----------------|
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json` | **Living CI / multi-lang SoT (docs)** | 252 | `{ version: 1, test_vectors: [7] }` |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/examples/bip360-workshop/common/tests/data/p2mr_pqc_construction.json` | Archival workshop snapshot | 252 | same |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_construction.json` | **Living multi-lang bar** | 303 | `{ version: 1, test_vectors: [9] }` |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/examples/bip360-workshop/common/tests/data/p2mr_construction.json` | Archival workshop snapshot | 303 | same |

No other `*p2mr*pqc*construction*` JSON under the repo. No path-symlinks between the two trees (duplicate files). Not covered by `scripts/sync-golden-vectors.py` / `make sync-vectors` (algorithm goldens only). Sole git submodule is `libbitcoinpqc` (C lib), unrelated to these fixtures.

### Local copies vs each other

- **`p2mr_pqc_construction.json`:** the two local paths match (same structure, vector IDs, sample hashes/control blocks including the `c13bb0…` second control block). Treat as **content-identical duplicates**.
- **`p2mr_construction.json`:** the two local paths match on structure and endpoints. Treat as **content-identical duplicates**.

### Local vs target upstream URL

Target:
`https://github.com/jeanpablojp/bips/blob/1baf0e71bb25445fa348c3cd6b92f68adac34e8b/bip-0360/ref-impl/common/tests/data/p2mr_pqc_construction.json`

Fetched raw at commit `1baf0e71…` and compared:

| File | Differs from jeanpablojp@1baf0e71? | Brief |
|------|-----------------------------------|--------|
| **p2mr_pqc_construction.json** | **Yes** | Same 7 IDs / OP_SUBSTR + `priv_key` style. Material: `p2mr_different_version_leaves` leaf-250 control block is local `c13bb0…` vs upstream `fb3bb0…` (leaf-version byte). `three_leaf_*` leafHashes **array order** differs; merkle roots / most control blocks match. |
| **p2mr_construction.json** | **Yes (major)** | Upstream rewrote scripts to `OP_CHECKSIG` (no `priv_key`); different keys, hashes, addresses throughout. Same rough ID set + `p2mr_duplicate_leaves`. Local still OP_SUBSTR-era snapshot from PR #2202 commit `3e3e4347…`. |

Repo docs pin provenance to bitcoin/bips#2202 commit `3e3e4347e2777abdb0cc131b7f8b4f5a37ec49ba`, not jeanpablojp `1baf0e71`.

## How tests consume fixtures

### Living SoT path (all hermetic CI)

`tests/vectors/p2mr/fixtures/` — required by docs (`tests/vectors/p2mr/README.md`: “languages **must** load these files from this directory”).

| Consumer | Path resolution | What it does with p2mr_pqc |
|----------|-----------------|----------------------------|
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/p2mr_construction.rs` | `CARGO_MANIFEST_DIR` + `tests/vectors/p2mr/fixtures` | Runs **full** equality on `p2mr_construction.json`; **presence + version + non-empty** only for `p2mr_pqc_construction.json` (`p2mr_pqc_fixture_present`) |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/python/p2mr/p2mr.py` | `default_fixtures_dir()` → repo `tests/vectors/p2mr/fixtures` | CLI `__main__` runs construction fully, then optionally full run of pqc JSON if present |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/python/tests/test_p2mr_construction.py` | via `default_fixtures_dir()` | Full run of construction; pqc **present + version only** |
| `/home/hunter/Projects/surmount/libbitcoinpqc-bindings/nodejs/tests/p2mr_construction.test.ts` | `__dirname/../../tests/vectors/p2mr/fixtures` | Full run of construction; pqc **present + version only** |

CI: GHA `cargo test` exercises Rust construction; Python/Node P2MR tests via `just` / language runners when used. Algorithm `vectors-in-sync` **does not** validate P2MR JSON. `tests/p2mr/` directory exists but is empty (no extra modules).

### Archival workshop (not default CI)

| Consumer | Path | Notes |
|----------|------|--------|
| `examples/bip360-workshop/rust/tests/p2mr_pqc_construction.rs` | `include_str!("../../common/tests/data/p2mr_pqc_construction.json")` | Full rust-bitcoin / kellnr registry tests; private registry |
| `examples/bip360-workshop/rust/tests/p2mr_construction.rs` | `…/p2mr_construction.json` | same stack |
| `examples/bip360-workshop/python/p2mr.py` | relative `../common/tests/data/p2mr_construction.json` only | Does **not** load pqc JSON |

Provenance: `examples/bip360-workshop/PROVENANCE.md` — snapshot of #2202 @ `3e3e4347…`; hermetic work is under `tests/vectors/p2mr/`.

## Docs policy (current)

- **Living SoT in-repo:** `tests/vectors/p2mr/fixtures/`
- **Update rule:** hand-edit JSON there; re-run language e2e; **not** generated by `sync-golden-vectors`
- **Roles:** `p2mr_construction.json` = required multi-lang bar; `p2mr_pqc_construction.json` = preserved for workshop / rust-bitcoin; CI only requires presence for pqc
- Workshop tree = archival duplicate, not the binding SoT

## Recommended single SoT in THIS repo

**`/home/hunter/Projects/surmount/libbitcoinpqc-bindings/tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json`**
(and sibling `p2mr_construction.json` for the classic set).

Rationale: already the documented multi-lang load path; hermetic Rust/Python/Node all point here; workshop copy should stay archival or be rewired/deleted later.

For the jeanpablojp URL: treat it as **upstream refresh source**, not runtime SoT. Prefer:

1. Update `tests/vectors/p2mr/fixtures/*.json` from that commit (or a fetch script that pins SHA + records provenance in `tests/vectors/p2mr/README.md`).
2. Optionally refresh or drop the workshop duplicates so they cannot drift.
3. Do **not** have CI curl the URL on every run (non-hermetic).

Optional: small `scripts/sync-p2mr-construction.sh` that downloads pinned commit into `tests/vectors/p2mr/fixtures/` and updates the README commit field (mirror of algorithm golden discipline, separate from `sync-golden-vectors.py`).

## Open questions for parent / operator

1. **Adopt jeanpablojp@1baf0e71 for p2mr_pqc only, or both JSON files?** Upstream `p2mr_construction.json` is a full rewrite (OP_CHECKSIG); may break pure-Python/Rust hermetic equality tests that currently pass on OP_SUBSTR vectors.
2. **Promote p2mr_pqc from “presence-only” to full multi-lang equality** after adopting the `fb…` leaf-version control-block fix? Hermetic runners already implement leafVersion-aware tapleaf hashes.
3. **Workshop duplicate:** delete, symlink to living fixtures, or leave archival frozen at `3e3e4347` while living path moves to `1baf0e71`?
4. **Canonical upstream repo going forward:** jeanpablojp/bips fork vs bitcoin/bips#2202 vs another ref-impl host? Docs still cite #2202 / `3e3e4347`.
5. **Fetch script vs one-shot manual copy** when updating?
