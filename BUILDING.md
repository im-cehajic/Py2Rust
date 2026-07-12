# Building and Testing Py2Rust

This document covers how to build, test, and run Py2Rust.

## Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Python 3.7+ (for validation, optional)

### Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_fibonacci -- --nocapture

# Run with detailed output
cargo test -- --nocapture --test-threads=1
```

### Comprehensive Test Suite

Run the complete build and test pipeline:

```bash
chmod +x build.sh
./build.sh
```

This runs:
- ✓ Full build
- ✓ All 30+ tests
- ✓ Documentation generation
- ✓ Code linting (clippy)
- ✓ Format checking
- ✓ Code statistics

## Using the CLI

### Convert a Python file to Rust

```bash
cargo run -- --input example.py --output example.rs
```

### Convert and print to stdout

```bash
cargo run -- --input example.py
```

### Examples

Try the included examples:

```bash
cargo run --example sorting --release
cargo run --example math_functions --release
cargo run --example data_structures --release
```

## Test Categories

### Basic Tests
- `test_simple_function` - Function definition and return
- `test_variable_declaration` - Variable assignment
- `test_if_statement` - If/else statements
- `test_for_loop` - For loops with range
- `test_while_loop` - While loops

### Algorithm Tests
- `test_fibonacci` - Recursive fibonacci
- `test_factorial` - Recursive factorial
- `test_bubble_sort` - Bubble sort implementation

### Data Structure Tests
- `test_list_literal` - List creation and conversion to Vec
- `test_dict_literal` - Dictionary conversion to HashMap
- `test_list_comprehension_simple` - Loop-based list building

### Expression Tests
- `test_arithmetic_operations` - All arithmetic operators
- `test_comparison_operators` - Comparison operators
- `test_boolean_operations` - Boolean logic
- `test_string_concatenation` - String operations

### Complex Tests
- `test_multiple_return_paths` - Different return branches
- `test_nested_loops` - Nested for loops
- `test_nested_if_statements` - Nested conditionals
- `test_multiple_functions` - Multiple function definitions

## Project Structure

```
Py2Rust/
├── src/
│   ├── lib.rs              # Main library
│   ├── main.rs             # CLI entry point
│   ├── cli.rs              # Command-line interface
│   ├── error.rs            # Error types
│   ├── parser/             # Python parsing
│   │   ├── mod.rs
│   │   ├── ast_visitor.rs
│   │   └── error.rs
│   ├── ir/                 # Intermediate representation
│   │   ├── mod.rs
│   │   └── class.rs
│   ├── generator/          # Rust code generation
│   │   ├── mod.rs
│   │   ├── expr_gen.rs
│   │   ├── stmt_gen.rs
│   │   ├── type_map.rs
│   │   ├── builtins.rs
│   │   └── string_methods.rs
│   └── transforms/         # IR transformations
│       ├── mod.rs
│       ├── type_inference.rs
│       ├── ownership.rs
│       └── optimizer.rs
├── tests/
│   ├── integration_test.rs
│   └── pipeline_test.rs
├── examples/
│   ├── sorting.rs
│   ├── math_functions.rs
│   └── data_structures.rs
├── Cargo.toml
├── build.sh
└── README.md
```

## Performance

The release build includes optimizations:

```bash
cargo build --release
```

Binary location: `target/release/py2rust`

## Troubleshooting

### Build Issues

**Error: `rustpython_parser not found`**
- Run `cargo update` to fetch dependencies

**Error: `MSVC toolchain not found` (Windows)**
- Install Visual Studio Build Tools
- Or use: `rustup default stable-msvc`

### Test Issues

**Some tests fail**
- Run with `--nocapture` to see output: `cargo test -- --nocapture`
- Check CHANGELOG.md for known limitations

### Runtime Issues

**Conversion produces invalid Rust**
- Python's dynamic typing can cause issues
- Add type hints in Python code when possible
- Check the generated code and fix manually if needed

## Development

### Adding Tests

Edit `tests/integration_test.rs` and add:

```rust
#[test]
fn test_my_feature() {
    let python_code = r#"
    // Python code here
    "#;
    
    let result = convert(python_code);
    assert!(result.is_ok());
    // More assertions...
}
```

Then run: `cargo test test_my_feature -- --nocapture`

### Documentation

Generate and open documentation:

```bash
cargo doc --open
```

## CI/CD

GitHub Actions workflow (`.github/workflows/test.yml`):

```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --release
```
