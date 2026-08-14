# just check vs CI (libbitcoinpqc-bindings)

Date: 2026-08-01
Scope: root `justfile`, `.github/workflows/ci.yml` (only workflow), root `Makefile` glance, note on `flake.nix` hermetic gate.

## Direct answer

**`just check` is an alias for `just ci`.** On this host it is intended to mirror every **blocking** GitHub Actions job, and it largely does for **Linux-shaped** work.

It does **not** cover **all possible** checks:

- No **macOS** matrix legs (`build-matrix`, `python-test`, `nodejs-test`).
- Does not run the **main-branch benchmark** job (optional: `just ci-benchmark`; CI marks it `continue-on-error`).
- **`vectors-in-sync` is stricter in CI** (full-tree clean) than locally (vector paths only).
- Broader/optional local gates (`just hermetic` / `nix flake check`, `fuzz-smoke`) are outside GHA.

So: good pre-push gate for Ubuntu-equivalent blocking CI; not full parity with every CI leg or every optional local check.

---

## 1. Root justfile recipes

| Recipe | What it runs | In `check`/`ci`? |
|--------|----------------|------------------|
| `default` | `just --list` | n/a |
| **`check`** | alias → **`ci`** | **yes (entry)** |
| **`ci`** | `submodule-check` + `vectors-in-sync` + `no-skipped-tests` + `ci-rust` + `c-lib` + `python` + `nodejs` + `wasm` + `emscripten` | **yes** |
| `test` | `submodule-check` + `rust` + `c-lib` + `python` + `nodejs` + `wasm` (no git gates, no scrub, no emscripten) | no |
| `test-all` | `test` + `emscripten` | no |
| `test-rust` | `submodule-check` + `rust` | no |
| `ci-benchmark` | scrub + `CI=true cargo bench --features bench -- --noplot` | no (optional) |
| `hermetic` | `nix flake check -L` | no (Nix gate) |
| `shell` | `nix develop` | no |
| `submodule-check` | require `libbitcoinpqc/CMakeLists.txt` | via `ci`/`test` |
| `vectors-in-sync` | force `LIBBITCOINPQC_SRC=$(pwd)/libbitcoinpqc`; `python3 scripts/sync-golden-vectors.py`; scoped `git diff` / untracked on algorithm vector paths + submodule `tests/vectors` | via `ci` |
| `no-skipped-tests` | `rg` skip/ignore markers under `tests/` `python/tests` `nodejs/tests` `wasm/test` | via `ci` |
| `ci-rust` | `rust-cache-scrub` + `rust-lint` + `rust-build` + `rust-test` + `fuzz-check` | via `ci` |
| `rust` | lint/build/test/fuzz-check **without** scrub | via `test` only |
| `rust-cache-scrub` | `rm` stale `target/*/build/bitcoinpqc-*` and related deps | via `ci-rust` |
| `rust-lint` | `cargo fmt -- --check`; `cargo clippy -- -D warnings` | via rust/`ci-rust` |
| `rust-build` | `cargo build --verbose` | via rust/`ci-rust` |
| `rust-test` | `cargo test --verbose -- --test-threads=1` + serde feature same | via rust/`ci-rust` |
| `fuzz-check` | `(cd fuzz && cargo check)` | via rust/`ci-rust` |
| `fuzz-smoke` | nightly `cargo fuzz run` 2s × 5 targets | **no** (manual) |
| `c-lib` | `make c-lib-test` (cmake BUILD_TESTS=ON + ctest) | via `ci`/`test` |
| `python` | `make c-lib` then `cd python && python3 -m unittest discover -s tests -v` | via `ci`/`test` |
| `nodejs` | `cd nodejs && npm ci --ignore-scripts && npm run build && npm test` | via `ci`/`test` |
| `wasm` | `cargo check --target wasm32-unknown-unknown` + `make wasm-test` (wasm-pack test --node) | via `ci`/`test` |
| `emscripten` | requires `emcc` on PATH; `cd wasm && npm ci && npm run build && npm test` (no emsdk pin) | via `ci` only (not plain `test`) |

Header comment in justfile already states the same macOS / benchmark caveats.

---

## 2. GitHub Actions (`.github/workflows/ci.yml` only)

Triggers: `push` / `pull_request` to branches `main`, `27-slh-dsa-sha-2-128s`, `wasm-tests`.

| Job | Runner / matrix | Commands (essence) | Blocking? |
|-----|-----------------|--------------------|-----------|
| `vectors-in-sync` | ubuntu-latest | checkout submodules; **`make sync-vectors`**; **full** `git diff --exit-code` + porcelain empty; same for `libbitcoinpqc` | yes |
| `no-skipped-tests` | ubuntu-latest | install `rg` if needed; same `rg` pattern/paths as just | yes |
| `test` | ubuntu-latest | apt deps; stable rustfmt+clippy; rust-cache; **scrub**; fmt; clippy `-D warnings`; `cargo build --verbose`; tests + serde tests single-thread; `cargo check` in `fuzz/` | yes |
| `build-matrix` | **ubuntu-latest + macos-latest**, rust stable | same Rust suite as `test` (fmt/clippy/build/tests/fuzz check + scrub); OS-specific cmake deps | yes |
| `c-lib-test` | ubuntu-latest | cmake `-DBUILD_TESTS=ON` Release; build; `ctest --test-dir build --output-on-failure` | yes |
| `python-test` | **ubuntu + macos** | build C lib (no BUILD_TESTS in this job); `python3 -m unittest discover -s tests -v` in `python/` | yes |
| `wasm-test` | ubuntu-latest | clang; Node 20; rust wasm32; wasm-pack; scrub; `cargo check --target wasm32-unknown-unknown`; `make wasm-test` | yes |
| `emscripten-wasm-test` | ubuntu-latest | Node 20; **emsdk `latest`** install/activate; `cd wasm && npm ci && npm run build && npm test` | yes |
| `nodejs-test` | **ubuntu + macos** | Node 20; `npm ci --ignore-scripts && npm run build && npm test` in `nodejs/` | yes |
| `benchmark` | ubuntu-latest | only `push` to **main**; needs `test`; **`continue-on-error: true`**; scrub; `CI=true cargo bench --features bench -- --noplot`; upload criterion artifact | soft / optional |

