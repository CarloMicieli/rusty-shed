---
name: Rust & Tauri Standards
description: This file describes the Rust & Tauri code style and workflow standards for the project.
applyTo: "src-tauri/src/**/*.rs"
---

# Rust & Tauri Standards

## Scope
Apply these rules to all files within the `src-tauri/` directory.
Follow idiomatic Rust practices and community standards when writing Rust code.

## Project Context & Pathing
- **WORKSPACE ROOT:** The Rust workspace and `Cargo.toml` are located in the `src-tauri/` directory.
- **EXECUTION:** All `cargo` commands MUST be run from within `src-tauri/` or via `pnpm` scripts from the root.
- **MCP TOOLS:** You have access to MCP tools. Use them to read files, search the codebase, and execute terminal commands efficiently.

## Mandatory Workflow (The Execution Loop)
Before a task is considered "Done", you MUST execute this sequence:
1. **PLAN:** Detail the logic in chat. Mention if you'll use `sqlx` or specific crates.
2. **EXECUTE:** Implement logic in `src-tauri/src`.
3. **DOCUMENT:** Add `///` docstrings to public APIs.
4. **FORMAT:** Run `pnpm rust:format` (or `cd src-tauri && cargo fmt`).
5. **VERIFY:** You MUST run and pass:
   - `pnpm rust:check` or `cd src-tauri && cargo check`.
   - `pnpm rust:clippy` or `cd src-tauri && cargo clippy` (Treat warnings as errors).
   - `pnpm rust:test` or `cd src-tauri && cargo test` (Ensure all inline tests pass).
If a command fails, use your MCP tools to read the compiler error, fix the code, and re-run the Verification step. 
Do not skip verification.

## General Instructions

- Always prioritize readability, safety, and maintainability.
- Use strong typing and leverage Rust's ownership system for memory safety.
- Break down complex functions into smaller, more manageable functions.
- For algorithm-related code, include explanations of the approach used.
- Write code with good maintainability practices, including comments on why certain design decisions were made.
- Handle errors gracefully using `Result<T, E>` and provide meaningful error messages.
- For external dependencies, mention their usage and purpose in documentation.
- Use consistent naming conventions following [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md).
- Write idiomatic, safe, and efficient Rust code that follows the borrow checker's rules.
- Ensure code compiles without warnings.

## Patterns to Follow
- Use modules (`mod`) and public interfaces (`pub`) to encapsulate logic.
- Handle errors properly using `?`, `match`, or `if let`.
- Use `serde` for serialization and `thiserror` or `anyhow` for custom errors.
- Implement traits to abstract services or external dependencies.
- Structure async code using `async/await` and `tokio`.
- Prefer enums over flags and states for type safety.
- Use builders for complex object creation.
- Use iterators instead of index-based loops as they're often faster and safer.
- Use `&str` instead of `String` for function parameters when you don't need ownership.
- Prefer borrowing and zero-copy operations to avoid unnecessary allocations.

### Ownership, Borrowing, and Lifetimes
- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary.
- Use `&mut T` when you need to modify borrowed data.
- Explicitly annotate lifetimes when the compiler cannot infer them.
- Use `Rc<T>` for single-threaded reference counting and `Arc<T>` for thread-safe reference counting.
- Use `RefCell<T>` for interior mutability in single-threaded contexts and `Mutex<T>` or `RwLock<T>` for multi-threaded contexts.

## Patterns to Avoid
- Don't use `unwrap()` or `expect()` unless absolutely necessary—prefer proper error handling.
- Avoid panics in code — return `Result` instead.
- Don't rely on global mutable state—use dependency injection or thread-safe containers.
- Avoid deeply nested logic—refactor with functions or combinators.
- Avoid `unsafe` unless required and fully documented.
- Don't overuse `clone()`, use borrowing instead of cloning unless ownership transfer is needed.
- Avoid premature `collect()`, keep iterators lazy until you actually need the collection.
- Avoid unnecessary allocations—prefer borrowing and zero-copy operations.

## Code Style and Formatting
- Follow the Rust Style Guide and use `rustfmt` for automatic formatting.
- Keep lines under 100 characters when possible.
- Place function and struct documentation immediately before the item using `///`.
- Use `cargo clippy` to catch common mistakes and enforce best practices.

## Error Handling
- Use `Result<T, E>` for recoverable errors and `panic!` only for unrecoverable errors.
- Prefer `?` operator over `unwrap()` or `expect()` for error propagation.
- Create custom error types using `thiserror` or implement `std::error::Error`.
- Use `Option<T>` for values that may or may not exist.
- Provide meaningful error messages and context.
- Error types should be meaningful and well-behaved (implement standard traits).
- Validate function arguments and return appropriate errors for invalid input.

## API Design Guidelines

### Common Traits Implementation
Eagerly implement common traits where appropriate:
- `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`
- Use standard conversion traits: `From`, `AsRef`, `AsMut`
- Collections should implement `FromIterator` and `Extend`
- Note: `Send` and `Sync` are auto-implemented by the compiler when safe; avoid manual implementation unless using `unsafe` code

### Type Safety and Predictability
- Use newtypes to provide static distinctions
- Arguments should convey meaning through types; prefer specific types over generic `bool` parameters
- Use `Option<T>` appropriately for truly optional values
- Functions with a clear receiver should be methods
- Only smart pointers should implement `Deref` and `DerefMut`

### Future Proofing
- Use sealed traits to protect against downstream implementations
- Structs should have private fields
- Functions should validate their arguments
- All public types must implement `Debug`

## Testing and Documentation
- Write comprehensive unit tests using `#[cfg(test)]` modules and `#[test]` annotations.
- Use test modules alongside the code they test (`mod tests { ... }`).
- Write integration tests in `tests/` directory with descriptive filenames.
- Write clear and concise comments for each function, struct, enum, and complex logic.
- Ensure functions have descriptive names and include comprehensive documentation.
- Document all public APIs with rustdoc (`///` comments) following the [API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `#[doc(hidden)]` to hide implementation details from public documentation.
- Document error conditions, panic scenarios, and safety considerations.
- Examples should use `?` operator, not `unwrap()` or deprecated `try!` macro.

## Quality Checklist
Before publishing or reviewing Rust code, ensure:

### Core Requirements
- [ ] **Naming**: Follows RFC 430 naming conventions
- [ ] **Traits**: Implements `Debug`, `Clone`, `PartialEq` where appropriate
- [ ] **Error Handling**: Uses `Result<T, E>` and provides meaningful error types
- [ ] **Documentation**: All public items have rustdoc comments with examples
- [ ] **Testing**: Comprehensive test coverage including edge cases

### Safety and Quality
- [ ] **Safety**: No unnecessary `unsafe` code, proper error handling
- [ ] **Performance**: Efficient use of iterators, minimal allocations
- [ ] **API Design**: Functions are predictable, flexible, and type-safe
- [ ] **Future Proofing**: Private fields in structs, sealed traits where appropriate
- [ ] **Tooling**: Code passes `cargo fmt`, `cargo clippy`, and `cargo test`
