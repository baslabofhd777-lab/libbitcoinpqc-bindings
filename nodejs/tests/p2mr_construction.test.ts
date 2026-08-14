import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";

const FIXTURES = path.join(
  __dirname,
  "../../tests/vectors/p2mr/fixtures"
);

function sha256(data: Buffer): Buffer {
  return crypto.createHash("sha256").update(data).digest();
}

function taggedHash(tag: string, data: Buffer): Buffer {
  const tagHash = sha256(Buffer.from(tag, "utf8"));
  return sha256(Buffer.concat([tagHash, tagHash, data]));
}

function h2b(h: string): Buffer {
  return Buffer.from(h, "hex");
}

function serializeVarbytes(b: Buffer): Buffer {
  const n = b.length;
  let prefix: Buffer;
  if (n < 0xfd) {
    prefix = Buffer.from([n]);
  } else if (n <= 0xffff) {
    prefix = Buffer.alloc(3);
    prefix[0] = 0xfd;
    prefix.writeUInt16LE(n, 1);
  } else {
    prefix = Buffer.alloc(5);
    prefix[0] = 0xfe;
    prefix.writeUInt32LE(n, 1);
  }
  return Buffer.concat([prefix, b]);
}

function tapleafHash(script: Buffer, tapleafVer: number): Buffer {
  const data = Buffer.concat([
    Buffer.from([tapleafVer & 0xfe]),
    serializeVarbytes(script),
  ]);
  return taggedHash("TapLeaf", data);
}

function tapbranchHash(left: Buffer, right: Buffer): Buffer {
  const [a, b] = Buffer.compare(left, right) <= 0 ? [left, right] : [right, left];
  return taggedHash("TapBranch", Buffer.concat([a, b]));
}

type Tree = any;

function leafVersion(leaf: Tree): number {
  return leaf.leafVersion ?? 0xc0;
}

function computeMerkleRoot(tree: Tree): Buffer {
  if (!Array.isArray(tree)) {
    return tapleafHash(h2b(tree.script), leafVersion(tree));
  }
  const left = computeMerkleRoot(tree[0]);
  const right = computeMerkleRoot(tree[1]);
  return tapbranchHash(left, right);
}

function collectLeafHashes(tree: Tree): Buffer[] {
  if (!Array.isArray(tree)) {
    return [tapleafHash(h2b(tree.script), leafVersion(tree))];
  }
  return [...collectLeafHashes(tree[0]), ...collectLeafHashes(tree[1])];
}

function walkPaths(tree: Tree, pathBits = 0, depth = 0): number[] {
  if (!Array.isArray(tree)) return [pathBits];
  return [
    ...walkPaths(tree[0], pathBits, depth + 1),
    ...walkPaths(tree[1], pathBits | (1 << depth), depth + 1),
  ];
}

function computeControlBlock(pathBits: number, tree: Tree): Buffer {
  if (!Array.isArray(tree)) {
    return Buffer.from([leafVersion(tree) | 1]);
  }
  let control = Buffer.alloc(0);
  let t: Tree = tree;
  let p = pathBits;
  while (Array.isArray(t)) {
    const bit = p & 1;
    const sibling = t[bit ^ 1];
    t = t[bit];
    const sibRoot = computeMerkleRoot(sibling);
    control = Buffer.concat([sibRoot, control]);
    p >>= 1;
  }
  return Buffer.concat([Buffer.from([leafVersion(t) | 1]), control]);
}

function collectControlBlocks(tree: Tree): Buffer[] {
  return walkPaths(tree).map((p) => computeControlBlock(p, tree));
}

// bech32m encode (witver 2)
const CHARSET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
function polymod(values: number[]): number {
  const GEN = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
  let chk = 1;
  for (const v of values) {
    const b = chk >> 25;
    chk = ((chk & 0x1ffffff) << 5) ^ v;
    for (let i = 0; i < 5; i++) {
      if ((b >> i) & 1) chk ^= GEN[i];
    }
  }
  return chk;
}
function hrpExpand(hrp: string): number[] {
  const out: number[] = [];
  for (const c of hrp) out.push(c.charCodeAt(0) >> 5);
  out.push(0);
  for (const c of hrp) out.push(c.charCodeAt(0) & 31);
  return out;
}
function convertbits(data: number[], from: number, to: number, pad: boolean): number[] {
  let acc = 0;
  let bits = 0;
  const ret: number[] = [];
  const maxv = (1 << to) - 1;
  for (const value of data) {
    acc = (acc << from) | value;
    bits += from;
    while (bits >= to) {
      bits -= to;
      ret.push((acc >> bits) & maxv);
    }
  }
  if (pad && bits > 0) ret.push((acc << (to - bits)) & maxv);
  return ret;
}
function encodeBech32m(hrp: string, witver: number, witprog: Buffer): string {
  const data = [witver, ...convertbits([...witprog], 8, 5, true)];
  const values = [...hrpExpand(hrp), ...data, 0, 0, 0, 0, 0, 0];
  const mod = polymod(values) ^ 0x2bc830a3;
  const combined = [...data];
  for (let i = 0; i < 6; i++) combined.push((mod >> (5 * (5 - i))) & 31);
  return hrp + "1" + combined.map((v) => CHARSET[v]).join("");
}

function runVector(v: any): void {
  const given = v.given || {};
  const intermediary = v.intermediary || {};
  const expected = v.expected || {};
  const id = v.id || "?";

  if (given.internalPubkey !== undefined) {
    expect(expected.error).toBeTruthy();
    return;
  }
  if (given.scriptTree === null || given.scriptTree === undefined) {
    expect(expected.error).toBeTruthy();
    return;
  }

  const tree = given.scriptTree;
  const leafHashes = collectLeafHashes(tree).map((h) => h.toString("hex"));
  expect(leafHashes).toEqual(intermediary.leafHashes);

  const merkle = computeMerkleRoot(tree).toString("hex");
  expect(merkle).toBe(intermediary.merkleRoot);
  expect(`5220${merkle}`).toBe(expected.scriptPubKey);

  if (expected.bip350Address) {
    expect(encodeBech32m("bc", 2, h2b(merkle))).toBe(expected.bip350Address);
  }
  if (expected.scriptPathControlBlocks) {
    const cbs = collectControlBlocks(tree).map((c) => c.toString("hex"));
    expect(cbs).toEqual(expected.scriptPathControlBlocks);
  }
}

function runFixture(name: string): void {
  const raw = fs.readFileSync(path.join(FIXTURES, name), "utf8");
  const data = JSON.parse(raw);
  expect(data.version).toBe(1);
  expect(data.test_vectors.length).toBeGreaterThan(0);
  for (const v of data.test_vectors) {
    runVector(v);
  }
}

describe("P2MR construction vectors", () => {
  test("p2mr_construction.json", () => {
    runFixture("p2mr_construction.json");
  });

  // Same full expected-value checks as classic (leaf hashes, merkle root,
  // scriptPubKey, bech32m address, control blocks including leafVersion).
  test("p2mr_pqc_construction.json", () => {
    runFixture("p2mr_pqc_construction.json");
  });
});
