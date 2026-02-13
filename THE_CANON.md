# THE CANON - Rho Circles Canonical Data Format

## Overview

This document defines the **single source of truth** for how data flows through the Rho Circles system. Every operation MUST follow this canon.

## THE CANON

### 1. Canonical Representation

**All data has exactly ONE canonical form:**

```
JSON Value → normalize() → {bytes: base64(canonical_json), cid: blake3(canonical_json)}
```

Where `canonical_json` is UTF-8 encoded JSON with:
- **No whitespace** between tokens
- **Keys sorted** lexicographically (byte order) at ALL depths
- **Unicode NFC** normalized strings
- **i64 integers only** (no floats, no exponential notation)
- **Null values removed** from objects (preserved in arrays)
- **No duplicate keys** (enforced by sorting)

### 2. Storage Canon

**CAS (Content Addressable Storage) stores raw bytes:**

```rust
// Storing normalized data
let normalized = normalize(value)?;
let canonical_bytes = BASE64.decode(&normalized.bytes)?;
let stored_cid = cas.put(canonical_bytes)?;

// INVARIANT: stored_cid MUST equal normalized.cid
assert_eq!(stored_cid, normalized.cid);
```

**Retrieval:**
```rust
let canonical_bytes = cas.get(&cid)?;
let value: Value = serde_json::from_slice(&canonical_bytes)?;
```

### 3. The Data Flow Canon

#### Input Flow (ALL inputs)
```
User Input (arbitrary JSON)
    ↓
normalize(input)
    ↓
{bytes: base64(canonical_json), cid: blake3(canonical_json)}
    ↓
base64_decode(bytes) → canonical_bytes
    ↓
cas.put(canonical_bytes) → cid
    ↓
INVARIANT: cid == normalized.cid
```

#### Operation Flow
```
Operation(input_cid, params)
    ↓
canonical_input_bytes = cas.get(input_cid)
    ↓
input_value = parse_json(canonical_input_bytes)
    ↓
output_value = operate(input_value, params)
    ↓
normalized_output = normalize(output_value)
    ↓
content_cid = normalized_output.cid
```

### 4. Determinism Laws (Canon Guarantees)

**Law 1: Normalization Determinism**
```rust
normalize(a) == normalize(b)  IFF  a and b are semantically equivalent
```

**Law 2: Key Order Invariance**
```rust
normalize({"b": 2, "a": 1}) == normalize({"a": 1, "b": 2})
// Both produce: {"a":1,"b":2}
```

**Law 3: Null Removal**
```rust
normalize({"a": 1, "b": null}) == normalize({"a": 1})
// Both produce: {"a":1}
```

**Law 4: CID Uniqueness**
```rust
canonical_bytes_1 == canonical_bytes_2  ⟺  cid_1 == cid_2
```

### 5. Implementation Rules

**Rule 1: ALL user inputs MUST be normalized**
```rust
// WRONG
pub fn validate(value: Value, schema_cid: Cid) -> Result<Output> {
    compile(&schema).validate(&value)  // ❌ Using raw value
}

// RIGHT
pub fn validate(value: Value, schema_cid: Cid, cas: &Cas) -> Result<Output> {
    let normalized = normalize(value)?;  // ✅ Normalize first
    let canonical_bytes = BASE64.decode(&normalized.bytes)?;
    let canonical_value: Value = serde_json::from_slice(&canonical_bytes)?;
    // ... now use canonical_value
}
```

**Rule 2: ALL stored data MUST use canonical bytes**
```rust
// WRONG
let cid = cas.put(serde_json::to_vec(&value)?)?;  // ❌ Arbitrary serialization

// RIGHT
let normalized = normalize(value)?;  // ✅ Normalize first
let canonical_bytes = BASE64.decode(&normalized.bytes)?;
let cid = cas.put(canonical_bytes)?;
assert_eq!(cid, normalized.cid);  // ✅ Verify canon
```

**Rule 3: ALL outputs MUST be normalized**
```rust
// WRONG
Ok(ExecOutput {
    body: raw_output,  // ❌ Raw output
    content_cid: "computed_somehow",
})

// RIGHT
let normalized_output = normalize(raw_output)?;  // ✅ Normalize first
Ok(ExecOutput {
    body: serde_json::from_slice(&BASE64.decode(&normalized_output.bytes)?)?,
    content_cid: normalized_output.cid,  // ✅ CID from normalization
})
```

### 6. Test Canon

**ALL tests MUST verify the canon:**

```rust
#[test]
fn test_follows_canon() {
    // Different representations, same canonical form
    let input1 = json!({"z": 3, "a": 1, "removed": null});
    let input2 = json!({"a": 1, "z": 3});
    
    let norm1 = normalize(input1)?;
    let norm2 = normalize(input2)?;
    
    // THE CANON: same CID
    assert_eq!(norm1.cid, norm2.cid);
    
    // THE CANON: same canonical bytes
    assert_eq!(norm1.bytes, norm2.bytes);
    
    // THE CANON: canonical form is {"a":1,"z":3}
    let canonical = String::from_utf8(BASE64.decode(&norm1.bytes)?)?;
    assert_eq!(canonical, r#"{"a":1,"z":3}"#);
}
```

### 7. Anti-Patterns (FORBIDDEN)

**❌ Never bypass normalization:**
```rust
// FORBIDDEN
let cid = blake3::hash(value.to_string().as_bytes());
```

**❌ Never use arbitrary JSON serialization:**
```rust
// FORBIDDEN
serde_json::to_string(&value)  // Not canonical!
serde_json::to_string_pretty(&value)  // Definitely not!
```

**❌ Never compare raw JSON:**
```rust
// FORBIDDEN
if value1 == value2 { ... }  // Key order matters!
```

**✅ Always normalize before comparing:**
```rust
// CORRECT
if normalize(value1)?.cid == normalize(value2)?.cid { ... }
```

### 8. The Canon Checklist

Before merging any code, verify:

- [ ] All user inputs pass through `normalize()`
- [ ] All CAS storage uses canonical bytes
- [ ] All CAS retrieval parses canonical bytes
- [ ] All outputs are normalized
- [ ] All CIDs come from `normalize().cid`
- [ ] All tests verify canonical form
- [ ] No raw JSON serialization (use normalize)
- [ ] No direct JSON comparison (compare CIDs)
- [ ] CID invariant verified: `cas.put(decode(norm.bytes)) == norm.cid`

## Summary

**THE CANON in one sentence:**

> All data flows through `normalize()` to produce canonical bytes, which are stored in CAS by their blake3 CID, and all operations work on canonical bytes retrieved from CAS.

**This is the law. This is the canon. Follow it.**