No other workflow files under `.github/workflows/`.

---

## 3. Comparison table

### CI → covered by `just check`?

| CI does | `just check`? | Notes |
|---------|---------------|--------|
| `vectors-in-sync` | **partial** | Same intent; local scopes git to algorithm golden paths; CI fails on **any** dirty tree after sync. Local forces submodule as `LIBBITCOINPQC_SRC`; CI `make sync-vectors` may prefer standalone `~/Projects/surmount/libbitcoinpqc` if present. |
| `no-skipped-tests` | **yes** | Same `rg` pattern and paths. |
| `test` (Rust fmt/clippy/build/tests/serde/fuzz check + scrub) | **yes** | Via `ci-rust`. |
| `build-matrix` Ubuntu | **yes** (redundant with `test`) | Same commands as `test` on Linux. |
| `build-matrix` **macOS** | **no** | Not runnable as second OS on one Linux host. |
| `c-lib-test` | **yes** | Via `c-lib` → `make c-lib-test`. |
| `python-test` Ubuntu | **yes** | Via `python`. |
| `python-test` **macOS** | **no** | |
| `nodejs-test` Ubuntu | **yes** | Via `nodejs`. |
| `nodejs-test` **macOS** | **no** | |
| `wasm-test` | **yes** | Via `wasm`. |
| `emscripten-wasm-test` | **partial** | Same npm build/test; CI pins **emsdk latest**; local uses whatever `emcc` is on PATH (no version pin). |
| `benchmark` (main, soft) | **no** by default | Optional `just ci-benchmark` (no artifact upload). |

### `just check` → also in CI?

| Local `just check` step | In CI? | Notes |
|-------------------------|--------|--------|
| `submodule-check` | implicit | CI uses `submodules: recursive` on checkout. |
| `vectors-in-sync` (scoped) | yes, stricter full-tree form | |
| `no-skipped-tests` | yes | |
| `ci-rust` (scrub + full Rust suite) | yes (`test` + Ubuntu matrix) | |
| `c-lib` / `make c-lib-test` | yes | |
| `python` | yes (Ubuntu leg) | Local also `make c-lib` first (CI python job builds cmake without tests). |
| `nodejs` | yes (Ubuntu leg) | |
| `wasm` | yes | |
| `emscripten` | yes (emsdk latest) | Toolchain source differs. |

### Not in `just check` (local-only or optional)

| Item | Role |
|------|------|
| `just test` / `test-all` / `test-rust` | Faster subsets; skip git gates / scrub / emscripten as above |
| `just fuzz-smoke` | Real libFuzzer smoke; **not in GHA** (CI only `cargo check` in fuzz/) |
| `just hermetic` / `nix flake check` | Composed Nix gate (upstream C checks + more); **not GHA** |
| `just ci-benchmark` | Soft CI job on main only |
| `just shell` | Dev shell |

---

## 4. Gaps both ways

**CI runs, local `just check` skips or weakens**

1. **macOS** for Rust matrix, Python, Node.
2. **Full-tree** dirty check after vector regen (local allows non-vector WIP).
3. **emsdk latest** vs host `emcc` (version skew risk).
4. Soft **benchmark** + criterion artifact (optional recipe only).
5. Duplicate Ubuntu Rust job (`test` vs `build-matrix`) is CI-only redundancy; local runs once.

**Local can run, CI does not (or not the same)**

1. **`fuzz-smoke`** (actual fuzz execution).
2. **`hermetic` / Nix flake checks** (broader hermetic composition).
3. **`just test`** without vectors/skip gates (WIP-friendly).
4. Host-local emscripten without emsdk.

**Intentional design notes (from justfile comments)**

- `just ci` / `check` claim to mirror **blocking** GHA on this host.
- Explicit non-goals: macOS matrix, main-branch soft benchmark.
- Local vectors gate scoped so unrelated WIP does not fail.

---

## 5. Makefile glance

Root `Makefile` is a **build/dev** surface, **not** a full CI aggregator:

- No `check` or `ci` target.
- `tests` / `test-rust` → only `cargo test` (no fmt/clippy, no bindings suites, no emscripten).
- Helpers used **by** just/CI: `c-lib-test`, `c-lib`, `wasm-test`, `sync-vectors`, `bench`.
- `lint` / `format` / `dev` exist separately; not wired into `just check`.
- Submodule `libbitcoinpqc/Makefile` is upstream C build, not the bindings CI entrypoint.

---

## Bottom line

| Question | Answer |
|----------|--------|
| Does `just check` run all checks CI would run? | **Almost all blocking Linux-equivalent checks: yes.** **All CI jobs including OS matrix and soft benchmark: no.** |
| Are all possible checks covered? | **No.** Missing macOS, soft benchmark by default, CI-strict full-tree vectors, emsdk pin; also local-only hermetic/fuzz-smoke are outside both full parity stories. |
| Closest default check recipe | **`just check` ≡ `just ci`**. Use `just test` for suites without git gates; `just hermetic` for Nix. |
