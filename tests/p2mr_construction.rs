//! Hermetic BIP 360 P2MR construction e2e against living vectors.
//! Logic mirrors `python/p2mr/p2mr.py` (notmike-5 / bitcoin/bips#2202).
//!
//! Both `p2mr_construction.json` and `p2mr_pqc_construction.json` get full
//! expected-value checks (leaf hashes, merkle root, scriptPubKey, address,
//! control blocks / leafVersion), not presence-only.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/vectors/p2mr/fixtures")
}

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn tagged_hash(tag: &str, data: &[u8]) -> [u8; 32] {
    let tag_hash = sha256(tag.as_bytes());
    let mut buf = Vec::with_capacity(64 + data.len());
    buf.extend_from_slice(&tag_hash);
    buf.extend_from_slice(&tag_hash);
    buf.extend_from_slice(data);
    sha256(&buf)
}

fn h2b(h: &str) -> Vec<u8> {
    hex::decode(h).expect("valid hex")
}

fn serialize_varbytes(b: &[u8]) -> Vec<u8> {
    let n = b.len() as u64;
    let mut out = Vec::new();
    if n < 0xfd {
        out.push(n as u8);
    } else if n <= 0xffff {
        out.push(0xfd);
        out.extend_from_slice(&(n as u16).to_le_bytes());
    } else if n <= 0xffff_ffff {
        out.push(0xfe);
        out.extend_from_slice(&(n as u32).to_le_bytes());
    } else {
        out.push(0xff);
        out.extend_from_slice(&n.to_le_bytes());
    }
    out.extend_from_slice(b);
    out
}

fn tapleaf_hash(script: &[u8], tapleaf_ver: u8) -> [u8; 32] {
    let mut data = vec![tapleaf_ver & 0xfe];
    data.extend_from_slice(&serialize_varbytes(script));
    tagged_hash("TapLeaf", &data)
}

fn tapbranch_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let (a, b) = if left <= right {
        (left.as_slice(), right.as_slice())
    } else {
        (right.as_slice(), left.as_slice())
    };
    let mut data = Vec::with_capacity(64);
    data.extend_from_slice(a);
    data.extend_from_slice(b);
    tagged_hash("TapBranch", &data)
}

fn leaf_version(leaf: &Value) -> u8 {
    leaf.get("leafVersion")
        .and_then(|v| v.as_u64())
        .unwrap_or(0xc0) as u8
}

fn leaf_script_bytes(leaf: &Value) -> Vec<u8> {
    let script = leaf
        .get("script")
        .and_then(|v| v.as_str())
        .expect("leaf script");
    h2b(script)
}

fn compute_merkle_root(tree: &Value) -> [u8; 32] {
    if tree.is_object() {
        return tapleaf_hash(&leaf_script_bytes(tree), leaf_version(tree));
    }
    let arr = tree.as_array().expect("branch is array");
    assert_eq!(arr.len(), 2, "branch must have two children");
    let left = compute_merkle_root(&arr[0]);
    let right = compute_merkle_root(&arr[1]);
    tapbranch_hash(&left, &right)
}

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

/// Mirror python walk_script_tree_paths.
fn walk_script_tree_paths(tree: &Value, path: u32, depth: u32) -> Vec<u32> {
    if tree.is_object() {
        return vec![path];
    }
    let arr = tree.as_array().expect("branch");
    assert_eq!(arr.len(), 2);
    let mut left = walk_script_tree_paths(&arr[0], path, depth + 1);
    let right = walk_script_tree_paths(&arr[1], path | (1u32 << depth), depth + 1);
    left.extend(right);
    left
}

/// Mirror python compute_control_block.
fn compute_control_block(path: u32, tree: &Value) -> Vec<u8> {
    // python:
    //   while branch: sibling = tree[(path&1)^1]; tree = tree[path&1];
    //                  control_block = merkle(sibling) + control_block; path >>= 1
    //   return bytes([leafVersion|1]) + control_block
    fn walk(path: u32, node: &Value) -> (u8, Vec<u8>) {
        if node.is_object() {
            return (leaf_version(node) | 1, Vec::new());
        }
        let arr = node.as_array().expect("branch");
        assert_eq!(arr.len(), 2);
        let bit = (path & 1) as usize;
        let sibling = &arr[bit ^ 1];
        let child = &arr[bit];
        let sib_root = compute_merkle_root(sibling);
        let (ver, deeper) = walk(path >> 1, child);
        // python: control_block = merkle(sibling) + control_block while walking root→leaf,
        // so leaf-adjacent siblings end up first and the root sibling last.
        // Recursing child-first: deeper || current_sibling.
        let mut out = deeper;
        out.extend_from_slice(&sib_root);
        (ver, out)
    }
    let (ver, siblings) = walk(path, tree);
    let mut out = vec![ver];
    out.extend(siblings);
    out
}

fn collect_control_blocks(tree: &Value) -> Vec<Vec<u8>> {
    walk_script_tree_paths(tree, 0, 0)
        .into_iter()
        .map(|p| compute_control_block(p, tree))
        .collect()
}

const CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

fn bech32_polymod(values: &[u8]) -> u32 {
    const GEN: [u32; 5] = [
        0x3b6a_57b2,
        0x2650_8e6d,
        0x1ea1_19fa,
        0x3d42_33dd,
        0x2a14_62b3,
    ];
    let mut chk: u32 = 1;
    for v in values {
        let b = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ u32::from(*v);
        for (i, g) in GEN.iter().enumerate() {
            if ((b >> i) & 1) != 0 {
                chk ^= g;
            }
        }
    }
    chk
}

