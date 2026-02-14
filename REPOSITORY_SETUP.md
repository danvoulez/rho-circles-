# Repository Setup Recommendations

This document provides recommendations for finalizing the GitHub repository setup to make it more discoverable and professional.

## Repository Description

Add this description in the GitHub repository settings:

```
Deterministic computation platform with content-addressable storage, WASM support, and cryptographic verification using blake3 and canonical JSON
```

## Repository Topics

Add the following topics/tags to the repository (in GitHub repository settings):

- `rust`
- `wasm`
- `webassembly`
- `content-addressable-storage`
- `blake3`
- `canonical-json`
- `ledger`
- `cryptography`
- `deterministic`
- `json-normalization`
- `cid`
- `typescript`
- `react`

## GitHub Issues

The repository currently has no open issues. Consider creating a "Super Issue" to track major milestones and sub-tasks for the project.

## Demo Deployment

Consider deploying the UI to a hosting platform:

- **Vercel**: Quick deployment for React apps
- **Netlify**: Simple static site hosting
- **GitHub Pages**: Free hosting directly from the repository

Once deployed, add the demo link to:
1. Repository description
2. README.md "Try it" section
3. Repository website field in GitHub settings

## Performance Targets

Document these performance targets in the README or a separate PERFORMANCE.md:

- WASM bundle size: ≤ 300KB gzipped
- P95 normalize time: < 20ms for 10-30KB objects
- Memory usage: Minimal allocations for typical operations

## Security Guidelines

Already documented in SECURITY_SUMMARY.md, ensure these are highlighted:

- Reject floats and numbers outside i64 range
- Deterministic error messages
- No network I/O in core functions
- Content-addressable storage prevents tampering
