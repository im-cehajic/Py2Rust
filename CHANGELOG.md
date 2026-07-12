# Py2Rust Changelog

## [Unreleased]

### Added
- ✅ Python to Rust converter with AST-based parsing
- ✅ Intermediate Representation (IR) system
- ✅ Type inference engine
- ✅ Function conversion with parameters and return types
- ✅ Control flow: if/else, for, while loops
- ✅ Data structures: lists (Vec), dictionaries (HashMap)
- ✅ Expression support: binary ops, unary ops, comparisons
- ✅ String literals and concatenation
- ✅ Comprehensive test suite
- 🔄 Built-in function mapping (partial)
- 🔄 String method conversion (partial)

### In Progress
- 📋 Class/struct conversion
- 📋 Decorator support
- 📋 List comprehensions
- 📋 Lambda expressions
- 📋 Generator functions
- 📋 Context managers (with statement)
- 📋 Exception handling
- 📋 Module imports

### Known Limitations
- Python's dynamic typing requires heuristic type inference
- Complex pattern matching in assignments not supported
- Decorators not yet implemented
- Generators and async/await not supported
- Some built-in functions have limited support

## [0.1.0] - Initial Release

### Features
- Basic Python AST parsing
- IR conversion pipeline
- Rust code generation
- Type mapping layer
- CLI interface
