# Join: bitcoin/bips#2220 fixture context

**PR:** https://github.com/bitcoin/bips/pull/2220
**Author:** jeanpablojp (not jbride)
**Commit:** `1baf0e71bb25445fa348c3cd6b92f68adac34e8b`
**Status (as of research):** open, labels Pending acceptance + Bug fix; jonatack LGTM, pinged cryptoquick / EthanHeilman
**Scope:** single file only

## What the fix is (plain English)

Two published bugs in the **PQC** P2MR construction vectors, found by jeanpablojp while implementing BIP-360 against the vectors:

1. **Swapped leaf hash list order** in two three-leaf vectors: `intermediary.leafHashes[0]` and `[2]` were swapped vs depth-first walk of `scriptTree`. Merkle root, scriptPubKey, address, and control blocks were already correct (depth-first). Leftover ordering issue related to what #2202 fixed for control blocks.
2. **Wrong control-block first byte** in `p2mr_different_version_leaves`: leaf has `leafVersion` 250 (`0xfa`), but `scriptPathControlBlocks[1]` started with `c1` instead of `fb` (upper 7 bits = version, low bit set). Path bytes and leafHashes/root already matched `0xfa`; only the control version nibble was wrong. A spender using the bad control block would rehash under `0xc0` and fail the merkle check.

After the PR, every success vector in that file recomputes cleanly from the BIP formulas.

## File(s)

| Path in BIPs PR | Local living copy | Local archive snapshot |
|-----------------|-------------------|------------------------|
| `bip-0360/ref-impl/common/tests/data/p2mr_pqc_construction.json` | `tests/vectors/p2mr/fixtures/p2mr_pqc_construction.json` | `examples/bip360-workshop/common/tests/data/p2mr_pqc_construction.json` |

Diff size: +5 / −5 (one JSON file).

### Concrete expected changes

| Vector id | Field | Change |
|-----------|--------|--------|
| `p2mr_different_version_leaves` | `expected.scriptPathControlBlocks[1]` first byte | `c1…` → `fb…` (rest of hex path unchanged). Leaf version stays **250 / 0xfa**; control byte becomes **0xfb**. |
| `p2mr_three_leaf_complex` | `intermediary.leafHashes` | swap index 0 ↔ 2 (middle hash stays) |
| `p2mr_three_leaf_alternative` | `intermediary.leafHashes` | swap index 0 ↔ 2 (middle hash stays) |

No change to merkle roots, scriptPubKeys, addresses, leafVersion fields, or classic tree structure.

## Classic `p2mr_construction.json` in scope?

**No.** PR #2220 does not touch classic `p2mr_construction.json`. jeanpablojp’s Core harness note: all nine classic vectors already pass; bugs were only in the PQC file. Doc-side issues went to **#2221** separately.

## Maintainership / provenance (this repo)

- **Canonical bindings home (operator/plan):** [jbride/libbitcoinpqc-bindings](https://github.com/jbride/libbitcoinpqc-bindings). Living P2MR fixtures under `tests/vectors/p2mr/`; workshop archive under `examples/bip360-workshop/`.
- **Local fixture provenance (pre-#2220 snapshot):** copied from [bitcoin/bips#2202](https://github.com/bitcoin/bips/pull/2202) at commit `3e3e4347e2777abdb0cc131b7f8b4f5a37ec49ba` (`tests/vectors/p2mr/README.md`, `examples/bip360-workshop/PROVENANCE.md`).
- **This PR author:** jeanpablojp fixing published BIP ref-impl vectors, not jbride. jbride maintains the bindings / workshop stack that *consumes* those vectors; BIPs PR ownership is jeanpablojp + BIP champions (cryptoquick / EthanHeilman per review ping).
- **CI roles (local README):** classic `p2mr_construction.json` = required multi-language bar; `p2mr_pqc_construction.json` = preserved living data (presence checked; full equality not required of pure-Python ref).

## Related

- #2202: original construction vectors / ordering work this leftover sits on.
- #2221: jeanpablojp doc-side problems (out of scope for this fixture PR).
- jeanpablojp vendored BIPs at `0fdf6ffdbb394a73c80978ae647322ceda8b9337` for Core tests until #2220 lands.
