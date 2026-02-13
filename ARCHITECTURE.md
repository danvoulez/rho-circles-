# Architecture: Rho Circles Chip Registry

## Overview

The Rho Circles system is a deterministic computation platform organized into three concentric rings, each building on the layer below it.

## Three Rings Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Outer Ring                        │
│                    PRODUCTS                         │
│  ┌───────────────────────────────────────────────┐  │
│  │              Middle Ring                      │  │
│  │               MODULES                         │  │
│  │  ┌─────────────────────────────────────────┐ │  │
│  │  │           Inner Ring                    │ │  │
│  │  │       BASE TRANSISTORS                  │ │  │
│  │  │                                         │ │  │
│  │  │  • rho.normalize    (opcode 2)         │ │  │
│  │  │  • rho.validate     (opcode 3)         │ │  │
│  │  │  • rho.policy.eval  (opcode 4)         │ │  │
│  │  │  • rho.compile      (opcode 5)         │ │  │
│  │  │  • rho.exec         (opcode 6)         │ │  │
│  │  │                                         │ │  │
│  │  └─────────────────────────────────────────┘ │  │
│  │                                               │  │
│  │  Modules (Capacities):                        │  │
│  │  • mod.log           • mod.permit             │  │
│  │  • mod.chip.*        • mod.judge              │  │
│  │  • mod.ledger        • mod.ghost              │  │
│  │                                               │  │
│  └───────────────────────────────────────────────┘  │
│                                                      │
│  Products:                                           │
│  • product.logline-trust                             │
│                                                      │
└─────────────────────────────────────────────────────┘
```

## Inner Ring: Base Transistors

Base transistors are the fundamental, deterministic operations. Each has:
- A unique opcode (2-255)
- Deterministic behavior (same input → same output, always)
- No external I/O (except designated gateways)
- Content-addressable by blake3 CID

### rho.normalize (opcode 2)

**Purpose:** Canonical JSON normalization

**Algorithm:**
1. Apply Unicode NFC normalization to all strings
2. Reject non-i64 numbers (no floats)
3. Remove null values from objects (not arrays)
4. Sort object keys lexicographically
5. Serialize without whitespace
6. Return base64(bytes) and blake3(bytes)

**Determinism Law:** `normalize(a) == normalize(b)` if `a` and `b` are semantically equivalent

**Example:**
```json
Input:  {"z": 3, "a": 1, "b": null}
Output: {"bytes": "eyJhIjoxLCJ6IjozfQ==", "cid": "..."}
Canonical: {"a":1,"z":3}
```

### rho.validate (opcode 3)

**Purpose:** JSON Schema validation against CAS-stored schemas

**Algorithm:**
1. Fetch schema by CID from CAS
2. Validate value using deterministic JSON Schema validator
3. Return `{valid: bool, errors?: string[]}`

**Determinism Law:** Validator must be pure (no network, no randomness)

### rho.policy.eval (opcode 4)

**Purpose:** Evaluate signature policies

**Grammar:**
```
policy = hybrid-and(list) | hybrid-or(list) | ed25519 | mldsa3 | true | false
list   = policy ("," policy)*
```

**Algorithm:**
1. Parse policy expression to AST
2. For each leaf (signature type), verify matching proof exists
3. Evaluate tree with short-circuit logic
4. Return boolean

### rho.compile (opcode 5)

**Purpose:** Transform chip_spec JSON → TLV bytecode

**TLV Format:**
```
[Version:1][Opcode:1][InputCount:1][Inputs:var][OutputCount:1][OutputType:1][Children:var]
```

**Algorithm:**
1. Validate chip_spec against schema
2. Resolve dependencies to CIDs
3. Topologically sort operations
4. Encode to TLV bytecode
5. Hash bytecode → rb_cid
6. Store in CAS

**Determinism Law:** `compile(spec) → rb_cid` must be reproducible

### rho.exec (opcode 6)

**Purpose:** Execute TLV bytecode with inputs

**Algorithm:**
1. Fetch bytecode by rb_cid from CAS
2. Initialize stack machine
3. Execute opcodes sequentially
4. Each op pops inputs, pushes outputs
5. Final stack top → result body
6. Hash body → content_cid

**Determinism Law:** `exec(rb_cid, inputs) → content_cid` must be reproducible

## Middle Ring: Modules (Capacities)

Modules compose base transistors via "wiring" specifications.

### mod.log

Structured logging with validation:
```
normalize → validate(schema) → rc.emit
```

### mod.chip.publish

Publish chip definition to registry:
```
normalize(chip_spec) → validate → cas.put → rc.emit
```

### mod.chip.build

Compile chip to bytecode:
```
cas.get(spec_cid) → compile → cas.put(rb_cid) → rc.emit
```

### mod.chip.eval

Execute chip:
```
cas.get(rb_cid) → normalize(inputs) → exec → rc.emit
```

## Outer Ring: Products

Products are complete applications composed of modules.

### product.logline-trust

LLM trust pipeline:
```
Flow:
1. permit(user, action, resource)
2. judge(prompt_cid, policy_cid) [if permitted]
3. ghost(rc) [privacy]
4. ledger(rc) [audit]
5. log(result)
```

## Core Infrastructure

### CAS (Content Addressable Storage)

All content identified by blake3 hash:
- `put(bytes) → cid`
- `get(cid) → bytes`

Deterministic: same bytes → same CID

### RC (Recibo Cards)

Signed, immutable computation records:
```json
{
  "body": { ... },
  "recibo": {
    "content_cid": "...",
    "signatures": [...]
  }
}
```

## Determinism Laws

1. **Spec→RB**: Compiling same chip_spec produces same rb_cid
2. **RB→RC**: Executing same bytecode with same inputs produces same content_cid
3. **Byte Law**: Changing signature/metadata doesn't change content_cid
4. **No-IO**: Base transistors must not use std::time, std::net, or other non-deterministic I/O

## Security Model

### Hybrid Signatures

Both classical and post-quantum:
- Ed25519 (classical)
- ML-DSA3 (post-quantum)

Policy: `hybrid-and(ed25519, mldsa3)` requires both signatures

### No-IO Enforcement

Compile-time checks prevent:
- Network access
- File I/O (except via CAS)
- System time
- Randomness (except designated RNGs)

## Testing Strategy

### Unit Tests
- Each transistor has isolated tests
- Test vectors from spec

### Integration Tests
- Multi-chip workflows
- End-to-end pipelines

### Determinism Tests
- Run twice, compare CIDs
- Byte-level comparison

### Property Tests
- Fuzzing with proptest
- Invariant checking

## Build Order

To close the inner ring first:

1. ✅ rho.normalize (COMPLETE)
2. ✅ rho.validate (COMPLETE)
3. ✅ rho.policy.eval (COMPLETE)
4. ✅ rho.compile (COMPLETE)
5. ✅ rho.exec (COMPLETE)
6. Then modules
7. Then products

## Implementation Status

See [README.md](README.md) for current status.
