# Super Issue Template: Core WASM in UI + Public Demo

Use this template to create a comprehensive tracking issue for completing the WASM integration and public demo.

---

# 🔧 Super Issue: Core WASM in UI + Public Demo

**Objective:** Complete UI integration with real WASM functionality, green CI, and published demo.

## Definition of Done

- [ ] UI uses real WASM module (no mock data)
- [ ] Normalize operation is idempotent
- [ ] CID generation is stable (Unicode NFC normalization)
- [ ] Validate function normalizes and returns JSON paths for errors (e.g., `$.a[2].b`)
- [ ] CID output format: base64url without padding
- [ ] CI pipeline: All jobs passing (fmt, clippy, tests, WASM build, UI build)
- [ ] Automated deployment on merge to main
- [ ] README includes link to live demo
- [ ] Demo video/GIF showing: edit → normalize → copy CID

## Subtasks

### 1. WASM Core Bindings

**Description:** Ensure `rho-core` compiles to wasm32-unknown-unknown with proper bindings

**Tasks:**
- [ ] Verify wasm-bindgen annotations on public functions
- [ ] Ensure all functions return serializable types (no raw pointers)
- [ ] Test WASM module loads in browser environment
- [ ] Optimize binary size (current: ~305KB, target: ≤300KB gzipped)

**Acceptance Criteria:**
- `./build-wasm.sh` completes without errors
- Generated .wasm file is under size budget
- JS bindings are properly typed

---

### 2. TypeScript Package (`rho-wasm`)

**Description:** Complete the TypeScript wrapper package with proper types and loader

**Tasks:**
- [ ] Add TypeScript type definitions for all exported functions
- [ ] Implement async WASM loader with error handling
- [ ] Add JSDoc comments for API documentation
- [ ] Create package.json with proper exports
- [ ] Add usage examples in package README

**Acceptance Criteria:**
- TypeScript types compile without errors
- Package can be imported in both Node.js and browser
- Functions match the API: `init()`, `normalize()`, `validate()`, `cid()`

---

### 3. UI Integration

**Description:** Replace mock data in UI with real WASM calls

**Tasks:**
- [ ] Initialize WASM module on app startup
- [ ] Wire normalize button to WASM normalize function
- [ ] Wire validate button to WASM validate function
- [ ] Display CID in proper format (base64url, no padding)
- [ ] Add loading states during WASM operations
- [ ] Add error toasts for WASM errors with clear messages
- [ ] Implement "copy CID" functionality
- [ ] Add success toast on copy

**Acceptance Criteria:**
- All normalization operations use real WASM
- UI handles errors gracefully
- CIDs are displayed and copyable
- No console errors in production build

---

### 4. Property-based Testing (`proptest`)

**Description:** Add comprehensive property-based tests for edge cases

**Tasks:**
- [ ] Add proptest for Unicode NFC normalization
- [ ] Test key ordering is stable across random inputs
- [ ] Test normalize is idempotent (normalize(normalize(x)) === normalize(x))
- [ ] Test null removal is consistent
- [ ] Test large objects (up to 1MB JSON)
- [ ] Test special Unicode characters (emoji, combining marks, etc.)

**Acceptance Criteria:**
- All proptests pass with 1000+ cases each
- Test coverage includes edge cases from THE_CANON.md
- CI runs proptests on every commit

---

### 5. CI/CD Pipeline

**Description:** Complete CI pipeline with automated deployment

**Tasks:**
- [ ] Ensure all CI jobs pass: test, determinism, build, lint, WASM, UI
- [ ] Add deployment job for UI (Vercel/Netlify/GitHub Pages)
- [ ] Set up preview deployments for PRs
- [ ] Add status checks requiring CI to pass before merge
- [ ] Configure caching for faster builds
- [ ] Add performance benchmarks to CI

**Acceptance Criteria:**
- CI badge in README shows passing status
- Main branch is always green
- PRs get automatic preview deployments
- Deployment completes in < 5 minutes

---

### 6. Documentation & Demo

**Description:** Create comprehensive documentation and public demo

**Tasks:**
- [ ] Update README with live demo link
- [ ] Create demo GIF/video showing key workflow
- [ ] Write "Quick Start" guide (5 minutes to first normalize)
- [ ] Add troubleshooting section for common issues
- [ ] Document WASM API with examples
- [ ] Add performance characteristics to docs
- [ ] Create CONTRIBUTING.md with development workflow

**Acceptance Criteria:**
- README has prominent demo link at top
- Demo video is < 30 seconds and shows core functionality
- New users can get started in under 5 minutes
- All public APIs are documented

---

## Technical Requirements

### CID Format
- **Encoding:** base64url (RFC 4648 §5)
- **Padding:** None (no trailing `=` characters)
- **Hash:** blake3 (256-bit)

### Error Format
- **Structure:** `{ path: string, message: string }`
- **Path format:** JSON path notation (e.g., `$.users[0].name`)
- **Messages:** Deterministic and user-friendly

### Performance Budget
- **WASM size:** ≤ 300KB gzipped
- **Normalize (10KB):** P95 < 10ms
- **Normalize (30KB):** P95 < 20ms
- **Validate:** P95 < 30ms for typical schemas

### Security Constraints
- Reject floating-point numbers with clear error
- Reject integers outside i64 range
- All errors must be deterministic (same input → same error)
- No undefined behavior on malformed input

---

## Testing Strategy

1. **Unit tests:** Core functionality (normalize, validate, CID)
2. **Integration tests:** WASM bindings work in browser
3. **Property tests:** Edge cases and invariants
4. **E2E tests:** Full user workflow in UI
5. **Performance tests:** Meet budget requirements

## Success Metrics

- [ ] CI: 100% green on main branch
- [ ] Coverage: > 85% for core functions
- [ ] Performance: All operations under budget
- [ ] Documentation: No reported confusion from new users
- [ ] Demo: Public URL accessible and working

---

## Related Documents

- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [THE_CANON.md](THE_CANON.md) - Canonical JSON specification
- [ACCEPTANCE_CRITERIA.md](ACCEPTANCE_CRITERIA.md) - Product acceptance criteria
- [SECURITY_SUMMARY.md](SECURITY_SUMMARY.md) - Security considerations
- [REPOSITORY_SETUP.md](REPOSITORY_SETUP.md) - Repository configuration

---

## Notes

- This is a tracking issue; close it only when all subtasks are complete
- Update checkboxes as work progresses
- Link PRs to this issue for traceability
- Celebrate when done! 🎉
