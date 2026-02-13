# Contributing to Rho Circles

Thank you for your interest in contributing to the Rho Circles Chip Registry!

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic understanding of deterministic computation

### Setup

```bash
git clone https://github.com/danvoulez/rho-circles-.git
cd rho-circles-
cargo build
cargo test
```

## Development Workflow

### 1. Choose a Task

Check the [current status](README.md#status) for incomplete components:
- Base transistors (rho.validate, rho.policy.eval, rho.compile, rho.exec)
- Modules (mod.*)
- Products

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Implement

Follow these principles:

#### Determinism First
- Same input → same output (always)
- No randomness (except cryptographic RNGs for key generation)
- No system time
- No network I/O (except designated gateways)

#### Content Addressability
- All content identified by blake3 CID
- Store in CAS, retrieve by CID

#### Test-Driven Development
```rust
#[test]
fn test_deterministic() {
    let input = json!({"test": "value"});
    let output1 = my_function(input.clone()).unwrap();
    let output2 = my_function(input.clone()).unwrap();
    assert_eq!(output1.cid, output2.cid);
}
```

### 4. Test

Run all tests:
```bash
cargo test
```

Run specific tests:
```bash
cargo test test_normalize
```

Run with output:
```bash
cargo test -- --nocapture
```

### 5. Format and Lint

```bash
cargo fmt
cargo clippy -- -D warnings
```

### 6. Commit

Use conventional commits:
```bash
git commit -m "feat(normalize): add support for nested arrays"
git commit -m "fix(validate): handle edge case in schema resolution"
git commit -m "test(compile): add TLV encoding tests"
```

### 7. Submit PR

Push and create a pull request:
```bash
git push origin feature/your-feature-name
```

## Coding Standards

### Naming Conventions

- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Chips: `lowercase.with.dots`
- Modules: `mod.lowercase`

### Error Handling

Use the `Result<T>` type with `RhoError`:
```rust
pub fn my_function() -> Result<Output> {
    let value = some_operation()
        .map_err(|e| RhoError::Normalize(format!("failed: {}", e)))?;
    Ok(Output { value })
}
```

### Documentation

Add doc comments to public APIs:
```rust
/// Normalize a JSON value to canonical form.
///
/// # Arguments
/// * `value` - A JSON value to normalize
///
/// # Returns
/// A `NormalizeOutput` containing the canonical bytes and CID
///
/// # Errors
/// Returns `RhoError::Normalize` if the value cannot be normalized
pub fn normalize(value: Value) -> Result<NormalizeOutput> {
    // ...
}
```

### Testing

Write tests for:
1. Happy path
2. Error cases
3. Edge cases
4. Determinism (run twice, compare CIDs)

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_happy_path() {
        let input = json!({"key": "value"});
        let result = my_function(input).unwrap();
        assert_eq!(result.status, "success");
    }

    #[test]
    fn test_error_case() {
        let input = json!({"invalid": true});
        let result = my_function(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic() {
        let input = json!({"key": "value"});
        let result1 = my_function(input.clone()).unwrap();
        let result2 = my_function(input).unwrap();
        assert_eq!(result1.cid, result2.cid);
    }
}
```

## Implementing a Base Transistor

Example: `rho.validate`

1. Define the spec in `chip_registry.json`
2. Create stub in `src/chips/validate.rs`
3. Add tests
4. Implement algorithm
5. Update README status
6. Submit PR

## Implementing a Module

Modules compose base transistors.

Example: `mod.log`

1. Define wiring in `examples/chip_specs/mod.log.json`
2. Create module in `src/modules/log.rs`
3. Wire operations together
4. Test end-to-end
5. Submit PR

## CI/CD

All PRs must pass:
- ✅ Tests (`cargo test`)
- ✅ Linting (`cargo fmt`, `cargo clippy`)
- ✅ Determinism tests
- ✅ Registry validation

GitHub Actions runs automatically on push.

## Questions?

Open an issue or discussion on GitHub.

## Code of Conduct

Be respectful, collaborative, and constructive.

## License

By contributing, you agree that your contributions will be licensed under the project's license.
