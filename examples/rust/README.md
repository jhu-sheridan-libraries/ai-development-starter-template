# Rust reference example

A minimal, working Rust crate showing the expected shape for a Rust project
built from `libraries-ai-starter`.

## Layout

```
Cargo.toml          # crate manifest, pinned deps
src/lib.rs           # library code
tests/integration.rs # integration tests
```

## Local development

```bash
cd examples/rust
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Promoting this example

See [TEMPLATE_SETUP.md](../../TEMPLATE_SETUP.md) at the repository root.
Roughly:

```bash
cp -r examples/rust/* src/
rm -rf examples/python examples/typescript examples/rust
```