fn bech32_hrp_expand(hrp: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for b in hrp.bytes() {
        out.push(b >> 5);
    }
    out.push(0);
    for b in hrp.bytes() {
        out.push(b & 31);
    }
    out
}

fn convertbits(data: &[u8], frombits: u32, tobits: u32, pad: bool) -> Option<Vec<u8>> {
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut ret = Vec::new();
    let maxv = (1u32 << tobits) - 1;
    for &value in data {
        if (u32::from(value) >> frombits) != 0 {
            return None;
        }
        acc = (acc << frombits) | u32::from(value);
        bits += frombits;
        while bits >= tobits {
            bits -= tobits;
            ret.push(((acc >> bits) & maxv) as u8);
        }
    }
    if pad {
        if bits > 0 {
            ret.push(((acc << (tobits - bits)) & maxv) as u8);
        }
    } else if bits >= frombits || ((acc << (tobits - bits)) & maxv) != 0 {
        return None;
    }
    Some(ret)
}

fn encode_bech32m(hrp: &str, witver: u8, witprog: &[u8]) -> String {
    let mut data = vec![witver];
    data.extend(convertbits(witprog, 8, 5, true).expect("convertbits"));
    let mut values = bech32_hrp_expand(hrp);
    values.extend_from_slice(&data);
    values.extend_from_slice(&[0; 6]);
    let polymod = bech32_polymod(&values) ^ 0x2bc8_30a3;
    let mut combined = data;
    for i in 0..6 {
        combined.push(((polymod >> (5 * (5 - i))) & 31) as u8);
    }
    let mut s = format!("{hrp}1");
    for v in combined {
        s.push(CHARSET[v as usize] as char);
    }
    s
}

fn run_vector(v: &Value) -> Result<(), String> {
    let given = v.get("given").cloned().unwrap_or(Value::Null);
    let intermediary = v.get("intermediary").cloned().unwrap_or(Value::Null);
    let expected = v.get("expected").cloned().unwrap_or(Value::Null);
    let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("?");

    let has_internal = given.get("internalPubkey").is_some();
    let script_tree = given.get("scriptTree");

    if has_internal {
        if expected.get("error").is_none() {
            return Err(format!("{id}: expected error for internalPubkey"));
        }
        return Ok(());
    }

    if script_tree.map(|t| t.is_null()).unwrap_or(true) {
        if expected.get("error").is_none() {
            return Err(format!("{id}: expected error for null tree"));
        }
        return Ok(());
    }

    let tree = script_tree.unwrap();
    let leaf_hashes: Vec<String> = collect_leaf_hashes(tree)
        .into_iter()
        .map(hex::encode)
        .collect();
    let expected_leaves = intermediary
        .get("leafHashes")
        .and_then(|x| x.as_array())
        .ok_or_else(|| format!("{id}: missing leafHashes"))?;
    let expected_leaves: Vec<&str> = expected_leaves
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect();
    if leaf_hashes != expected_leaves {
        return Err(format!(
            "{id}: leaf hash mismatch\n  derived: {leaf_hashes:?}\n  expected: {expected_leaves:?}"
        ));
    }

    let merkle = compute_merkle_root(tree);
    let merkle_hex = hex::encode(merkle);
    let exp_merkle = intermediary
        .get("merkleRoot")
        .and_then(|x| x.as_str())
        .ok_or_else(|| format!("{id}: missing merkleRoot"))?;
    if merkle_hex != exp_merkle {
        return Err(format!(
            "{id}: merkle root mismatch derived={merkle_hex} expected={exp_merkle}"
        ));
    }

    let script_pubkey = format!("5220{merkle_hex}");
    let exp_spk = expected
        .get("scriptPubKey")
        .and_then(|x| x.as_str())
        .ok_or_else(|| format!("{id}: missing scriptPubKey"))?;
    if script_pubkey != exp_spk {
        return Err(format!(
            "{id}: scriptPubKey mismatch derived={script_pubkey} expected={exp_spk}"
        ));
    }

    if let Some(addr) = expected.get("bip350Address").and_then(|x| x.as_str()) {
        let derived = encode_bech32m("bc", 2, &h2b(&merkle_hex));
        if derived != addr {
            return Err(format!(
                "{id}: address mismatch derived={derived} expected={addr}"
            ));
        }
    }

    if let Some(cbs) = expected
        .get("scriptPathControlBlocks")
        .and_then(|x| x.as_array())
    {
        let derived: Vec<String> = collect_control_blocks(tree)
            .into_iter()
            .map(hex::encode)
            .collect();
        let exp: Vec<&str> = cbs.iter().map(|x| x.as_str().unwrap()).collect();
        if derived != exp {
            return Err(format!(
                "{id}: control blocks mismatch\n  derived: {derived:?}\n  expected: {exp:?}"
            ));
        }
    }

    Ok(())
}

fn load_and_run(name: &str) {
    let path = fixtures_dir().join(name);
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let data: Value = serde_json::from_str(&raw).expect("json");
    assert_eq!(data["version"], 1);
    let vectors = data["test_vectors"].as_array().expect("test_vectors");
    let mut failures = Vec::new();
    for v in vectors {
        if let Err(e) = run_vector(v) {
            failures.push(e);
        }
    }
    assert!(
        failures.is_empty(),
        "P2MR construction failures in {name}:\n{}",
        failures.join("\n")
    );
}

#[test]
fn p2mr_construction_vectors() {
    load_and_run("p2mr_construction.json");
}

#[test]
fn p2mr_pqc_construction_vectors() {
    // Same full expected-value checks as classic (leaf hashes, merkle root,
    // scriptPubKey, bech32m address, control blocks including leafVersion).
    load_and_run("p2mr_pqc_construction.json");
}
