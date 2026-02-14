# Chip Registry

This repository implements the Rho Circles Chip Registry system, a deterministic computation platform based on content-addressable storage and cryptographic verification.

## Architecture

The system consists of three concentric rings:

### Inner Ring: Base Transistors
- **rho.normalize** (opcode 2): Canonical JSON normalization with blake3 CID
- **rho.validate** (opcode 3): JSON Schema validation (WIP)
- **rho.policy.eval** (opcode 4): Signature policy evaluation (WIP)
- **rho.compile** (opcode 5): Chip spec to TLV bytecode compilation (WIP)
- **rho.exec** (opcode 6): Stack machine interpreter (WIP)

### Middle Ring: Modules (Capacities)
- **mod.log**: Structured logging ✅
- **mod.chip.publish**: Chip definition publishing ✅
- **mod.chip.build**: Chip compilation ✅
- **mod.chip.eval**: Chip execution ✅
- **mod.ledger**: Append-only ledger ✅
- **mod.permit**: Access control ✅
- **mod.judge**: LLM gateway ✅

### Outer Ring: Products
- **product.api-notary**: HTTP wrapper for B2B API receipt generation ✅
- **product.content-sign**: CLI tool for content signing and verification ✅
- **product.ai-passport**: AI model registry with compliance certification ✅

## Core Principles

1. **Determinism**: spec→rb and rb→rc must be reproducible
2. **Content Addressability**: All content identified by blake3 CID
3. **No IO**: Pure computation (except designated gateways)
4. **Hybrid Signatures**: Ed25519 + ML-DSA3 post-quantum crypto

## Building

```bash
cargo build
```

### Building WASM

To build the WebAssembly module for browser use:

```bash
# Install wasm32 target (one-time)
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli (one-time)
cargo install wasm-bindgen-cli --version 0.2.108

# Build WASM
./build-wasm.sh
```

This compiles `crates/rho-core` to WebAssembly and generates JavaScript bindings in `packages/rho-wasm/`.

## Testing

All code is tested with comprehensive unit and integration tests.

```bash
cargo test
```

**Test Coverage**: 85 tests
- 70 unit tests (Inner Ring + Middle Ring + Products + Infrastructure)
- 15 integration tests (Normalization spec vectors)

All tests verify determinism and follow THE CANON.

## Running

```bash
cargo run
```

## Products

The Outer Ring contains complete applications built on top of the Middle Ring modules:

### product.api-notary

**Purpose**: Sidecar for B2B APIs that generates cryptographic receipts for every data exchange.

**Use Case**: Eliminates "I didn't receive that" disputes in API integrations by providing immutable proof of what was sent and received.

**How it works**:
1. Intercepts HTTP request/response pairs
2. Normalizes the transaction data using `rho.normalize`
3. Emits a signed Recibo Card (RC) that both parties can verify
4. Supports hybrid signatures (Ed25519 + ML-DSA3)

**Example**:
```rust
use rho_circles::products::{notarize, ApiTransaction};

let transaction = ApiTransaction {
    method: "POST".to_string(),
    path: "/api/v1/payment".to_string(),
    timestamp: "2024-01-01T12:00:00Z".to_string(),
    request_body: Some(json!({"amount": 100})),
    response_body: Some(json!({"status": "success"})),
    status_code: 200,
};

let receipt = notarize(transaction, signatures)?;
```

### product.content-sign

**Purpose**: CLI tool for content creators to sign files/JSONs and generate verifiable receipts.

**Use Case**: Newsrooms, publishers, and content creators can prove authenticity of their content, creating a "blue checkmark" system to combat fake news.

**How it works**:
1. Takes any content (text, JSON, binary files)
2. Computes blake3 CID of the content
3. Creates a signed metadata record with author, timestamp, and content CID
4. Emits a Recibo Card that anyone can verify offline

**Example**:
```rust
use rho_circles::products::sign_json;

let article = json!({
    "headline": "Breaking News",
    "content": "..."
});

let receipt = sign_json(
    article,
    "Verified News Agency".to_string(),
    "2024-01-01T12:00:00Z".to_string(),
    signatures,
)?;
```

### product.ai-passport

**Purpose**: Simple registry for AI models with compliance certification and bias metrics.

**Use Case**: Satisfies EU AI Act requirements by providing immutable proof of model characteristics, training data, and compliance status.

**How it works**:
1. Developers upload model weights hash and compliance documentation
2. System records bias metrics (demographic parity, fairness scores, toxicity)
3. Generates a unified Recibo Card proving compliance
4. Validates metrics against regulatory thresholds

**Bias Metrics** (represented as integers 0-10000 for determinism):
- `demographic_parity`: Should be ≤ 2000 (0.20 or 20%)
- `equal_opportunity`: Should be ≥ 8000 (0.80 or 80%)
- `fairness_score`: Should be ≥ 7000 (0.70 or 70%)
- `toxicity_score`: Should be ≤ 3000 (0.30 or 30%)

**Example**:
```rust
use rho_circles::products::ai_passport::{register_with_hash, BiasMetrics};

let bias_metrics = BiasMetrics {
    demographic_parity: 1200,  // 0.12
    equal_opportunity: 8800,   // 0.88
    fairness_score: 8500,      // 0.85
    toxicity_score: Some(1000), // 0.10
};

let passport = register_with_hash(
    model_info,
    model_weights_cid,
    compliance_doc,
    bias_metrics,
    timestamp,
    signatures,
)?;
```

## UI Template System

**New!** A complete, modular UI template system is now available in `ui/`:

- **Design Philosophy:** Glass & Ledger - translucent panels over immutable data
- **Tech Stack:** React 18 + TypeScript + Vite + Tailwind CSS + Framer Motion
- **Components:** 7 reusable atomic components (CidBadge, StatusIndicator, JsonViewer, etc.)
- **WASM Integration:** Real cryptographic operations run in the browser via WebAssembly
- **Zero Custody:** Client-side hashing and verification
- **Production Ready:** Can be applied to any Rho product

See `ui/README.md` and `PRODUCT_UI_EXAMPLES.md` for complete documentation.

**Quick Start:**
```bash
# Build WASM first
./build-wasm.sh

# Start UI dev server
cd ui && npm install && npm run dev

# Build for production
cd ui && npm run build
```

The UI uses the real `rho-core` WASM module for:
- JSON normalization (canonical form)
- Blake3 CID generation (base64url, no padding)
- Content validation

**Performance:** 
- WASM module: ~305 KB (~90 KB gzipped)
- Normalization: <20ms for 10-30KB objects
- All operations deterministic and reproducible

## Status

- [x] Project structure
- [x] rho.normalize implementation
- [x] CAS (Content Addressable Storage)
- [x] rho.validate
- [x] rho.policy.eval
- [x] rho.compile
- [x] rho.exec
- [x] Modules (mod.log, mod.chip.*, mod.ledger, mod.permit, mod.judge)
- [x] Products (api-notary, content-sign, ai-passport)
- [x] **UI Template System** (Glass & Ledger design)
- [x] CI/CD pipeline

## Determinism Laws

1. **Spec→RB**: compile(chip_spec) twice → same rb_cid
2. **RB→RC**: exec(rb_cid, inputs) twice → same content_cid
3. **Byte Law**: change recibo → same content_cid
4. **No-IO**: panic if std::time or std::net used (except gateways)
