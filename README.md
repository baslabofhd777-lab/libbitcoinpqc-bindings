# libbitcoinpqc-bindings

Language bindings (Rust, Python, Node.js, WASM) for the [libbitcoinpqc](https://github.com/cryptoquick/libbitcoinpqc) C library. [BIP 360](https://github.com/bitcoin/bips/blob/master/bip-0360.mediawiki) defines P2MR tapscript signature overloads; this library implements all three:

1. **secp256k1 Schnorr** (BIP-340): Classical elliptic-curve signatures with x-only public keys.
2. **ML-DSA-44** (formerly CRYSTALS-Dilithium): A lattice-based scheme from the NIST PQC standardization.
3. **SLH-DSA-SHA2-128s** (formerly SPHINCS+): A stateless hash-based scheme using SHA-256, aligned with Bitcoin's native hash primitive.

The two post-quantum algorithms (ML-DSA-44 and SLH-DSA-SHA2-128s) are FIPS-certified, which should help with future native hardware support.

## P2MR (BIP 360)

[P2MR](https://github.com/bitcoin/bips/blob/master/bip-0360.mediawiki) (Pay-to-Merkle-Root) is the output and tapscript framework in BIP 360. This library supplies the signature primitives for its script-path spends — classical secp256k1 Schnorr plus optional post-quantum schemes. That is separate from broader Bitcoin post-quantum migration work; BIP 360 is specifically about P2MR tapscript overloads.

## Features

- Clean, unified C API for all three signature algorithms
- Safe Rust bindings with memory safety and zero-copy operations
- NodeJS TypeScript bindings with full type safety
- Python bindings for easy integration
- WASM builds for browser and Node.js (`bitcoinpqc/wasm`)
- User-provided entropy (bring your own randomness)
- Key generation, signing, and verification functions
- Minimal dependencies

## Key Characteristics

| Algorithm | Public Key Size | Secret Key Size | Signature Size | Security Level |
|-----------|----------------|----------------|----------------|----------------|
| SECP256K1_SCHNORR | 32 bytes | 32 bytes | 64 bytes | Classical |
| ML-DSA-44 | 1,312 bytes | 2,560 bytes | 2,420 bytes | NIST Level 2 |
| SLH-DSA-SHA2-128s | 32 bytes | 64 bytes | 7,856 bytes | NIST Level 1 |

See [REPORT.md](benches/REPORT.md) for performance and size comparison to secp256k1.

## Breaking Changes (Phase 2)

Phase 2 renames SLH-DSA bindings from SHAKE-128s to SHA2-128s. Update identifiers as follows:

| Old identifier | New identifier |
|----------------|----------------|
| `SLH_DSA_128S` (Rust) | `SLH_DSA_SHA2_128S` |
| `SLH_DSA_SHAKE_128S` (Python/Node.js/WASM) | `SLH_DSA_SHA2_128S` |

- Enum wire value `2` is unchanged.
- Key sizes (32/64/7856 bytes) are unchanged.
- **Enum wire values:** `SECP256K1_SCHNORR=0`, `ML_DSA_44=1`, `SLH_DSA_SHA2_128S=2`. Removed `FN_DSA_512`.
- **Re-keying required:** keys and signatures from SHAKE-128s are cryptographically incompatible with SHA2-128s. Generate new key pairs after upgrading.

## Security Notes

- This library does not provide its own random number generation. It is essential that the user provide entropy from a cryptographically secure source. See [docs/user_provided_entropy.md](docs/user_provided_entropy.md).
- Random data is required for key generation, but not for signing. All signatures are deterministic, based on the message and secret key.
- The implementations are based on reference code from the NIST PQC standardization process and are not production-hardened.
- Care should be taken to securely manage secret keys in applications.

## BIP 360 / P2MR compliance

This library implements the three tapscript `OP_CHECKSIG` overloads specified in [BIP 360 (P2MR)](https://github.com/bitcoin/bips/blob/master/bip-0360.mediawiki), with the parameter sets named in the BIP.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Dependencies

The C library is included as a git submodule from [libbitcoinpqc](https://github.com/cryptoquick/libbitcoinpqc) at `libbitcoinpqc/`, tracking branch `27-slh-dsa-sha-2-128s` (see `.gitmodules`). To bump the pinned commit after upstream changes on that branch (or once merged to `main`):

```bash
cd libbitcoinpqc && git fetch && git checkout <new-ref> && cd ..
git add libbitcoinpqc && git commit -m "Bump libbitcoinpqc submodule"
```

Build outputs belong in the **parent** `build/` directory (`make c-lib`, `make c-lib-test`). Do not run `cmake -B build` or bare `ctest` inside `libbitcoinpqc/` — that leaves `libbitcoinpqc/build/` and `libbitcoinpqc/Testing/` behind and makes `git submodule status` report untracked content. Run `make clean` (or `make clean-submodule`) to remove those artifacts.

## Building

### Prerequisites

- CMake 3.10 or higher
- C99 compiler
- Rust 1.50 or higher

### Building

```bash
# Clone the repository (with submodules)
git clone --recurse-submodules https://github.com/cryptoquick/libbitcoinpqc-bindings.git
cd libbitcoinpqc-bindings

# Or, if already cloned without submodules:
# git submodule update --init --recursive

# Build the Rust bindings (automatically builds the C library from the submodule)
cargo build --release

# Or use the Makefile
make rust-lib
```

## Fuzz Testing

This library includes fuzz testing targets using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

### Prerequisites

```bash
# Install cargo-fuzz
cargo install cargo-fuzz
```

### Available Fuzz Targets

1. **keypair_generation** - Tests key pair generation with different algorithms
2. **sign_verify** - Tests signature creation and verification
3. **cross_algorithm** - Tests verification with mismatched keys and signatures from different algorithms

### Running Fuzz Tests

```bash
# Run a specific fuzz target
cargo fuzz run keypair_generation
cargo fuzz run sign_verify
cargo fuzz run cross_algorithm

# Run a fuzz target for a specific amount of time (in seconds)
cargo fuzz run keypair_generation -- -max_total_time=60

# Run a fuzz target with a specific number of iterations
cargo fuzz run sign_verify -- -runs=1000000
```

See `fuzz/README.md` for more details on fuzz testing.

## C API Usage

```c
#include <libbitcoinpqc/bitcoinpqc.h>

// Generate random data (from a secure source in production)
uint8_t random_data[256];
// Fill random_data with entropy...

// Generate a key pair
bitcoin_pqc_keypair_t keypair;
bitcoin_pqc_keygen(BITCOIN_PQC_ML_DSA_44, &keypair, random_data, sizeof(random_data));

// Sign a message
const uint8_t message[] = "Message to sign";
bitcoin_pqc_signature_t signature;
bitcoin_pqc_sign(BITCOIN_PQC_ML_DSA_44, keypair.secret_key, keypair.secret_key_size,
                message, sizeof(message) - 1, &signature);

// Verify the signature
bitcoin_pqc_error_t result = bitcoin_pqc_verify(BITCOIN_PQC_ML_DSA_44,
                                             keypair.public_key, keypair.public_key_size,
                                             message, sizeof(message) - 1,
                                             signature.signature, signature.signature_size);

// Clean up resources
bitcoin_pqc_signature_free(&signature);
bitcoin_pqc_keypair_free(&keypair);
```

## Rust API Usage

Rust docs can be found on [docs.rs](https://docs.rs/bitcoinpqc/latest/bitcoinpqc/).

```rust
use bitcoinpqc::{Algorithm, generate_keypair, sign, verify};
use rand::{RngCore, rngs::OsRng};

// Generate random data for key generation
let mut random_data = vec![0u8; 128];
OsRng.fill_bytes(&mut random_data);

// Generate a key pair
let keypair = generate_keypair(Algorithm::ML_DSA_44, &random_data).unwrap();

// Create a message to sign
let message = b"Message to sign";

// Sign the message deterministically
let signature = sign(&keypair.secret_key, message).unwrap();

// Verify the signature
verify(&keypair.public_key, message, &signature).unwrap();
```

## Python API Usage

[Python bindings](https://pypi.org/project/bitcoinpqc/0.4.1/) for all three algorithms.

### Installation

```bash
# Install the Python package
cd python
pip install -e .
```

### Prerequisites

- Python 3.7 or higher
- The libbitcoinpqc C library must be built and installed

### Example Usage

```python
import secrets
from bitcoinpqc import Algorithm, keygen, sign, verify

# Generate random data for key generation
random_data = secrets.token_bytes(128)

# Generate a key pair
algorithm = Algorithm.ML_DSA_44  # CRYSTALS-Dilithium
keypair = keygen(algorithm, random_data)

# Create a message to sign
message = b"Hello, Bitcoin PQC!"

# Sign the message
signature = sign(algorithm, keypair.secret_key, message)

# Verify the signature
is_valid = verify(algorithm, keypair.public_key, message, signature)
print(f"Signature valid: {is_valid}")  # Should print True

# Verification with incorrect message will fail
bad_message = b"Tampered message!"
is_valid = verify(algorithm, keypair.public_key, bad_message, signature)
print(f"Signature valid: {is_valid}")  # Should print False
```

### Python API Reference

The Python API mirrors the C API closely, with some Pythonic improvements:

- **Algorithm** - Enum class for algorithm selection
  - `SECP256K1_SCHNORR`
  - `ML_DSA_44` (CRYSTALS-Dilithium)
  - `SLH_DSA_SHA2_128S` (SPHINCS+)

- **KeyPair** - Class to hold a public/secret key pair
  - `algorithm` - The algorithm used
  - `public_key` - The public key as bytes
  - `secret_key` - The secret key as bytes

- **Signature** - Class to hold a signature
  - `algorithm` - The algorithm used
  - `signature` - The signature as bytes

- **Functions**
  - `public_key_size(algorithm)` - Get the public key size for an algorithm
  - `secret_key_size(algorithm)` - Get the secret key size for an algorithm
  - `signature_size(algorithm)` - Get the signature size for an algorithm
  - `keygen(algorithm, random_data)` - Generate a key pair (32 bytes for `SECP256K1_SCHNORR`, 128 for PQC)
  - `sign(algorithm, secret_key, message)` - Sign a message
  - `verify(algorithm, public_key, message, signature)` - Verify a signature

## NodeJS TypeScript API Usage

[NodeJS TypeScript bindings](https://www.npmjs.com/package/bitcoinpqc) for all three algorithms in JavaScript/TypeScript projects.

### Installation

```bash
# Install the Node.js package
npm install bitcoinpqc
```

### Prerequisites

- Node.js 16 or higher
- The libbitcoinpqc C library must be built and installed

### Example Usage

```typescript
import { Algorithm, generateKeyPair, sign, verify } from 'bitcoinpqc';
import crypto from 'crypto';

// Generate random data for key generation
const randomData = crypto.randomBytes(128);

// Generate a key pair using ML-DSA-44 (CRYSTALS-Dilithium)
const keypair = generateKeyPair(Algorithm.ML_DSA_44, randomData);

// Create a message to sign
const message = Buffer.from('Message to sign');

// Sign the message deterministically
const signature = sign(keypair.secretKey, message);

// Verify the signature
verify(keypair.publicKey, message, signature);
// If verification fails, it will throw a PqcError

// You can also verify using the raw signature bytes
verify(keypair.publicKey, message, signature.bytes);
```

### NodeJS TypeScript API Reference

The TypeScript API provides a clean, modern interface:

- **Algorithm** - Enum for algorithm selection
  - `SECP256K1_SCHNORR`
  - `ML_DSA_44` (CRYSTALS-Dilithium)
  - `SLH_DSA_SHA2_128S` (SPHINCS+)

- **Classes**
  - `PublicKey` - Public key wrapper
  - `SecretKey` - Secret key wrapper with secure handling
  - `KeyPair` - Container for public/secret key pairs
  - `Signature` - Signature wrapper

- **Functions**
  - `publicKeySize(algorithm)` - Get the public key size for an algorithm
  - `secretKeySize(algorithm)` - Get the secret key size for an algorithm
  - `signatureSize(algorithm)` - Get the signature size for an algorithm
  - `generateKeyPair(algorithm, randomData)` - Generate a key pair (32 bytes of entropy for `SECP256K1_SCHNORR`, 128 bytes for PQC)
  - `sign(secretKey, message)` - Sign a message
  - `verify(publicKey, message, signature)` - Verify a signature

For more details, see the [NodeJS TypeScript bindings README](nodejs/README.md).

## WASM API Usage

Browser and Node.js WASM builds are published as [`bitcoinpqc/wasm`](https://www.npmjs.com/package/bitcoinpqc/wasm). The Emscripten high-level API covers all three algorithms with the same entropy rules as the native bindings.

```bash
cd wasm
npm install
npm run build
npm test
```

See [wasm/README.md](wasm/README.md) for the full API reference and browser testing notes.

## Acknowledgments

- The original NIST PQC competition teams for their reference implementations
- The NIST PQC standardization process for advancing post-quantum cryptography
- The BIP 360 (P2MR) contributors


## Test vectors

This repo carries two kinds of shared test data. They look related (both sit
under `tests/vectors/`) but they answer different questions.

**Signature algorithms** (Schnorr, ML-DSA-44, SLH-DSA-SHA2-128s): known-good
keys, messages, and signatures for sign/verify tests. You edit the JSON under
`tests/vectors/fixtures/`; a script regenerates the Rust, Python, JavaScript,
and C copies so every language sees the same bytes. Details:
[`tests/vectors/README.md`](tests/vectors/README.md).

**P2MR construction** (BIP 360): given a script tree, what merkle root,
`scriptPubKey`, bech32m address, and control blocks should you get? That data
is plain JSON under `tests/vectors/p2mr/fixtures/`. Every language loads those
files from that one directory (no generate step). Details:
[`tests/vectors/p2mr/README.md`](tests/vectors/p2mr/README.md).
