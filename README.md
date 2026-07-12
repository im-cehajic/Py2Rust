# Py2Rust

A Python to Rust code converter that translates Python code into idiomatic Rust.

## Overview

Py2Rust is inspired by projects like Cpp2Rust and uses a three-stage architecture:

1. **Python Parser** - Parse Python code into an Abstract Syntax Tree (AST)
2. **Intermediate Representation (IR)** - Language-agnostic internal representation
3. **Rust Code Generator** - Emit safe, idiomatic Rust code

## Features

- ✅ Function conversion
- ✅ Class to struct/impl conversion
- ✅ Type inference and mapping
- ✅ Variable and ownership analysis
- ✅ Basic control flow (if/else, loops)
- ✅ Collection conversions (list → Vec, dict → HashMap)
- ✅ Error handling
- 🚧 Advanced features (decorators, generators, async/await)

## Installation

```bash
git clone https://github.com/im-cehajic/Py2Rust.git
cd Py2Rust
cargo build --release
```

## Usage

```bash
cargo run -- --input example.py --output example.rs
```

## Example

### Python Input
```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

result = fibonacci(10)
```

### Rust Output
```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let result = fibonacci(10);
}
```

## Architecture

```
Python Source
     |
     v
┌─────────────────┐
│  Python Parser  │  (using rustpython_parser)
└────────┬────────┘
         |
         v
┌─────────────────┐
│       IR        │  (Intermediate Representation)
└────────┬────────┘
         |
         v
┌─────────────────┐
│  Rust Generator │  (Type mapping, ownership analysis)
└────────┬────────┘
         |
         v
   Rust Source
```

## Project Structure

```
Py2Rust/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── parser/              # Python parsing
│   ├── ir/                  # Intermediate representation
│   ├── generator/           # Rust code generation
│   ├── transforms/          # IR transformations
│   ├── error.rs             # Error types
│   └── cli.rs               # Command-line interface
├── tests/                   # Integration tests
├── examples/                # Example Python files
└── Cargo.toml
```

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- --input example.py

# Build documentation
cargo doc --open
```

## Limitations

- Python is dynamically typed; type inference is heuristic-based
- Not all Python idioms have direct Rust equivalents
- Complex metaprogramming and decorators require manual intervention
- Generators and async/await are not yet supported

## Contributing

Contributions are welcome! Please open an issue or pull request.

## License

Apache-2.0
