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
- **product.logline-trust**: Complete LLM trust pipeline

## Core Principles

1. **Determinism**: spec→rb and rb→rc must be reproducible
2. **Content Addressability**: All content identified by blake3 CID
3. **No IO**: Pure computation (except designated gateways)
4. **Hybrid Signatures**: Ed25519 + ML-DSA3 post-quantum crypto

## Building

```bash
cargo build
```

## Testing

All code is tested with comprehensive unit and integration tests.

```bash
cargo test
```

**Test Coverage**: 71 tests
- 56 unit tests (Inner Ring + Middle Ring + Infrastructure)
- 15 integration tests (Normalization spec vectors)

All tests verify determinism and follow THE CANON.

## Running

```bash
cargo run
```

## Status

- [x] Project structure
- [x] rho.normalize implementation
- [x] CAS (Content Addressable Storage)
- [x] rho.validate
- [x] rho.policy.eval
- [x] rho.compile
- [x] rho.exec
- [x] Modules (mod.log, mod.chip.*, mod.ledger, mod.permit, mod.judge)
- [ ] Products
- [x] CI/CD pipeline

## Determinism Laws

1. **Spec→RB**: compile(chip_spec) twice → same rb_cid
2. **RB→RC**: exec(rb_cid, inputs) twice → same content_cid
3. **Byte Law**: change recibo → same content_cid
4. **No-IO**: panic if std::time or std::net used (except gateways)
