# Rho Core WASM

WebAssembly bindings for Rho Circles core functionality.

## Usage

```typescript
import { initialize, normalize, validate, cid } from '@rho-circles/wasm';

// Initialize WASM (call once at app startup)
await initialize();

// Normalize JSON data
const result = await normalize({ b: 2, a: 1 });
console.log(result.normalized); // '{"a":1,"b":2}'
console.log(result.cid); // base64url-encoded blake3 hash (no padding)

// Compute CID from bytes
const bytes = new Uint8Array([1, 2, 3]);
const hash = await cid(bytes);
console.log(hash); // base64url CID

// Validate (simplified for now)
const validation = await validate('schema-cid', { foo: 'bar' });
console.log(validation.ok); // true/false
```

## Features

- **Normalize**: Canonical JSON normalization following THE CANON
  - NFC Unicode normalization
  - Lexicographic key sorting
  - Null value removal from objects
  - Integer-only numbers (no floats)
  - Deterministic output

- **CID**: Content-addressable identifier using blake3
  - Base64url encoding (URL-safe, no padding)
  - 256-bit hash output
  - Deterministic and reproducible

- **Validate**: Schema validation (basic implementation)
  - Normalizes input before validation
  - Returns validation errors with paths

## Building

To rebuild the WASM module:

```bash
# From repository root
./build-wasm.sh
```

This will:
1. Compile `crates/rho-core` to `wasm32-unknown-unknown`
2. Generate JavaScript bindings with `wasm-bindgen`
3. Output files to `packages/rho-wasm/`

## Files

- `index.ts` - TypeScript wrapper API
- `rho_core.js` - Generated JavaScript bindings
- `rho_core.d.ts` - TypeScript definitions
- `rho_core_bg.wasm` - WebAssembly binary (~305 KB)

## Size

The WASM module is optimized for size:
- Release build with `opt-level = "z"`
- LTO enabled
- ~305 KB uncompressed
- ~90 KB gzipped (when served)

## Browser Compatibility

Works in all modern browsers with WebAssembly support:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Opera 44+

## License

Same as parent repository.
