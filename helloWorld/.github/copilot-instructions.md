# Copilot Instructions for helloWorld Rust Project

## Project Overview
- This is a simple Rust project with a single binary crate.
- Main logic is in `src/main.rs`.
- The project demonstrates basic function definition, usage, and testing in Rust.

## Architecture & Structure
- Entry point: `fn main()` in `src/main.rs`.
- Core function: `add(a: i32, b: i32) -> i32` for integer addition.
- Inline documentation uses Rust doc comments (`///` or `/** ... */`).
- Unit tests are defined in a `#[cfg(test)] mod tests` block within `src/main.rs`.

## Developer Workflows
- **Build:** Use `cargo build` to compile the project.
- **Run:** Use `cargo run` to execute the binary and see output.
- **Test:** Use `cargo test` to run unit tests (see `mod tests` in `src/main.rs`).
- **Debug:** Use `cargo check` for fast syntax/type checking.

## Patterns & Conventions
- All logic and tests are currently in a single file (`src/main.rs`).
- Functions are defined using `fn` and tested with Rust's built-in test framework.
- Print output uses `println!` macro.
- No external dependencies; only Rust standard library is used.
- Comments use both block (`/** ... */`) and line (`// ...`) styles.

## Integration Points
- No external crates or APIs are integrated.
- No cross-component communication; all code is local to the binary crate.

## Example: Adding Two Numbers
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Example: Running Tests
```shell
cargo test
```

## Key Files
- `src/main.rs`: Main logic, function definitions, and tests.
- `Cargo.toml`: Project manifest (no dependencies beyond Rust std).

---

For more complex workflows, add new modules in `src/` and update `Cargo.toml` as needed.

---

If any section is unclear or missing, please provide feedback to improve these instructions.
