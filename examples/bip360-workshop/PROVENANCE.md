# BIP 360 ref-impl snapshot

Preserved from [bitcoin/bips#2202](https://github.com/bitcoin/bips/pull/2202) (`notmike-5` and collaborators).

- **PR:** https://github.com/bitcoin/bips/pull/2202
- **Commit:** `3e3e4347e2777abdb0cc131b7f8b4f5a37ec49ba`
- **Imported:** 2026-07-16

This tree is an archival snapshot of workshop/spend material and
registry-specific Cargo configs. It does **not** keep its own copies of the
P2MR construction JSON fixtures.

**Living construction fixtures (single load path):**
`tests/vectors/p2mr/fixtures/` (`p2mr_construction.json`,
`p2mr_pqc_construction.json`). Workshop Rust and Python tests load from that
directory. Hermetic multi-language CI also uses those files plus binding tests
under `python/`, `tests/p2mr_construction.rs`, and `nodejs/`.
