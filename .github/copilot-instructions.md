# AI Agent Instructions for hello-rust

## Project Overview

**hello-rust** is an educational Rust learning codebase designed to teach Rust fundamentals through practical examples. It's a single-file binary project with no external dependencies.

- **Type**: Learning/Tutorial project
- **Structure**: Monolithic `main.rs` covering Rust concepts sequentially
- **Edition**: Rust 2024
- **Main File**: [src/main.rs](../src/main.rs)

## Build & Execution

```bash
# Build the project
cargo build

# Build and run in one command
cargo run

# Run in release mode (optimized)
cargo run --release

# Build only (no execution)
cargo build --release
```

## Project Structure & Key Patterns

### Single Main Function Organization
All code resides in `main()` with extensive inline comments separating logical blocks:
- Each section starts with `// ============================================`
- Sections are independent demonstrations of Rust concepts
- No helper functions except `greet()` and `get_user()` at the end

### Covered Rust Concepts (in order)
1. **Variables & Mutability** (`mut` keyword, reassignment)
2. **Data Types** (integers, floats, chars, bools, &str, String)
3. **Constants** (SCREAMING_SNAKE_CASE, immutable)
4. **Arithmetic Operators** (+, -, *, /, %)
5. **Compound Assignment** (+=, -=, *=, /=, %=)
6. **Comparison & Logical Operators** (==, !=, <, >, <=, >=, &&, ||, !)
7. **Control Flow** (if/else, if expressions, match)
8. **Loops** (loop, while, for with ranges)
9. **Functions** (parameters, return types)
10. **Variable Shadowing** (scope-based redeclaration)
11. **String Operations** (to_string(), String::from(), push_str, format!)
12. **Collections** (HashMap, Vec, tuples)
13. **Structs** (named fields, mutability)
14. **Enums** (variants with associated data, pattern matching)

## Common Development Tasks

### Adding New Learning Sections
When adding content, maintain the established format:
- Add a new section with the standard comment block separator
- Include clear explanatory comments before code
- Keep examples concise and focused on one concept
- Insert before the final function definitions

### Modifying Examples
- Preserve comment blocks that explain *why* patterns work
- Keep output examples in `println!` statements for learners to see
- If changing behavior, update inline comments to reflect changes

### Running Specific Sections
There's no module system here—comments delineate sections. To test a specific concept:
1. Comment out other sections or specific `println!` calls
2. Run `cargo run` to execute and observe output
3. Use `cargo check` to verify syntax without running

## Key Developer Conventions

- **Ownership Model**: Examples demonstrate Rust's ownership and borrowing rules (see String ownership section around line 320)
- **Pattern Matching**: `match` expressions are exhaustive—all branches must be covered or use `_` catch-all
- **Type Annotations**: Explicit types shown in variable declarations for clarity (e.g., `let my_num: i8 = 5`)
- **String Types**: Distinction between `&str` (immutable slice) and `String` (heap-allocated) is critical throughout

## Debugging & Validation

### Check Syntax Without Running
```bash
cargo check
```

### View Detailed Error Messages
```bash
cargo build 2>&1 | less
```

### Format Code (if needed)
```bash
cargo fmt
```

## Important Notes for AI Agents

- **Linear Learning Flow**: This is designed for sequential learning; sections build on previous concepts
- **No External Dependencies**: `HashMap` and `Vec` are from `std::collections` (already imported)
- **Output-Focused**: Most code examples end with `println!` to show expected behavior—this is intentional for learning
- **Single Executable**: No tests, libraries, or multi-binary structure to consider
